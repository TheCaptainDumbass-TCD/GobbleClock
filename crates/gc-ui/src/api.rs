use gc_core::{HistoryEntry, PlanInput, Settings};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::tauri::{args, invoke};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveCook {
    pub input: PlanInput,
    pub settings: Settings,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Data {
    pub settings: Settings,
    pub history: Vec<HistoryEntry>,
    pub active: Option<ActiveCook>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub commit: String,
    pub identifier: String,
    pub data_dir: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StepDue {
    pub title: String,
    pub body: String,
}

pub async fn app_info() -> Result<AppInfo, String> {
    invoke("app_info", JsValue::UNDEFINED).await
}

pub async fn load_state() -> Result<Data, String> {
    invoke("load_state", JsValue::UNDEFINED).await
}

pub async fn save_settings(settings: Settings) -> Result<(), String> {
    #[derive(Serialize)]
    struct A {
        settings: Settings,
    }
    invoke("save_settings", args(&A { settings })).await
}

pub async fn save_history(history: Vec<HistoryEntry>) -> Result<(), String> {
    #[derive(Serialize)]
    struct A {
        history: Vec<HistoryEntry>,
    }
    invoke("save_history", args(&A { history })).await
}

pub async fn start_cook(input: PlanInput) -> Result<(), String> {
    #[derive(Serialize)]
    struct A {
        input: PlanInput,
    }
    invoke("start_cook", args(&A { input })).await
}

pub async fn stop_cook() -> Result<(), String> {
    invoke("stop_cook", JsValue::UNDEFINED).await
}

pub async fn test_reminder() -> Result<(), String> {
    invoke("test_reminder", JsValue::UNDEFINED).await
}

pub async fn export_schedule(input: PlanInput, kind: &str) -> Result<Option<String>, String> {
    #[derive(Serialize)]
    struct A<'a> {
        input: PlanInput,
        kind: &'a str,
    }
    invoke("export_schedule", args(&A { input, kind })).await
}

pub async fn print_page() -> Result<(), String> {
    invoke("print_page", JsValue::UNDEFINED).await
}

pub async fn quit_app() -> Result<(), String> {
    invoke("quit_app", JsValue::UNDEFINED).await
}
