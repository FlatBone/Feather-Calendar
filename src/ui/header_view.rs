use egui::Ui;
use crate::app::{AppState, ViewMode};
use chrono::{Datelike, NaiveDate, Months};

pub fn header_view(ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);

        // Previous month button with rounded icon style
        let prev_btn = egui::Button::new("<").rounding(5.0);
        if ui.add(prev_btn).clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
            app_state.current_month = (prev_month_date.year(), prev_month_date.month());
        }

        ui.add_space(8.0);

        // Today button with emphasis
        let accent_color = ui.style().visuals.selection.bg_fill;
        let today_btn = egui::Button::new("Today").fill(accent_color);
        if ui.add(today_btn).clicked() {
            let now = chrono::Local::now().date_naive();
            app_state.current_month = (now.year(), now.month());
        }

        ui.add_space(8.0);

        // Next month button with rounded icon style
        let next_btn = egui::Button::new(">").rounding(5.0);
        if ui.add(next_btn).clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();
            app_state.current_month = (next_month_date.year(), next_month_date.month());
        }

        ui.add_space(12.0);

        // View mode toggle button
        let view_text = match app_state.view_mode {
            ViewMode::SingleMonth => "1M",
            ViewMode::ThreeMonths => "3M",
        };
        if ui.button(view_text).clicked() {
            app_state.view_mode = match app_state.view_mode {
                ViewMode::SingleMonth => ViewMode::ThreeMonths,
                ViewMode::ThreeMonths => ViewMode::SingleMonth,
            };
        }

        ui.add_space(8.0);

        // Pin to top toggle switch
        let toggle_text = if app_state.is_always_on_top { "📌 ON " } else { "📍 OFF " };
        ui.toggle_value(&mut app_state.is_always_on_top, toggle_text);

        ui.add_space(8.0);

        // Reset marked dates button
        if ui.button("Clear").clicked() {
            app_state.marked_dates.clear();
        }

        ui.add_space(4.0);
    });
}
