#![windows_subsystem = "windows"]
mod app;

use feather_calendar::app::AppState;
use chrono::{Datelike, NaiveDate, Months};
use image::GenericImageView;

fn main() -> eframe::Result<()> {
    let icon = load_icon();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 300.0]).with_icon(icon),
        ..Default::default()
    };
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
        let level = if self.app_state.is_always_on_top {
            egui::viewport::WindowLevel::AlwaysOnTop
        } else {
            egui::viewport::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            feather_calendar::ui::header_view::header_view(ui, &mut self.app_state);
            ui.separator();

            // Calendars
            let (year, month) = self.app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

            // 前月
            let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
            // 翌月
            let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();

            ui.columns(3, |columns| {
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[0], prev_month_date.year(), prev_month_date.month(), &mut self.app_state.marked_dates);
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[1], year, month, &mut self.app_state.marked_dates);
                feather_calendar::ui::calendar_view::calendar_view(&mut columns[2], next_month_date.year(), next_month_date.month(), &mut self.app_state.marked_dates);
            });
        });
    }
}

fn load_icon() -> egui::IconData {
    // include_bytes!マクロでコンパイル時に画像をバイナリとして埋め込む
    let icon_bytes = include_bytes!("../icon.png");
    let image = image::load_from_memory_with_format(icon_bytes, image::ImageFormat::Png)
        .expect("Failed to load icon");
    let image_buffer = image.to_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image_buffer.into_raw(),
        width,
        height,
    }
}