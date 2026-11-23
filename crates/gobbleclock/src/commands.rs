use std::path::PathBuf;
use std::sync::Mutex;

use chrono::Local;
use gc_core::{build, to_ics, to_text, HistoryEntry, PlanInput, Settings};
use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::store::{self, ActiveCook, Data};

pub struct AppState {
    pub dir: PathBuf,
    pub data: Mutex<Data>,
}

#[derive(Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub commit: String,
    pub identifier: String,
    pub data_dir: String,
}

#[tauri::command]
pub fn app_info(app: AppHandle, state: State<'_, AppState>) -> AppInfo {
    AppInfo {
        name: "Gobble Clock".into(),
        version: app.package_info().version.to_string(),
        commit: env!("GC_BUILD_COMMIT").into(),
        identifier: app.config().identifier.clone(),
        data_dir: state.dir.display().to_string(),
    }
}

#[tauri::command]
pub fn load_state(state: State<'_, AppState>) -> Result<Data, String> {
    Ok(state.data.lock().map_err(|e| e.to_string())?.clone())
}

#[tauri::command]
pub fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    store::write(&store::settings_path(&state.dir), &settings)?;
    state.data.lock().map_err(|e| e.to_string())?.settings = settings;
    Ok(())
}

#[tauri::command]
pub fn save_history(state: State<'_, AppState>, history: Vec<HistoryEntry>) -> Result<(), String> {
    store::write(&store::history_path(&state.dir), &history)?;
    state.data.lock().map_err(|e| e.to_string())?.history = history;
    Ok(())
}

#[tauri::command]
pub fn start_cook(state: State<'_, AppState>, input: PlanInput) -> Result<(), String> {
    let mut data = state.data.lock().map_err(|e| e.to_string())?;
    build(&input, &data.settings).map_err(|e| e.to_string())?;
    let cook = ActiveCook { input, settings: data.settings.clone() };
    store::write(&store::active_path(&state.dir), &cook)?;
    data.active = Some(cook);
    Ok(())
}

#[tauri::command]
pub fn stop_cook(state: State<'_, AppState>) -> Result<(), String> {
    let path = store::active_path(&state.dir);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("Could not stop the cook: {e}"))?;
    }
    state.data.lock().map_err(|e| e.to_string())?.active = None;
    Ok(())
}

#[tauri::command]
pub fn test_reminder(app: AppHandle, state: State<'_, AppState>) {
    let sound = state.data.lock().map(|d| d.settings.sound).unwrap_or(true);
    crate::reminders::notify(
        &app,
        "Gobble Clock reminders work",
        "You will get a reminder like this at every step while the turkey cooks.",
        sound,
    );
}

/// Asks where to save, then writes the schedule as a calendar file or a text checklist.
#[tauri::command]
pub async fn export_schedule(
    app: AppHandle,
    state: State<'_, AppState>,
    input: PlanInput,
    kind: String,
) -> Result<Option<String>, String> {
    let settings = state.data.lock().map_err(|e| e.to_string())?.settings.clone();
    let schedule = build(&input, &settings).map_err(|e| e.to_string())?;
    let year = input.carve_at.format("%Y");
    let (contents, name, filter, ext) = match kind.as_str() {
        "ics" => (to_ics(&schedule, Local::now().naive_local()), format!("turkey-{year}.ics"), "Calendar", "ics"),
        _ => (to_text(&schedule, &settings), format!("turkey-{year}.txt"), "Text", "txt"),
    };
    let picked = app
        .dialog()
        .file()
        .set_file_name(&name)
        .add_filter(filter, &[ext])
        .blocking_save_file();
    let Some(picked) = picked else { return Ok(None) };
    let path = picked.into_path().map_err(|e| e.to_string())?;
    std::fs::write(&path, contents).map_err(|e| format!("Could not save {}: {e}", path.display()))?;
    Ok(Some(path.display().to_string()))
}

#[tauri::command]
pub fn print_page(app: AppHandle) -> Result<(), String> {
    let window = app.get_webview_window(crate::MAIN_WINDOW).ok_or("The main window is missing")?;
    window.print().map_err(|e| e.to_string())
}

/// Opens a page of the recipe website in the default browser. Other addresses are refused.
#[tauri::command]
pub fn open_link(url: String) -> Result<(), String> {
    if !url.starts_with("https://howtocookathanksgivingturkey.com/") {
        return Err("Only the recipe website can be opened".into());
    }
    std::process::Command::new("xdg-open")
        .arg(&url)
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}
