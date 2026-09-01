use crate::logic::calendar_logic::{CalendarDay, WeekNumberRule, WeekStart};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const DISPLAY_SETTINGS_STORAGE_KEY: &str = "display_settings";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    SingleMonth,
    ThreeMonths,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeekdayLanguage {
    English,
    Japanese,
}

impl Default for WeekdayLanguage {
    fn default() -> Self {
        Self::English
    }
}

impl WeekdayLanguage {
    pub const fn label(self, weekday: chrono::Weekday) -> &'static str {
        match (self, weekday) {
            (Self::English, chrono::Weekday::Sun) => "Sun.",
            (Self::English, chrono::Weekday::Mon) => "Mon.",
            (Self::English, chrono::Weekday::Tue) => "Tue.",
            (Self::English, chrono::Weekday::Wed) => "Wed.",
            (Self::English, chrono::Weekday::Thu) => "Thu.",
            (Self::English, chrono::Weekday::Fri) => "Fri.",
            (Self::English, chrono::Weekday::Sat) => "Sat.",
            (Self::Japanese, chrono::Weekday::Sun) => "日",
            (Self::Japanese, chrono::Weekday::Mon) => "月",
            (Self::Japanese, chrono::Weekday::Tue) => "火",
            (Self::Japanese, chrono::Weekday::Wed) => "水",
            (Self::Japanese, chrono::Weekday::Thu) => "木",
            (Self::Japanese, chrono::Weekday::Fri) => "金",
            (Self::Japanese, chrono::Weekday::Sat) => "土",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DisplaySettings {
    pub weekday_language: WeekdayLanguage,
    pub show_week_numbers: bool,
    pub week_start: WeekStart,
    pub week_number_rule: WeekNumberRule,
}

impl DisplaySettings {
    pub fn load(storage: Option<&dyn eframe::Storage>) -> Self {
        storage
            .and_then(|storage| eframe::get_value(storage, DISPLAY_SETTINGS_STORAGE_KEY))
            .unwrap_or_default()
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, DISPLAY_SETTINGS_STORAGE_KEY, self);
    }
}

pub struct AppState {
    // 表示の中心となる年月
    pub current_month: (i32, u32), // (year, month)
    // 色付けされた日付の集合
    pub marked_dates: HashSet<NaiveDate>,
    // 最前面表示の状態
    pub is_always_on_top: bool,
    // 表示モード（一ヶ月/三ヶ月）
    pub view_mode: ViewMode,
    // 表示するカレンダーのデータ（キャッシュ）
    pub calendar_days: (Vec<CalendarDay>, Vec<CalendarDay>, Vec<CalendarDay>), // (prev, current, next)
    // 曜日、週番号、週始まりに関する表示設定
    pub display_settings: DisplaySettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_month: (0, 0),
            marked_dates: HashSet::new(),
            is_always_on_top: false,
            view_mode: ViewMode::ThreeMonths, // デフォルトは三ヶ月表示
            calendar_days: (Vec::new(), Vec::new(), Vec::new()),
            display_settings: DisplaySettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::Storage as _;
    use std::collections::HashMap;

    #[derive(Default)]
    struct MemoryStorage {
        values: HashMap<String, String>,
    }

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.values.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.values.insert(key.to_owned(), value);
        }

        fn flush(&mut self) {}
    }

    #[test]
    fn display_settings_round_trip_through_storage() {
        let settings = DisplaySettings {
            weekday_language: WeekdayLanguage::Japanese,
            show_week_numbers: true,
            week_start: WeekStart::Monday,
            week_number_rule: WeekNumberRule::SundayDate,
        };
        let mut storage = MemoryStorage::default();

        settings.save(&mut storage);

        assert_eq!(DisplaySettings::load(Some(&storage)), settings);
    }

    #[test]
    fn display_settings_use_defaults_for_missing_fields() {
        #[derive(Serialize)]
        struct LegacyDisplaySettings {
            weekday_language: WeekdayLanguage,
        }

        let mut storage = MemoryStorage::default();
        eframe::set_value(
            &mut storage,
            DISPLAY_SETTINGS_STORAGE_KEY,
            &LegacyDisplaySettings {
                weekday_language: WeekdayLanguage::Japanese,
            },
        );

        assert_eq!(
            DisplaySettings::load(Some(&storage)),
            DisplaySettings {
                weekday_language: WeekdayLanguage::Japanese,
                ..Default::default()
            }
        );
    }

    #[test]
    fn display_settings_use_defaults_for_corrupt_storage() {
        let mut storage = MemoryStorage::default();
        storage.set_string(DISPLAY_SETTINGS_STORAGE_KEY, "not valid ron".to_owned());

        assert_eq!(
            DisplaySettings::load(Some(&storage)),
            DisplaySettings::default()
        );
    }

    #[test]
    fn weekday_labels_cover_both_languages_and_start_days() {
        for language in [WeekdayLanguage::English, WeekdayLanguage::Japanese] {
            for week_start in [WeekStart::Sunday, WeekStart::Monday] {
                let labels: Vec<_> = week_start
                    .weekdays()
                    .into_iter()
                    .map(|weekday| language.label(weekday))
                    .collect();

                match (language, week_start) {
                    (WeekdayLanguage::English, WeekStart::Sunday) => assert_eq!(
                        labels,
                        ["Sun.", "Mon.", "Tue.", "Wed.", "Thu.", "Fri.", "Sat."]
                    ),
                    (WeekdayLanguage::English, WeekStart::Monday) => assert_eq!(
                        labels,
                        ["Mon.", "Tue.", "Wed.", "Thu.", "Fri.", "Sat.", "Sun."]
                    ),
                    (WeekdayLanguage::Japanese, WeekStart::Sunday) => {
                        assert_eq!(labels, ["日", "月", "火", "水", "木", "金", "土"])
                    }
                    (WeekdayLanguage::Japanese, WeekStart::Monday) => {
                        assert_eq!(labels, ["月", "火", "水", "木", "金", "土", "日"])
                    }
                }
            }
        }
    }
}
