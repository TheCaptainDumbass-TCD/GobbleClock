#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod reminders;
mod store;

use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, WindowEvent};

pub const MAIN_WINDOW: &str = "main";

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let data = store::Data::load(&dir);
            app.manage(commands::AppState { dir, data: Mutex::new(data) });
            reminders::start(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<commands::AppState>();
                let cooking = state.data.lock().map(|d| d.active.is_some()).unwrap_or(false);
                if window.label() == MAIN_WINDOW && cooking {
                    api.prevent_close();
                    let _ = window.emit("close-requested", ());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::load_state,
            commands::save_settings,
            commands::save_history,
            commands::start_cook,
            commands::stop_cook,
            commands::test_reminder,
            commands::export_schedule,
            commands::print_page,
            commands::quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Gobble Clock");
}
