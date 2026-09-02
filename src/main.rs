#![windows_subsystem = "windows"]

use chrono::{Datelike, Months, NaiveDate};
use feather_calendar::app::{AppConfig, AppState, ViewMode, WindowPosition};
use feather_calendar::logic::calendar_logic;
use image::GenericImageView;

const THREE_MONTH_WINDOW_SIZE: [f32; 2] = [860.0, 350.0];
const SINGLE_MONTH_WINDOW_SIZE: [f32; 2] = [340.0, 350.0];
const SINGLE_MONTH_CALENDAR_WIDTH: f32 = 280.0;
const WEEK_NUMBER_CALENDAR_EXTRA_WIDTH: f32 = 26.0;

fn main() -> eframe::Result<()> {
    let icon = load_icon();
    let config = AppConfig::load();
    // Create or migrate the portable config file as soon as the app starts.
    let _ = config.save();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(window_size(config.view_mode))
        .with_icon(icon)
        .with_resizable(true);
    if let Some(position) = config.window_position {
        viewport = viewport.with_position(egui::Pos2::from(position));
    }

    let native_options = eframe::NativeOptions {
        viewport,
        persist_window: false,
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
    window_position: Option<WindowPosition>,
}

impl FeatherCalendarApp {
    fn new(cc: &eframe::CreationContext<'_>, config: AppConfig) -> Self {
        feather_calendar::ui::fonts::install_japanese_weekday_font(&cc.egui_ctx);

        let now = chrono::Local::now().date_naive();
        let (year, month) = (now.year(), now.month());
        let view_mode = config.view_mode;
        let mut app = Self {
            app_state: AppState {
                current_month: (year, month),
                marked_dates: config.marked_dates,
                is_always_on_top: config.is_always_on_top,
                view_mode,
                display_settings: config.display_settings,
                ..Default::default()
            },
            previous_view_mode: view_mode,
            window_position: config.window_position,
        };
        app.update_calendar_days();
        app
    }

    fn update_calendar_days(&mut self) {
        let (year, month) = self.app_state.current_month;
        let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let prev_month_date = current_month_date
            .checked_sub_months(Months::new(1))
            .unwrap();
        let next_month_date = current_month_date
            .checked_add_months(Months::new(1))
            .unwrap();

        self.app_state.calendar_days = (
            calendar_logic::generate_calendar_days_with_week_start(
                prev_month_date.year(),
                prev_month_date.month(),
                self.app_state.display_settings.week_start,
            ),
            calendar_logic::generate_calendar_days_with_week_start(
                year,
                month,
                self.app_state.display_settings.week_start,
            ),
            calendar_logic::generate_calendar_days_with_week_start(
                next_month_date.year(),
                next_month_date.month(),
                self.app_state.display_settings.week_start,
            ),
        );
    }

    fn save_config(&self) {
        let config = AppConfig {
            window_position: self.window_position,
            marked_dates: self.app_state.marked_dates.clone(),
            is_always_on_top: self.app_state.is_always_on_top,
            view_mode: self.app_state.view_mode,
            display_settings: self.app_state.display_settings,
        };
        let _ = config.save();
    }
}

impl eframe::App for FeatherCalendarApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(rect) = ctx.input(|input| input.viewport().outer_rect) {
            self.window_position = Some(rect.left_top().into());
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
        let week_start_before = self.app_state.display_settings.week_start;

        // 表示モードが変更された場合、ウィンドウサイズを調整
        if self.app_state.view_mode != self.previous_view_mode {
            let new_size = window_size(self.app_state.view_mode);
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::from(new_size)));
            self.previous_view_mode = self.app_state.view_mode;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            feather_calendar::ui::header_view::header_view(ui, &mut self.app_state);
            ui.separator();

            if self.app_state.current_month != current_month_before
                || self.app_state.display_settings.week_start != week_start_before
            {
                self.update_calendar_days();
            }

            // Calendars
            let (year, month) = self.app_state.current_month;
            let current_month_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let prev_month_date = current_month_date
                .checked_sub_months(Months::new(1))
                .unwrap();
            let next_month_date = current_month_date
                .checked_add_months(Months::new(1))
                .unwrap();

            let visuals = ui.style().visuals.clone();

            match self.app_state.view_mode {
                ViewMode::SingleMonth => {
                    // 一ヶ月表示
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.set_width(single_month_calendar_width(
                            self.app_state.display_settings.show_week_numbers,
                        ));
                        feather_calendar::ui::calendar_view::calendar_view(
                            ui,
                            year,
                            month,
                            &self.app_state.calendar_days.1,
                            &mut self.app_state.marked_dates,
                            &visuals,
                            self.app_state.display_settings,
                        );
                    });
                }
                ViewMode::ThreeMonths => {
                    // 三ヶ月表示（既存の実装）
                    let calendar_width = (ui.available_width() - 50.0) / 3.0;
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(
                                ui,
                                prev_month_date.year(),
                                prev_month_date.month(),
                                &self.app_state.calendar_days.0,
                                &mut self.app_state.marked_dates,
                                &visuals,
                                self.app_state.display_settings,
                            );
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(
                                ui,
                                year,
                                month,
                                &self.app_state.calendar_days.1,
                                &mut self.app_state.marked_dates,
                                &visuals,
                                self.app_state.display_settings,
                            );
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.set_width(calendar_width);
                            feather_calendar::ui::calendar_view::calendar_view(
                                ui,
                                next_month_date.year(),
                                next_month_date.month(),
                                &self.app_state.calendar_days.2,
                                &mut self.app_state.marked_dates,
                                &visuals,
                                self.app_state.display_settings,
                            );
                        });
                    });
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_config();
    }
}

const fn window_size(view_mode: ViewMode) -> [f32; 2] {
    match view_mode {
        ViewMode::SingleMonth => SINGLE_MONTH_WINDOW_SIZE,
        ViewMode::ThreeMonths => THREE_MONTH_WINDOW_SIZE,
    }
}

const fn single_month_calendar_width(show_week_numbers: bool) -> f32 {
    SINGLE_MONTH_CALENDAR_WIDTH
        + if show_week_numbers {
            WEEK_NUMBER_CALENDAR_EXTRA_WIDTH
        } else {
            0.0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_month_week_numbers_get_extra_space_without_changing_window_size() {
        assert_eq!(window_size(ViewMode::SingleMonth), [340.0, 350.0]);
        assert_eq!(single_month_calendar_width(false), 280.0);
        assert_eq!(single_month_calendar_width(true), 306.0);
        assert!(single_month_calendar_width(true) < SINGLE_MONTH_WINDOW_SIZE[0]);
    }
}
