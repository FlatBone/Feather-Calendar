mod app;

use feather_calendar::app::AppState;
use chrono::Datelike;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Feather Calendar",
        native_options,
        Box::new(|_cc| Box::new(FeatherCalendarApp::default())),
    )
}

#[derive(Default)]
struct FeatherCalendarApp {
    app_state: AppState,
}

impl eframe::App for FeatherCalendarApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let now = chrono::Local::now();
            let year = now.year();
            let month = now.month();
            feather_calendar::ui::calendar_view::calendar_view(ui, year, month);
        });
    }
}
