#![windows_subsystem = "windows"]

use feather_calendar::app::{AppState, ViewMode};
use chrono::{Datelike, NaiveDate, Months};
use feather_calendar::logic::calendar_logic;
use image::GenericImageView;

fn main() -> eframe::Result<()> {
    let icon = load_icon();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([860.0, 310.0]).with_icon(icon).with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Feather Calendar",
        native_options,
        Box::new(|_cc| Box::new(FeatherCalendarApp::new())),
    )
}

struct FeatherCalendarApp {
    app_state: AppState,
    previous_view_mode: ViewMode,
}

impl FeatherCalendarApp {
    fn new() -> Self {
        let now = chrono::Local::now().date_naive();
        let (year, month) = (now.year(), now.month());
        let mut app = Self {
            app_state: AppState {
                current_month: (year, month),
                ..Default::default()
            },
            previous_view_mode: ViewMode::ThreeMonths,
        };
        app.update_calendar_days();
        app
    }

    fn update_calendar_days(&mut self) {
        let (year, month) = self.app_state.current_month;
        let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
        let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();

        self.app_state.calendar_days = (
            calendar_logic::generate_calendar_days(prev_month_date.year(), prev_month_date.month()),
            calendar_logic::generate_calendar_days(year, month),
            calendar_logic::generate_calendar_days(next_month_date.year(), next_month_date.month()),
        );
    }
}

impl eframe::App for FeatherCalendarApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // OSのテーマ設定に応じてeguiのテーマを切り替える
        if let Some(theme) = frame.info().system_theme {
            ctx.set_visuals(match theme {
                eframe::Theme::Dark => egui::Visuals::dark(),
                eframe::Theme::Light => egui::Visuals::light(),
            });
        }

        let level = if self.app_state.is_always_on_top {
            egui::viewport::WindowLevel::AlwaysOnTop
        } else {
            egui::viewport::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));

        let current_month_before = self.app_state.current_month;

        // 表示モードが変更された場合、ウィンドウサイズを調整
        if self.app_state.view_mode != self.previous_view_mode {
            let new_size = match self.app_state.view_mode {
                ViewMode::SingleMonth => [300.0, 310.0], // 一ヶ月表示時のサイズ
                ViewMode::ThreeMonths => [860.0, 310.0], // 三ヶ月表示時のサイズ
            };
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::from(new_size)));
            self.previous_view_mode = self.app_state.view_mode;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            feather_calendar::ui::header_view::header_view(ui, &mut self.app_state);
            ui.separator();

            if self.app_state.current_month != current_month_before {
                self.update_calendar_days();
            }

            // Calendars
            let (year, month) = self.app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let prev_month_date = current_month_date.checked_sub_months(Months::new(1)).unwrap();
            let next_month_date = current_month_date.checked_add_months(Months::new(1)).unwrap();

            let visuals = ui.style().visuals.clone();

            match self.app_state.view_mode {
                ViewMode::SingleMonth => {
                    // 一ヶ月表示
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.set_max_width(280.0); // 一ヶ月表示時の最大幅を設定
                        feather_calendar::ui::calendar_view::calendar_view(ui, year, month, &self.app_state.calendar_days.1, &mut self.app_state.marked_dates, &visuals);
                    });
                }
                ViewMode::ThreeMonths => {
                    // 三ヶ月表示（既存の実装）
                    let calendar_width = (ui.available_width() - 50.0) / 3.0;
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(ui, prev_month_date.year(), prev_month_date.month(), &self.app_state.calendar_days.0, &mut self.app_state.marked_dates, &visuals);
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(ui, year, month, &self.app_state.calendar_days.1, &mut self.app_state.marked_dates, &visuals);
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(ui, next_month_date.year(), next_month_date.month(), &self.app_state.calendar_days.2, &mut self.app_state.marked_dates, &visuals);
                        });
                    });
                }
            }
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