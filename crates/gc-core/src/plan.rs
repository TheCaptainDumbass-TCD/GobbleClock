use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{f_to_c, format_duration, format_time};

/// Heaviest turkey the method's author recommends for the 250°F slow cook.
pub const MAX_RECOMMENDED_LB: f64 = 20.0;
pub const MAX_WEIGHT_LB: f64 = 40.0;
pub const BREAST_DONE_F: u32 = 170;
pub const THIGH_DONE_F: u32 = 180;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CoverMode {
    /// Cover 15 minutes after turning the oven down, uncover for the last 20 minutes.
    #[default]
    CoverThenBrown,
    /// Cover 15 minutes after turning the oven down and leave it covered.
    Covered,
    /// Never covered. Darker, crisper skin.
    Uncovered,
}

impl CoverMode {
    pub const ALL: [CoverMode; 3] = [CoverMode::CoverThenBrown, CoverMode::Covered, CoverMode::Uncovered];

    pub fn key(self) -> &'static str {
        match self {
            CoverMode::CoverThenBrown => "cover_then_brown",
            CoverMode::Covered => "covered",
            CoverMode::Uncovered => "uncovered",
        }
    }

    pub fn from_key(key: &str) -> CoverMode {
        CoverMode::ALL.into_iter().find(|m| m.key() == key).unwrap_or_default()
    }

    pub fn label(self) -> &'static str {
        match self {
            CoverMode::CoverThenBrown => "Cover, then uncover to brown",
            CoverMode::Covered => "Covered the whole time",
            CoverMode::Uncovered => "Uncovered the whole time",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TempUnit {
    #[default]
    F,
    C,
}

impl TempUnit {
    pub fn show(self, f: u32) -> String {
        match self {
            TempUnit::F => format!("{f}°F"),
            TempUnit::C => format!("{}°C", f_to_c(f)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WeightUnit {
    #[default]
    Lb,
    Kg,
}

/// Method timings and app preferences. Defaults follow the website's method
/// and the routine from the original spreadsheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub warmup_minutes: u32,
    pub prep_minutes: u32,
    pub sear_minutes: u32,
    pub sear_temp_f: u32,
    pub low_temp_f: u32,
    pub minutes_per_pound: u32,
    pub cover_after_minutes: u32,
    pub brown_minutes: u32,
    pub rest_minutes: u32,
    pub buffer_enabled: bool,
    pub buffer_minutes: u32,
    pub cover_mode: CoverMode,
    pub temp_unit: TempUnit,
    pub weight_unit: WeightUnit,
    pub theme: String,
    pub sound: bool,
    pub default_carve_time: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            warmup_minutes: 60,
            prep_minutes: 20,
            sear_minutes: 20,
            sear_temp_f: 475,
            low_temp_f: 250,
            minutes_per_pound: 20,
            cover_after_minutes: 15,
            brown_minutes: 20,
            rest_minutes: 20,
            buffer_enabled: false,
            buffer_minutes: 60,
            cover_mode: CoverMode::CoverThenBrown,
            temp_unit: TempUnit::F,
            weight_unit: WeightUnit::Lb,
            theme: "system".into(),
            sound: true,
            default_carve_time: "16:20".into(),
        }
    }
}

impl Settings {
    /// Keeps the app preferences and puts every method timing back to the default.
    pub fn with_default_method(&self) -> Settings {
        Settings {
            cover_mode: self.cover_mode,
            temp_unit: self.temp_unit,
            weight_unit: self.weight_unit,
            theme: self.theme.clone(),
            sound: self.sound,
            default_carve_time: self.default_carve_time.clone(),
            buffer_enabled: self.buffer_enabled,
            ..Settings::default()
        }
    }
}

/// What the cook tells the app: turkey weight in pounds and when to carve.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanInput {
    pub weight_lb: f64,
    pub carve_at: NaiveDateTime,
    #[serde(default)]
    pub fresh: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepKind {
    Warmup,
    Preheat,
    Prep,
    Sear,
    ReduceHeat,
    Cover,
    FreshCheck,
    Uncover,
    Remove,
    Rest,
    Carve,
}

/// Color group for a step: oven work, hands on work, checks and the finish.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tone {
    Oven,
    Hands,
    Check,
    Done,
}

impl Tone {
    pub fn key(self) -> &'static str {
        match self {
            Tone::Oven => "oven",
            Tone::Hands => "hands",
            Tone::Check => "check",
            Tone::Done => "done",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub kind: StepKind,
    pub at: NaiveDateTime,
    pub title: String,
    pub detail: String,
    pub minutes: Option<u32>,
    pub oven_f: Option<u32>,
    pub tone: Tone,
}

/// The end of one pound's worth of 250°F cooking, like the pound column on the sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoundMark {
    pub pound: u32,
    pub starts: NaiveDateTime,
    pub ends: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThawPlan {
    pub days_min: u32,
    pub days_max: u32,
    /// Move the turkey to the refrigerator on this day at the latest.
    pub latest: NaiveDate,
    pub earliest: NaiveDate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Info,
    Caution,
    Danger,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notice {
    pub level: Level,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub input: PlanInput,
    pub cook_pounds: u32,
    pub low_minutes: u32,
    /// From taking the turkey out of the refrigerator to carving.
    pub total_minutes: u32,
    pub steps: Vec<Step>,
    pub pounds: Vec<PoundMark>,
    pub thaw: Option<ThawPlan>,
    pub notices: Vec<Notice>,
}

impl Schedule {
    pub fn starts_at(&self) -> NaiveDateTime {
        self.steps.first().map(|s| s.at).unwrap_or(self.input.carve_at)
    }

    pub fn step(&self, kind: StepKind) -> Option<&Step> {
        self.steps.iter().find(|s| s.kind == kind)
    }

    /// Index of the step running at `now`: the last one whose time has come.
    pub fn current_index(&self, now: NaiveDateTime) -> Option<usize> {
        self.steps.iter().rposition(|s| s.at <= now)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlanError {
    Weight,
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::Weight => write!(f, "Enter a turkey weight between 1 and {MAX_WEIGHT_LB:.0} pounds."),
        }
    }
}

impl std::error::Error for PlanError {}

/// Pounds used for timing: under half a pound rounds down, half a pound or more rounds up.
pub fn cook_pounds(weight_lb: f64) -> u32 {
    // Hundredths first, so 12.4999999 typed as 12.5 still rounds up.
    let hundredths = (weight_lb * 100.0).round() as i64;
    (((hundredths + 50) / 100).max(1)) as u32
}

/// Days to thaw in the refrigerator, from the website's table. Above 24 pounds it
/// continues at about one day per 4 pounds.
pub fn thaw_days(weight_lb: f64) -> (u32, u32) {
    match weight_lb {
        w if w <= 12.0 => (2, 3),
        w if w <= 16.0 => (3, 4),
        w if w <= 20.0 => (4, 5),
        w if w <= 24.0 => (5, 6),
        w => {
            let max = (w / 4.0).ceil() as u32;
            (max - 1, max)
        }
    }
}

fn minutes(m: u32) -> Duration {
    Duration::minutes(m as i64)
}

pub fn build(input: &PlanInput, s: &Settings) -> Result<Schedule, PlanError> {
    if !input.weight_lb.is_finite() || input.weight_lb < 1.0 || input.weight_lb > MAX_WEIGHT_LB {
        return Err(PlanError::Weight);
    }
    let t = s.temp_unit;
    let pounds = cook_pounds(input.weight_lb);
    let low_minutes = pounds * s.minutes_per_pound;
    let buffer = if s.buffer_enabled { s.buffer_minutes } else { 0 };

    let carve = input.carve_at;
    let ready = carve - minutes(buffer);
    let remove = ready - minutes(s.rest_minutes);
    let low_start = remove - minutes(low_minutes);
    let sear_start = low_start - minutes(s.sear_minutes);
    let prep_start = sear_start - minutes(s.prep_minutes);
    let warmup_start = prep_start - minutes(s.warmup_minutes);

    let mut steps = vec![
        Step {
            kind: StepKind::Warmup,
            at: warmup_start,
            title: "Take the turkey out of the refrigerator".into(),
            detail: "Let it warm toward room temperature. A cold turkey is the top reason it is not done on time. Remove the neck and giblets.".into(),
            minutes: Some(s.warmup_minutes),
            oven_f: None,
            tone: Tone::Hands,
        },
        Step {
            kind: StepKind::Preheat,
            at: prep_start,
            title: format!("Set the rack and preheat the oven to {}", t.show(s.sear_temp_f)),
            detail: "Middle rack for most birds, one level lower for anything over 20 pounds. Check that the pan fits, with the lid if you cover it.".into(),
            minutes: None,
            oven_f: Some(s.sear_temp_f),
            tone: Tone::Oven,
        },
        Step {
            kind: StepKind::Prep,
            at: prep_start,
            title: "Prep the turkey".into(),
            detail: "Rub melted butter (rosemary and thyme optional) under the breast skin, not on top. Lightly salt and pepper the outside. No stuffing inside.".into(),
            minutes: Some(s.prep_minutes),
            oven_f: None,
            tone: Tone::Hands,
        },
        Step {
            kind: StepKind::Sear,
            at: sear_start,
            title: format!("Turkey in the oven, uncovered, at {}", t.show(s.sear_temp_f)),
            detail: "Close the door. From here on, do not open it until the time is up.".into(),
            minutes: Some(s.sear_minutes),
            oven_f: Some(s.sear_temp_f),
            tone: Tone::Oven,
        },
        Step {
            kind: StepKind::ReduceHeat,
            at: low_start,
            title: format!("Turn the oven down to {}", t.show(s.low_temp_f)),
            detail: format!(
                "Leave the turkey in and the door closed. The {} at {} starts now, not when the oven cools.",
                format_duration(low_minutes as i64),
                t.show(s.low_temp_f)
            ),
            minutes: Some(low_minutes),
            oven_f: Some(s.low_temp_f),
            tone: Tone::Oven,
        },
    ];

    let cover_at = low_start + minutes(s.cover_after_minutes);
    let covers = s.cover_mode != CoverMode::Uncovered && s.cover_after_minutes < low_minutes;
    if covers {
        steps.push(Step {
            kind: StepKind::Cover,
            at: cover_at,
            title: "Put the lid on the roasting pan".into(),
            detail: "Be quick so the oven keeps its heat. Foil works too.".into(),
            minutes: None,
            oven_f: Some(s.low_temp_f),
            tone: Tone::Hands,
        });
    }

    if input.fresh {
        let at = low_start + minutes(low_minutes * 2 / 3);
        steps.push(Step {
            kind: StepKind::FreshCheck,
            at,
            title: "Fresh turkey: check the temperature early".into(),
            detail: format!(
                "Fresh, never frozen turkeys can cook faster. If the breast reads {} and the thigh {}, take it out and keep it warm.",
                t.show(BREAST_DONE_F),
                t.show(THIGH_DONE_F)
            ),
            minutes: None,
            oven_f: Some(s.low_temp_f),
            tone: Tone::Check,
        });
    }

    let uncover_at = remove - minutes(s.brown_minutes);
    if covers && s.cover_mode == CoverMode::CoverThenBrown && uncover_at > cover_at {
        steps.push(Step {
            kind: StepKind::Uncover,
            at: uncover_at,
            title: "Take the lid off to brown the skin".into(),
            detail: format!("The last {} uncovered crisps the skin.", format_duration(s.brown_minutes as i64)),
            minutes: Some(s.brown_minutes),
            oven_f: Some(s.low_temp_f),
            tone: Tone::Hands,
        });
    }

    steps.push(Step {
        kind: StepKind::Remove,
        at: remove,
        title: "Take the turkey out and check the temperature".into(),
        detail: format!(
            "Use a meat thermometer: {} in the breast and {} in the thigh. If it is not there yet, put it back in and check again in 20 minutes.",
            t.show(BREAST_DONE_F),
            t.show(THIGH_DONE_F)
        ),
        minutes: None,
        oven_f: None,
        tone: Tone::Check,
    });
    steps.push(Step {
        kind: StepKind::Rest,
        at: remove,
        title: "Let it rest uncovered".into(),
        detail: "Resting lets the juices settle so it stays moist and carves cleanly.".into(),
        minutes: Some(s.rest_minutes),
        oven_f: None,
        tone: Tone::Done,
    });
    steps.push(Step {
        kind: StepKind::Carve,
        at: carve,
        title: "Carve and eat".into(),
        detail: if buffer > 0 {
            format!("The turkey was ready {} early. Keep it tented with foil until now.", format_duration(buffer as i64))
        } else {
            "Happy Thanksgiving!".into()
        },
        minutes: None,
        oven_f: None,
        tone: Tone::Done,
    });
    steps.sort_by_key(|s| s.at);

    let pound_marks = (0..pounds)
        .map(|i| PoundMark {
            pound: i + 1,
            starts: low_start + minutes(i * s.minutes_per_pound),
            ends: low_start + minutes((i + 1) * s.minutes_per_pound),
        })
        .collect();

    let thaw = (!input.fresh).then(|| {
        let (days_min, days_max) = thaw_days(input.weight_lb);
        let day = warmup_start.date();
        ThawPlan {
            days_min,
            days_max,
            latest: day - Duration::days(days_max as i64),
            earliest: day - Duration::days(days_max as i64 + 1),
        }
    });

    let mut notices = Vec::new();
    if input.weight_lb > MAX_RECOMMENDED_LB {
        notices.push(Notice {
            level: Level::Danger,
            text: format!(
                "The website recommends this slow cook method only for turkeys of {MAX_RECOMMENDED_LB:.0} pounds or less. Consider a higher oven temperature for this bird."
            ),
        });
    }
    let rounded = (input.weight_lb - pounds as f64).abs() > 0.001;
    if rounded {
        notices.push(Notice {
            level: Level::Info,
            text: format!("{} lb is timed as {pounds} lb. Under half a pound rounds down, half a pound or more rounds up.", trim(input.weight_lb)),
        });
    }
    if buffer > 0 {
        notices.push(Notice {
            level: Level::Info,
            text: format!("Wiggle room is on: the turkey is planned to be ready {} before you carve.", format_duration(buffer as i64)),
        });
    }
    let early = warmup_start.time() < chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap();
    if early {
        notices.push(Notice {
            level: Level::Caution,
            text: format!("An early start: the turkey comes out of the refrigerator at {}.", format_time(warmup_start)),
        });
    }

    Ok(Schedule {
        input: input.clone(),
        cook_pounds: pounds,
        low_minutes,
        total_minutes: (carve - warmup_start).num_minutes() as u32,
        steps,
        pounds: pound_marks,
        thaw,
        notices,
    })
}

/// "12.14", "15.5", "11".
pub fn trim(n: f64) -> String {
    let s = format!("{n:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(y: i32, mo: u32, d: u32, h: u32, mi: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, mo, d).unwrap().and_hms_opt(h, mi, 0).unwrap()
    }

    fn plan(weight: f64) -> Schedule {
        let input = PlanInput { weight_lb: weight, carve_at: at(2025, 11, 27, 16, 20), fresh: false };
        build(&input, &Settings::default()).unwrap()
    }

    fn time_of(s: &Schedule, kind: StepKind) -> NaiveDateTime {
        s.step(kind).unwrap().at
    }

    #[test]
    fn rounding_rule() {
        assert_eq!(cook_pounds(11.0), 11);
        assert_eq!(cook_pounds(12.14), 12);
        assert_eq!(cook_pounds(13.23), 13);
        assert_eq!(cook_pounds(12.49), 12);
        assert_eq!(cook_pounds(12.5), 13);
        assert_eq!(cook_pounds(15.5), 16);
        assert_eq!(cook_pounds(15.51), 16);
        assert_eq!(cook_pounds(1.2), 1);
    }

    #[test]
    fn matches_2020_sheet() {
        // 11 lb: in at 12:00, down to 250 at 12:20, lid 12:35, uncover 3:40, out 4:00, carve 4:20.
        let s = plan(11.0);
        assert_eq!(time_of(&s, StepKind::Sear), at(2025, 11, 27, 12, 0));
        assert_eq!(time_of(&s, StepKind::ReduceHeat), at(2025, 11, 27, 12, 20));
        assert_eq!(time_of(&s, StepKind::Cover), at(2025, 11, 27, 12, 35));
        assert_eq!(time_of(&s, StepKind::Uncover), at(2025, 11, 27, 15, 40));
        assert_eq!(time_of(&s, StepKind::Remove), at(2025, 11, 27, 16, 0));
        assert_eq!(time_of(&s, StepKind::Carve), at(2025, 11, 27, 16, 20));
        assert_eq!(time_of(&s, StepKind::Preheat), at(2025, 11, 27, 11, 40));
        assert_eq!(time_of(&s, StepKind::Warmup), at(2025, 11, 27, 10, 40));
        assert_eq!(s.pounds.len(), 11);
        assert_eq!(s.pounds[0].starts, at(2025, 11, 27, 12, 20));
        assert_eq!(s.pounds[10].ends, at(2025, 11, 27, 16, 0));
    }

    #[test]
    fn matches_2024_sheet() {
        // 12.14 lb times as 12 lb: down to 250 at 12:00.
        let s = plan(12.14);
        assert_eq!(s.cook_pounds, 12);
        assert_eq!(time_of(&s, StepKind::ReduceHeat), at(2025, 11, 27, 12, 0));
        assert_eq!(time_of(&s, StepKind::Sear), at(2025, 11, 27, 11, 40));
    }

    #[test]
    fn fixes_2021_sheet() {
        // 15.5 lb times as 16 lb: 5 h 20 min at 250, in at 10:20.
        let s = plan(15.5);
        assert_eq!(s.low_minutes, 320);
        assert_eq!(time_of(&s, StepKind::ReduceHeat), at(2025, 11, 27, 10, 40));
        assert_eq!(time_of(&s, StepKind::Sear), at(2025, 11, 27, 10, 20));
    }

    #[test]
    fn buffer_shifts_everything() {
        let input = PlanInput { weight_lb: 13.23, carve_at: at(2025, 11, 27, 16, 20), fresh: false };
        let settings = Settings { buffer_enabled: true, ..Settings::default() };
        let s = build(&input, &settings).unwrap();
        assert_eq!(time_of(&s, StepKind::Remove), at(2025, 11, 27, 15, 0));
        assert_eq!(time_of(&s, StepKind::Carve), at(2025, 11, 27, 16, 20));
    }

    #[test]
    fn cover_modes() {
        let input = PlanInput { weight_lb: 12.0, carve_at: at(2025, 11, 27, 16, 20), fresh: false };
        let uncovered = build(&input, &Settings { cover_mode: CoverMode::Uncovered, ..Settings::default() }).unwrap();
        assert!(uncovered.step(StepKind::Cover).is_none());
        assert!(uncovered.step(StepKind::Uncover).is_none());
        let covered = build(&input, &Settings { cover_mode: CoverMode::Covered, ..Settings::default() }).unwrap();
        assert!(covered.step(StepKind::Cover).is_some());
        assert!(covered.step(StepKind::Uncover).is_none());
    }

    #[test]
    fn fresh_turkey_gets_early_check_and_no_thaw() {
        let input = PlanInput { weight_lb: 12.0, carve_at: at(2025, 11, 27, 16, 20), fresh: true };
        let s = build(&input, &Settings::default()).unwrap();
        // Two thirds of 240 minutes after 12:00.
        assert_eq!(time_of(&s, StepKind::FreshCheck), at(2025, 11, 27, 14, 40));
        assert!(s.thaw.is_none());
    }

    #[test]
    fn thaw_table() {
        assert_eq!(thaw_days(12.0), (2, 3));
        assert_eq!(thaw_days(13.23), (3, 4));
        assert_eq!(thaw_days(16.0), (3, 4));
        assert_eq!(thaw_days(20.0), (4, 5));
        assert_eq!(thaw_days(24.0), (5, 6));
        assert_eq!(thaw_days(8.0), (2, 3));
        assert_eq!(thaw_days(28.0), (6, 7));
        let s = plan(13.23);
        let thaw = s.thaw.unwrap();
        assert_eq!(thaw.latest, NaiveDate::from_ymd_opt(2025, 11, 23).unwrap());
    }

    #[test]
    fn warnings() {
        assert!(plan(21.0).notices.iter().any(|n| n.level == Level::Danger));
        assert!(plan(12.0).notices.is_empty());
        assert!(build(
            &PlanInput { weight_lb: 0.0, carve_at: at(2025, 11, 27, 16, 20), fresh: false },
            &Settings::default()
        )
        .is_err());
    }

    #[test]
    fn steps_are_in_order() {
        let s = plan(18.7);
        assert!(s.steps.windows(2).all(|w| w[0].at <= w[1].at));
        assert_eq!(s.steps.first().unwrap().kind, StepKind::Warmup);
        assert_eq!(s.steps.last().unwrap().kind, StepKind::Carve);
    }
}
