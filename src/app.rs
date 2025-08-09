use crate::logic::calendar_logic::CalendarDay;
use chrono::NaiveDate;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    SingleMonth,
    ThreeMonths,
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
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_month: (0, 0),
            marked_dates: HashSet::new(),
            is_always_on_top: false,
            view_mode: ViewMode::ThreeMonths, // デフォルトは三ヶ月表示
            calendar_days: (Vec::new(), Vec::new(), Vec::new()),
        }
    }
}