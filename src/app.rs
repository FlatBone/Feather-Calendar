use crate::logic::calendar_logic::{CalendarDay, WeekNumberRule, WeekStart};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE_NAME: &str = "feather_calendar_config.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ViewMode {
    SingleMonth,
    #[default]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: f32,
    pub y: f32,
}

impl From<egui::Pos2> for WindowPosition {
    fn from(position: egui::Pos2) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

impl From<WindowPosition> for egui::Pos2 {
    fn from(position: WindowPosition) -> Self {
        Self::new(position.x, position.y)
    }
}

/// Portable application settings stored next to the executable.
///
/// The first four fields intentionally match the v0.1.5 JSON schema. Adding
/// `display_settings` with `#[serde(default)]` keeps existing files readable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AppConfig {
    pub window_position: Option<WindowPosition>,
    pub marked_dates: HashSet<NaiveDate>,
    pub is_always_on_top: bool,
    pub view_mode: ViewMode,
    pub display_settings: DisplaySettings,
}

impl AppConfig {
    pub fn path() -> io::Result<PathBuf> {
        let executable = std::env::current_exe()?;
        config_path_for_executable(&executable)
    }

    pub fn load() -> Self {
        Self::path()
            .ok()
            .and_then(|path| fs::read_to_string(path).ok())
            .map(|json| Self::from_json(&json))
            .unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
        self.save_to_path(&Self::path()?)
    }

    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap_or_default()
    }

    fn save_to_path(&self, path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, json)
    }
}

pub fn config_path_for_executable(executable: &Path) -> io::Result<PathBuf> {
    let directory = executable.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "cannot determine the executable directory",
        )
    })?;
    Ok(directory.join(CONFIG_FILE_NAME))
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

    fn display_settings_from_json(json: &str) -> DisplaySettings {
        serde_json::from_str(json).unwrap_or_default()
    }

    #[test]
    fn display_settings_round_trip_through_json() {
        let settings = DisplaySettings {
            weekday_language: WeekdayLanguage::Japanese,
            show_week_numbers: true,
            week_start: WeekStart::Monday,
            week_number_rule: WeekNumberRule::SundayDate,
        };
        let json = serde_json::to_string(&settings).unwrap();

        assert_eq!(display_settings_from_json(&json), settings);
    }

    #[test]
    fn display_settings_use_defaults_for_missing_fields() {
        assert_eq!(
            display_settings_from_json(r#"{"weekday_language":"japanese"}"#),
            DisplaySettings {
                weekday_language: WeekdayLanguage::Japanese,
                ..Default::default()
            }
        );
    }

    #[test]
    fn display_settings_use_defaults_for_corrupt_storage() {
        assert_eq!(
            display_settings_from_json("not valid json"),
            DisplaySettings::default()
        );
    }

    #[test]
    fn v0_1_5_config_is_loaded_with_default_display_settings() {
        let legacy_json = r#"
        {
          "window_position": { "x": 120.5, "y": 80.25 },
          "marked_dates": ["2026-09-01"],
          "is_always_on_top": true,
          "view_mode": "SingleMonth"
        }
        "#;

        let config = AppConfig::from_json(legacy_json);

        assert_eq!(
            config.window_position,
            Some(WindowPosition { x: 120.5, y: 80.25 })
        );
        assert!(
            config
                .marked_dates
                .contains(&NaiveDate::from_ymd_opt(2026, 9, 1).unwrap())
        );
        assert!(config.is_always_on_top);
        assert_eq!(config.view_mode, ViewMode::SingleMonth);
        assert_eq!(config.display_settings, DisplaySettings::default());
    }

    #[test]
    fn config_path_is_next_to_executable() {
        let executable = Path::new(r"C:\Portable\Feather-Calendar.exe");

        assert_eq!(
            config_path_for_executable(executable).unwrap(),
            PathBuf::from(r"C:\Portable\feather_calendar_config.json")
        );
    }

    #[test]
    fn config_file_is_created_and_round_trips() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "feather_calendar_config_test_{}_{}.json",
            std::process::id(),
            unique
        ));
        let config = AppConfig {
            is_always_on_top: true,
            view_mode: ViewMode::SingleMonth,
            display_settings: DisplaySettings {
                show_week_numbers: true,
                ..Default::default()
            },
            ..Default::default()
        };

        config.save_to_path(&path).unwrap();
        let restored = AppConfig::from_json(&fs::read_to_string(&path).unwrap());
        fs::remove_file(&path).unwrap();

        assert_eq!(restored, config);
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
