#![windows_subsystem = "windows"]

use feather_calendar::app::{AppState, ViewMode};
use chrono::{Datelike, NaiveDate, Months};
use feather_calendar::logic::calendar_logic;
use image::GenericImageView;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct AppConfig {
    window_position: Option<egui::Pos2>,
    marked_dates: HashSet<NaiveDate>,
    is_always_on_top: bool,
    view_mode: ViewMode,
}

fn get_config_path() -> io::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Cannot get parent directory of executable")
    })?;
    Ok(exe_dir.join("feather_calendar_config.json"))
}

fn load_config() -> AppConfig {
    let config_path = match get_config_path() {
        Ok(path) => path,
        Err(_) => return AppConfig::default(),
    };

    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(_) => return AppConfig::default(),
    };

    serde_json::from_str(&content).unwrap_or_default()
}

fn save_config(config: &AppConfig) {
    let config_path = match get_config_path() {
        Ok(path) => path,
        Err(_) => return,
    };

    let json = match serde_json::to_string_pretty(config) {
        Ok(j) => j,
        Err(_) => return,
    };

    let _ = fs::write(&config_path, json);
}

fn main() -> eframe::Result<()> {
    let icon = load_icon();
    let config = load_config();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([860.0, 310.0])
        .with_icon(icon)
        .with_resizable(true);

    if let Some(pos) = config.window_position {
        viewport = viewport.with_position(pos);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Feather Calendar",
        native_options,
        Box::new(move |cc| Box::new(FeatherCalendarApp::new(cc, config))),
    )
}

struct FeatherCalendarApp {
    app_state: AppState,
    previous_view_mode: ViewMode,
    window_position: Option<egui::Pos2>,
}

impl FeatherCalendarApp {
    fn new(_cc: &eframe::CreationContext<'_>, config: AppConfig) -> Self {
        // 設定ファイルから前回の状態を復元
        let now = chrono::Local::now().date_naive();
        let app_state = AppState {
            current_month: (now.year(), now.month()),
            marked_dates: config.marked_dates,
            is_always_on_top: config.is_always_on_top,
            view_mode: config.view_mode,
            calendar_days: (Vec::new(), Vec::new(), Vec::new()),
        };

        let view_mode = app_state.view_mode;
        let mut app = Self {
            app_state,
            previous_view_mode: view_mode,
            window_position: None,
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
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // 自前の設定ファイルに保存するため、eframe標準の保存は使わない
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // アプリ終了時に設定を保存
        let config = AppConfig {
            window_position: self.window_position,
            marked_dates: self.app_state.marked_dates.clone(),
            is_always_on_top: self.app_state.is_always_on_top,
            view_mode: self.app_state.view_mode,
        };
        save_config(&config);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // ウィンドウ位置を記録
        if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
            self.window_position = Some(rect.left_top());
        }

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