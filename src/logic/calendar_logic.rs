use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeekStart {
    Sunday,
    Monday,
}

impl Default for WeekStart {
    fn default() -> Self {
        Self::Sunday
    }
}

impl WeekStart {
    pub const fn weekdays(self) -> [Weekday; 7] {
        match self {
            Self::Sunday => [
                Weekday::Sun,
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
            ],
            Self::Monday => [
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
                Weekday::Sat,
                Weekday::Sun,
            ],
        }
    }

    fn days_from_start(self, weekday: Weekday) -> u32 {
        match self {
            Self::Sunday => weekday.num_days_from_sunday(),
            Self::Monday => weekday.num_days_from_monday(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeekNumberRule {
    Iso8601,
    SundayDate,
}

impl Default for WeekNumberRule {
    fn default() -> Self {
        Self::Iso8601
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDay {
    pub date: NaiveDate,
    pub is_current_month: bool,
}

pub fn generate_calendar_days(year: i32, month: u32) -> Vec<CalendarDay> {
    generate_calendar_days_with_week_start(year, month, WeekStart::Sunday)
}

pub fn generate_calendar_days_with_week_start(
    year: i32,
    month: u32,
    week_start: WeekStart,
) -> Vec<CalendarDay> {
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    let mut days = Vec::new();

    // Add days from the previous month to fill the first week
    let weekday_of_first_day = first_day_of_month.weekday();
    let days_from_prev_month = week_start.days_from_start(weekday_of_first_day);
    for i in (0..days_from_prev_month).rev() {
        days.push(CalendarDay {
            date: first_day_of_month - Duration::days(i as i64 + 1),
            is_current_month: false,
        });
    }

    // Add days of the current month
    for day in first_day_of_month
        .iter_days()
        .take(last_day_of_month.day() as usize)
    {
        days.push(CalendarDay {
            date: day,
            is_current_month: true,
        });
    }

    // Add days from the next month to fill the last week
    let weekday_of_last_day = last_day_of_month.weekday();
    let days_from_next_month = 6 - week_start.days_from_start(weekday_of_last_day);
    if days_from_next_month > 0 {
        for i in 1..=days_from_next_month {
            days.push(CalendarDay {
                date: last_day_of_month + Duration::days(i as i64),
                is_current_month: false,
            });
        }
    }

    days
}

pub fn week_number_for_row(days: &[CalendarDay], rule: WeekNumberRule) -> Option<u32> {
    let representative_weekday = match rule {
        WeekNumberRule::Iso8601 => Weekday::Thu,
        WeekNumberRule::SundayDate => Weekday::Sun,
    };

    days.iter()
        .find(|day| day.date.weekday() == representative_weekday)
        .map(|day| day.date.iso_week().week())
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
        assert!(!days[0].is_current_month);

        // 3. Check the first day of the actual month (July 1st)
        let first_day_of_month = days
            .iter()
            .find(|d| d.date.day() == 1 && d.is_current_month)
            .unwrap();
        assert_eq!(first_day_of_month.date.weekday(), Weekday::Mon);

        // 4. Check number of days in the current month
        let current_month_days = days.iter().filter(|d| d.is_current_month).count();
        assert_eq!(current_month_days, 31);

        // 5. Check the last day of the grid (should be Saturday, August 3, 2024)
        assert_eq!(
            days.last().unwrap().date,
            NaiveDate::from_ymd_opt(2024, 8, 3).unwrap()
        );
        assert!(!days.last().unwrap().is_current_month);
    }

    #[test]
    fn test_generate_calendar_for_july_2024_with_monday_start() {
        let days = generate_calendar_days_with_week_start(2024, 7, WeekStart::Monday);

        assert_eq!(days.len(), 35);
        assert_eq!(days[0].date, NaiveDate::from_ymd_opt(2024, 7, 1).unwrap());
        assert_eq!(
            days.last().unwrap().date,
            NaiveDate::from_ymd_opt(2024, 8, 4).unwrap()
        );
    }

    #[test]
    fn test_four_week_months_are_supported() {
        let sunday_start = generate_calendar_days_with_week_start(2026, 2, WeekStart::Sunday);
        let monday_start = generate_calendar_days_with_week_start(2021, 2, WeekStart::Monday);

        assert_eq!(sunday_start.len(), 28);
        assert_eq!(monday_start.len(), 28);
        assert_eq!(
            sunday_start[0].date,
            NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()
        );
        assert_eq!(
            monday_start[0].date,
            NaiveDate::from_ymd_opt(2021, 2, 1).unwrap()
        );
    }

    #[test]
    fn test_leap_month_and_six_week_month() {
        let leap_month = generate_calendar_days_with_week_start(2024, 2, WeekStart::Sunday);
        let six_week_month = generate_calendar_days_with_week_start(2024, 9, WeekStart::Monday);

        assert_eq!(
            leap_month.iter().filter(|day| day.is_current_month).count(),
            29
        );
        assert_eq!(six_week_month.len(), 42);
        assert_eq!(
            six_week_month[0].date,
            NaiveDate::from_ymd_opt(2024, 8, 26).unwrap()
        );
        assert_eq!(
            six_week_month.last().unwrap().date,
            NaiveDate::from_ymd_opt(2024, 10, 6).unwrap()
        );
    }

    #[test]
    fn test_generated_days_are_complete_consecutive_weeks() {
        for week_start in [WeekStart::Sunday, WeekStart::Monday] {
            for (year, month) in [(2024, 1), (2024, 2), (2025, 12), (2026, 2)] {
                let days = generate_calendar_days_with_week_start(year, month, week_start);

                assert_eq!(days.len() % 7, 0);
                assert_eq!(days[0].date.weekday(), week_start.weekdays()[0]);
                for pair in days.windows(2) {
                    assert_eq!(pair[1].date - pair[0].date, Duration::days(1));
                }
            }
        }
    }

    #[test]
    fn test_week_number_rules_at_iso_year_boundary() {
        let days = generate_calendar_days_with_week_start(2025, 1, WeekStart::Sunday);
        let first_row = &days[..7];

        assert_eq!(
            first_row[0].date,
            NaiveDate::from_ymd_opt(2024, 12, 29).unwrap()
        );
        assert_eq!(
            week_number_for_row(first_row, WeekNumberRule::Iso8601),
            Some(1)
        );
        assert_eq!(
            week_number_for_row(first_row, WeekNumberRule::SundayDate),
            Some(52)
        );
    }

    #[test]
    fn test_week_number_requires_the_representative_weekday() {
        let partial_row = [CalendarDay {
            date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            is_current_month: true,
        }];

        assert_eq!(
            week_number_for_row(&partial_row, WeekNumberRule::Iso8601),
            None
        );
        assert_eq!(
            week_number_for_row(&partial_row, WeekNumberRule::SundayDate),
            None
        );
    }
}
