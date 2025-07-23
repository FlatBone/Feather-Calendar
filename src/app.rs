use crate::logic::calendar_logic::CalendarDay;
use chrono::NaiveDate;
use std::collections::HashSet;

pub struct AppState {
    // 表示の中心となる年月
    pub current_month: (i32, u32), // (year, month)
    // 色付けされた日付の集合
    pub marked_dates: HashSet<NaiveDate>,
    // 最前面表示の状態
    pub is_always_on_top: bool,
    // 表示するカレンダーのデータ（キャッシュ）
    pub calendar_days: (Vec<CalendarDay>, Vec<CalendarDay>, Vec<CalendarDay>), // (prev, current, next)
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_month: (0, 0),
            marked_dates: HashSet::new(),
            is_always_on_top: false,
            calendar_days: (Vec::new(), Vec::new(), Vec::new()),
        }
    }
}