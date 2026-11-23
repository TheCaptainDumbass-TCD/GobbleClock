use chrono::{Datelike, NaiveDate, Weekday};

/// US Thanksgiving: the fourth Thursday of November.
pub fn thanksgiving(year: i32) -> NaiveDate {
    NaiveDate::from_weekday_of_month_opt(year, 11, Weekday::Thu, 4).expect("November has four Thursdays")
}

/// This year's Thanksgiving, or next year's once this year's has passed.
pub fn next_thanksgiving(today: NaiveDate) -> NaiveDate {
    let this_year = thanksgiving(today.year());
    if today <= this_year {
        this_year
    } else {
        thanksgiving(today.year() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_dates() {
        assert_eq!(thanksgiving(2020), NaiveDate::from_ymd_opt(2020, 11, 26).unwrap());
        assert_eq!(thanksgiving(2021), NaiveDate::from_ymd_opt(2021, 11, 25).unwrap());
        assert_eq!(thanksgiving(2024), NaiveDate::from_ymd_opt(2024, 11, 28).unwrap());
        assert_eq!(thanksgiving(2025), NaiveDate::from_ymd_opt(2025, 11, 27).unwrap());
        assert_eq!(thanksgiving(2026), NaiveDate::from_ymd_opt(2026, 11, 26).unwrap());
    }

    #[test]
    fn rolls_to_next_year() {
        let after = NaiveDate::from_ymd_opt(2026, 11, 27).unwrap();
        assert_eq!(next_thanksgiving(after), thanksgiving(2027));
        let day_of = NaiveDate::from_ymd_opt(2026, 11, 26).unwrap();
        assert_eq!(next_thanksgiving(day_of), day_of);
    }
}
