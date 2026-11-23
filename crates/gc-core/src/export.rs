use chrono::{Duration, NaiveDateTime};

use crate::{format_day, format_duration, format_time, plan::thermometer_note, plan::trim, Schedule, Settings};

fn ics_time(t: NaiveDateTime) -> String {
    t.format("%Y%m%dT%H%M%S").to_string()
}

fn ics_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace(';', "\\;").replace(',', "\\,").replace('\n', "\\n")
}

/// Calendar file with one event per step and a reminder at the start of each.
/// Times are floating local times, so they follow the computer's time zone.
pub fn to_ics(schedule: &Schedule, stamp: NaiveDateTime) -> String {
    let mut out = String::from(
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Dennis Ayotte//Gobble Clock//EN\r\nCALSCALE:GREGORIAN\r\n",
    );
    for (i, step) in schedule.steps.iter().enumerate() {
        let end = step.at + Duration::minutes(step.minutes.unwrap_or(5).max(5) as i64);
        out.push_str("BEGIN:VEVENT\r\n");
        out.push_str(&format!("UID:gobbleclock-{}-{i}@gobbleclock\r\n", ics_time(schedule.input.carve_at)));
        out.push_str(&format!("DTSTAMP:{}\r\n", ics_time(stamp)));
        out.push_str(&format!("DTSTART:{}\r\n", ics_time(step.at)));
        out.push_str(&format!("DTEND:{}\r\n", ics_time(end)));
        out.push_str(&format!("SUMMARY:{}\r\n", ics_escape(&format!("Turkey: {}", step.title))));
        out.push_str(&format!("DESCRIPTION:{}\r\n", ics_escape(&step.detail)));
        out.push_str("BEGIN:VALARM\r\nACTION:DISPLAY\r\nTRIGGER:PT0M\r\n");
        out.push_str(&format!("DESCRIPTION:{}\r\n", ics_escape(&step.title)));
        out.push_str("END:VALARM\r\nEND:VEVENT\r\n");
    }
    out.push_str("END:VCALENDAR\r\n");
    out
}

/// Plain text checklist.
pub fn to_text(schedule: &Schedule, settings: &Settings) -> String {
    let t = settings.temp_unit;
    let carve = schedule.input.carve_at;
    let mut out = format!(
        "GOBBLE CLOCK: TURKEY SCHEDULE\n{}\n\n{}: {} lb, timed as {} lb\nCarve at: {}\nOven time: {}\n",
        format_day(carve.date()),
        if schedule.input.breast { "Turkey breast" } else { "Turkey" },
        trim(schedule.input.weight_lb),
        schedule.cook_pounds,
        format_time(carve),
        format_duration((schedule.low_minutes + settings.sear_minutes) as i64),
    );
    if let Some(thaw) = &schedule.thaw {
        out.push_str(&format!(
            "Thaw: move it to the refrigerator by {} ({} to {} days)\n",
            format_day(thaw.latest),
            thaw.days_min,
            thaw.days_max
        ));
    }
    out.push('\n');
    for step in &schedule.steps {
        let oven = step.oven_f.map(|f| format!("  [{}]", t.show(f))).unwrap_or_default();
        out.push_str(&format!("[ ] {:>8}  {}{}\n", format_time(step.at), step.title, oven));
    }
    out.push_str(&format!(
        "\n{}\n\nKeep the oven door closed from the moment the turkey goes in until it comes out.\n",
        thermometer_note(t, schedule.input.breast)
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build, PlanInput};
    use chrono::NaiveDate;

    #[test]
    fn exports_every_step() {
        let carve = NaiveDate::from_ymd_opt(2026, 11, 26).unwrap().and_hms_opt(16, 20, 0).unwrap();
        let s = build(&PlanInput { weight_lb: 13.0, carve_at: carve, fresh: false, breast: false }, &Settings::default()).unwrap();
        let ics = to_ics(&s, carve);
        assert_eq!(ics.matches("BEGIN:VEVENT").count(), s.steps.len());
        assert!(ics.contains("DTSTART:20261126T162000"));
        let text = to_text(&s, &Settings::default());
        assert!(text.contains("meat thermometer"));
        assert!(text.contains("4:20 PM"));
    }
}
