//! Schedule engine for the slow cook turkey method: 20 minutes at 475°F, then
//! 20 minutes per pound at 250°F, timed backward from the moment you carve.

mod calendar;
mod export;
mod history;
mod plan;

pub use calendar::{next_thanksgiving, thanksgiving};
pub use export::{to_ics, to_text};
pub use history::{seed_history, HistoryEntry};
pub use plan::*;

/// Converts Fahrenheit to whole degrees Celsius.
pub fn f_to_c(f: u32) -> i32 {
    ((f as f64 - 32.0) * 5.0 / 9.0).round() as i32
}

pub const KG_PER_LB: f64 = 0.453_592_37;

pub fn kg_to_lb(kg: f64) -> f64 {
    kg / KG_PER_LB
}

pub fn lb_to_kg(lb: f64) -> f64 {
    lb * KG_PER_LB
}

/// "4 h 40 min", "45 min" or "3 h".
pub fn format_duration(minutes: i64) -> String {
    let (h, m) = (minutes / 60, minutes % 60);
    match (h, m) {
        (0, m) => format!("{m} min"),
        (h, 0) => format!("{h} h"),
        (h, m) => format!("{h} h {m} min"),
    }
}

/// "4:20 PM".
pub fn format_time(t: chrono::NaiveDateTime) -> String {
    t.format("%-I:%M %p").to_string()
}

/// "Thursday, November 26".
pub fn format_day(d: chrono::NaiveDate) -> String {
    d.format("%A, %B %-d").to_string()
}
