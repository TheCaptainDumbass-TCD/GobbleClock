use std::path::{Path, PathBuf};

use gc_core::{seed_history, HistoryEntry, PlanInput, Settings};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// A cook in progress. Reminders resume from this after a restart.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveCook {
    pub input: PlanInput,
    pub settings: Settings,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Data {
    pub settings: Settings,
    pub history: Vec<HistoryEntry>,
    pub active: Option<ActiveCook>,
}

const SETTINGS: &str = "settings.json";
const HISTORY: &str = "history.json";
const ACTIVE: &str = "active.json";

fn read<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let text = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&text) {
        Ok(v) => Some(v),
        Err(e) => {
            eprintln!("gobbleclock: ignoring {}: {e}", path.display());
            None
        }
    }
}

/// Writes through a temporary file so a crash never leaves half a file.
pub fn write<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text).map_err(|e| format!("Could not save {}: {e}", path.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("Could not save {}: {e}", path.display()))
}

impl Data {
    pub fn load(dir: &Path) -> Data {
        let history = match read::<Vec<HistoryEntry>>(&dir.join(HISTORY)) {
            Some(h) => h,
            None => {
                let seeded = seed_history();
                let _ = write(&dir.join(HISTORY), &seeded);
                seeded
            }
        };
        Data {
            settings: read(&dir.join(SETTINGS)).unwrap_or_default(),
            history,
            active: read(&dir.join(ACTIVE)),
        }
    }
}

pub fn settings_path(dir: &Path) -> PathBuf {
    dir.join(SETTINGS)
}

pub fn history_path(dir: &Path) -> PathBuf {
    dir.join(HISTORY)
}

pub fn active_path(dir: &Path) -> PathBuf {
    dir.join(ACTIVE)
}
