use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::{thanksgiving, CoverMode};

/// One year's turkey, kept so next year starts from what worked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: u64,
    pub date: NaiveDate,
    pub weight_lb: f64,
    pub carve_time: NaiveTime,
    #[serde(default)]
    pub fresh: bool,
    #[serde(default)]
    pub breast: bool,
    #[serde(default)]
    pub cover_mode: CoverMode,
    /// 0 means not rated, otherwise 1 to 5.
    #[serde(default)]
    pub rating: u8,
    #[serde(default)]
    pub notes: String,
}

/// The turkeys from the original Turkey Schedule spreadsheet.
pub fn seed_history() -> Vec<HistoryEntry> {
    let carve = NaiveTime::from_hms_opt(16, 20, 0).unwrap();
    [(2020, 11.0), (2021, 15.5), (2024, 12.14), (2025, 13.23)]
        .into_iter()
        .enumerate()
        .map(|(i, (year, weight))| HistoryEntry {
            id: i as u64 + 1,
            date: thanksgiving(year),
            weight_lb: weight,
            carve_time: carve,
            fresh: false,
            breast: false,
            cover_mode: CoverMode::CoverThenBrown,
            rating: 5,
            notes: "From the Turkey Schedule spreadsheet. Out of the oven at 4:00 PM and carved at 4:20 PM.".into(),
        })
        .collect()
}
