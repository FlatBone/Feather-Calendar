use chrono::{Datelike, Duration, NaiveDate, Weekday};

pub struct CalendarDay {
    pub date: NaiveDate,
    pub is_current_month: bool,
}

pub fn generate_calendar_days(year: i32, month: u32) -> Vec<CalendarDay> {
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap().pred_opt().unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap()
    };

    let mut days = Vec::new();

    // Add days from the previous month to fill the first week
    let weekday_of_first_day = first_day_of_month.weekday();
    let days_from_prev_month = weekday_of_first_day.num_days_from_sunday();
    for i in (0..days_from_prev_month).rev() {
        days.push(CalendarDay {
            date: first_day_of_month - Duration::days(i as i64 + 1),
            is_current_month: false,
        });
    }

    // Add days of the current month
    for day in first_day_of_month.iter_days().take(last_day_of_month.day() as usize) {
        days.push(CalendarDay {
            date: day,
            is_current_month: true,
        });
    }

    // Add days from the next month to fill the last week
    let weekday_of_last_day = last_day_of_month.weekday();
    if weekday_of_last_day != Weekday::Sat {
        let days_from_next_month = 6 - weekday_of_last_day.num_days_from_sunday();
        for i in 1..=days_from_next_month {
            days.push(CalendarDay {
                date: last_day_of_month + Duration::days(i as i64),
                is_current_month: false,
            });
        }
    }

    days
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn test_generate_calendar_for_july_2024() {
        let year = 2024;
        let month = 7;
        let days = generate_calendar_days(year, month);

        // Expected: July 2024 starts on a Monday. The grid starts on Sunday, June 30th.
        // It has 31 days. The last day is Wednesday, July 31st.
        // The grid should end on Saturday, August 3rd.
        // Total days in grid: 1 (prev) + 31 (current) + 3 (next) = 35 days.

        // 1. Check total days in the grid view
        assert_eq!(days.len(), 35);

        // 2. Check the first day of the grid (should be Sunday, June 30, 2024)
        assert_eq!(days[0].date, NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
        assert_eq!(days[0].is_current_month, false);

        // 3. Check the first day of the actual month (July 1st)
        let first_day_of_month = days.iter().find(|d| d.date.day() == 1 && d.is_current_month).unwrap();
        assert_eq!(first_day_of_month.date.weekday(), Weekday::Mon);

        // 4. Check number of days in the current month
        let current_month_days = days.iter().filter(|d| d.is_current_month).count();
        assert_eq!(current_month_days, 31);

        // 5. Check the last day of the grid (should be Saturday, August 3, 2024)
        assert_eq!(days.last().unwrap().date, NaiveDate::from_ymd_opt(2024, 8, 3).unwrap());
        assert_eq!(days.last().unwrap().is_current_month, false);
    }
}