use egui::Ui;
use crate::app::AppState;
use chrono::{Datelike, NaiveDate, Months};

pub fn header_view(ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("<").clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
            app_state.current_month = (prev_month_date.year(), prev_month_date.month());
        }
        if ui.button("Today").clicked() {
            let now = chrono::Local::now().date_naive();
            app_state.current_month = (now.year(), now.month());
        }
        if ui.button(">").clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();
            app_state.current_month = (next_month_date.year(), next_month_date.month());
        }

        // Pin to top button
        let pin_button_text = if app_state.is_always_on_top { "📌" } else { "📍" };
        if ui.button(pin_button_text).clicked() {
            app_state.is_always_on_top = !app_state.is_always_on_top;
        }

        // Reset marked dates button
        if ui.button("Clear").clicked() {
            app_state.marked_dates.clear();
        }
    });
}
