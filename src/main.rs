mod app;

use feather_calendar::app::AppState;
use chrono::{Datelike, NaiveDate, Months};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Feather Calendar",
        native_options,
        Box::new(|_cc| Box::new(FeatherCalendarApp::default())),
    )
}

struct FeatherCalendarApp {
    app_state: AppState,
}

impl Default for FeatherCalendarApp {
    fn default() -> Self {
        let now = chrono::Local::now().date_naive();
        Self {
            app_state: AppState {
                current_month: (now.year(), now.month()),
                marked_dates: Default::default(),
                is_always_on_top: false,
            },
        }
    }
}

impl eframe::App for FeatherCalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (year, month) = self.app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

            // 前月
            let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
            // 翌月
            let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();

            ui.columns(3, |columns| {
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[0], prev_month_date.year(), prev_month_date.month());
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[1], year, month);
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[2], next_month_date.year(), next_month_date.month());
            });
        });
    }
}
