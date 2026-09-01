use crate::app::{AppState, ViewMode, WeekdayLanguage};
use crate::logic::calendar_logic::{WeekNumberRule, WeekStart};
use chrono::{Datelike, Months, NaiveDate};
use egui::Ui;

pub fn header_view(ui: &mut Ui, app_state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);

        let prev_btn = egui::Button::new("<").rounding(5.0);
        if ui.add(prev_btn).clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let prev_month_date = current_month_date
                .checked_sub_months(Months::new(1))
                .unwrap();
            app_state.current_month = (prev_month_date.year(), prev_month_date.month());
        }

        ui.add_space(8.0);

        let accent_color = ui.style().visuals.selection.bg_fill;
        let today_btn = egui::Button::new("Today").fill(accent_color);
        if ui.add(today_btn).clicked() {
            let now = chrono::Local::now().date_naive();
            app_state.current_month = (now.year(), now.month());
        }

        ui.add_space(8.0);

        let next_btn = egui::Button::new(">").rounding(5.0);
        if ui.add(next_btn).clicked() {
            let (year, month) = app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month_date = current_month_date
                .checked_add_months(Months::new(1))
                .unwrap();
            app_state.current_month = (next_month_date.year(), next_month_date.month());
        }

        ui.add_space(12.0);

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

        ui.menu_button("⚙", |ui| {
            ui.set_min_width(190.0);

            ui.label("Weekday labels");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut app_state.display_settings.weekday_language,
                    WeekdayLanguage::English,
                    "EN",
                );
                ui.selectable_value(
                    &mut app_state.display_settings.weekday_language,
                    WeekdayLanguage::Japanese,
                    "日",
                );
            });

            ui.separator();
            ui.checkbox(
                &mut app_state.display_settings.show_week_numbers,
                "Show week numbers",
            );

            ui.separator();
            ui.label("Week starts");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut app_state.display_settings.week_start,
                    WeekStart::Sunday,
                    "S",
                )
                .on_hover_text("Sunday");
                ui.selectable_value(
                    &mut app_state.display_settings.week_start,
                    WeekStart::Monday,
                    "M",
                )
                .on_hover_text("Monday");
            });

            ui.separator();
            ui.label("Week-number rule");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut app_state.display_settings.week_number_rule,
                    WeekNumberRule::Iso8601,
                    "ISO",
                )
                .on_hover_text("ISO 8601: use the Thursday in each row");
                ui.selectable_value(
                    &mut app_state.display_settings.week_number_rule,
                    WeekNumberRule::SundayDate,
                    "Sun",
                )
                .on_hover_text("Use the ISO week of the Sunday in each row");
            });
        });

        ui.add_space(8.0);

        let toggle_text = if app_state.is_always_on_top {
            "📌 ON "
        } else {
            "📍 OFF "
        };
        ui.toggle_value(&mut app_state.is_always_on_top, toggle_text);

        ui.add_space(8.0);

        if ui.button("Clear").clicked() {
            app_state.marked_dates.clear();
        }

        ui.add_space(4.0);
    });
}
