use std::collections::HashSet;
use chrono::NaiveDate;

#[derive(Default)]
pub struct AppState {
    // 表示の中心となる年月
    pub current_month: (i32, u32), // (year, month)
    // 色付けされた日付の集合
    pub marked_dates: HashSet<NaiveDate>,
    // 最前面表示の状態
    pub is_always_on_top: bool,
}