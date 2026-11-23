use std::time::Duration;

use chrono::{Local, NaiveDateTime};
use gc_core::{build, format_time, Step};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::commands::AppState;

/// Steps that come due later than this after their time are skipped silently,
/// for example after the laptop wakes from sleep hours later.
const STALE_MINUTES: i64 = 10;

/// The system sound played with a reminder. Linux uses a freedesktop sound name, Windows a toast sound.
#[cfg(windows)]
const SOUND: &str = "Reminder";
#[cfg(not(windows))]
const SOUND: &str = "complete";

#[derive(Clone, serde::Serialize)]
pub struct StepDue {
    pub title: String,
    pub body: String,
}

pub fn notify(app: &AppHandle, title: &str, body: &str, sound: bool) {
    let mut builder = app.notification().builder().title(title).body(body);
    if sound {
        builder = builder.sound(SOUND);
    }
    if let Err(e) = builder.show() {
        eprintln!("gobbleclock: notification failed: {e}");
    }
    let _ = app.emit("step-due", StepDue { title: title.into(), body: body.into() });
}

fn group_due(steps: &[Step], after: NaiveDateTime, now: NaiveDateTime) -> Vec<Vec<&Step>> {
    let mut groups: Vec<Vec<&Step>> = Vec::new();
    for step in steps.iter().filter(|s| s.at > after && s.at <= now) {
        if (now - step.at).num_minutes() >= STALE_MINUTES {
            continue;
        }
        match groups.last_mut() {
            Some(g) if g[0].at == step.at => g.push(step),
            _ => groups.push(vec![step]),
        }
    }
    groups
}

/// Checks every few seconds which steps came due and sends one notification per time slot.
pub fn start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last = Local::now().naive_local();
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let now = Local::now().naive_local();
            let active = {
                let state = app.state::<AppState>();
                let data = state.data.lock();
                data.map(|d| d.active.clone()).unwrap_or(None)
            };
            if let Some(cook) = active {
                if let Ok(schedule) = build(&cook.input, &cook.settings) {
                    for group in group_due(&schedule.steps, last, now) {
                        let title = group.iter().map(|s| s.title.as_str()).collect::<Vec<_>>().join(" and ");
                        let body = format!("{}: {}", format_time(group[0].at), group[0].detail);
                        notify(&app, &title, &body, cook.settings.sound);
                    }
                }
            }
            last = now;
        }
    });
}
