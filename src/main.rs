mod app;

use app::AppState;

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
            ui.heading("Hello from Feather Calendar!");
        });
    }
}
