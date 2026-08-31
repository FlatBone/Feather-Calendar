use crate::logic::calendar_logic::CalendarDay;
use egui::{Align, Button, Color32, Layout, RichText, Ui, Vec2, Visuals};
use chrono::{Datelike, NaiveDate};
use std::collections::HashSet;

pub fn calendar_view(ui: &mut Ui, year: i32, month: u32, days: &[CalendarDay], marked_dates: &mut HashSet<NaiveDate>, visuals: &Visuals) {
    ui.vertical(|ui| {
        ui.add_space(16.0);
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(format!("{}/{}", year, month)).size(20.0).strong());
        });
        ui.add_space(14.0);

        // Weekday headers
        ui.columns(7, |columns| {
            let weekdays = ["日", "月", "火", "水", "木", "金", "土"];
            for (i, column) in columns.iter_mut().enumerate() {
                column.with_layout(Layout::top_down(Align::Center), |ui| {
                    let text_color = match i {
                        0 => Color32::from_rgb(220, 50, 50),   // Sunday - softer red
                        6 => Color32::from_rgb(50, 100, 200),  // Saturday - softer blue
                        _ => visuals.text_color(), // Weekdays
                    };
                    ui.label(RichText::new(weekdays[i]).color(text_color));
                });
            }
        });

        ui.separator();
        ui.add_space(6.0);

        // Calendar days
        let mut day_iter = days.iter();

        for _week in 0..6 { // Max 6 weeks in a month view
            if day_iter.len() == 0 {
                break;
            }
            ui.columns(7, |columns| {
                for column in columns.iter_mut() {
                    if let Some(day) = day_iter.next() {
                        column.with_layout(Layout::top_down(Align::Center), |ui| {
                            // Get weekday number (0=Sunday, 6=Saturday)
                            let weekday_num = day.date.weekday().num_days_from_sunday();

                            // Apply weekday coloring to date cells
                            let text_color = if day.is_current_month {
                                match weekday_num {
                                    0 => Color32::from_rgb(220, 50, 50),   // Sunday - red
                                    6 => Color32::from_rgb(50, 100, 200),  // Saturday - blue
                                    _ => visuals.text_color(),             // Weekdays
                                }
                            } else {
                                Color32::DARK_GRAY // Non-current month stays gray
                            };

                            let mut text = RichText::new(format!("{}", day.date.day())).color(text_color);

                            let is_marked = marked_dates.contains(&day.date);

                            // Highlight today's date with theme color
                            if day.date == chrono::Local::now().date_naive() {
                                let today_bg = visuals.selection.bg_fill;
                                text = text.background_color(today_bg).color(Color32::WHITE);
                            } else if is_marked {
                                // Use warn color for marked dates (theme-aware)
                                let mark_bg = visuals.warn_fg_color;
                                text = text.background_color(mark_bg).color(Color32::WHITE);
                            }

                            let available_width = ui.available_width();
                            let cell_size = Vec2::new(available_width, 32.0);

                            let button = Button::new(text)
                                .min_size(cell_size)
                                .rounding(4.0)
                                .frame(true);

                            let response = ui.add(button).on_hover_cursor(egui::CursorIcon::PointingHand);

                            if response.clicked() {
                                if day.is_current_month {
                                    if is_marked {
                                        marked_dates.remove(&day.date);
                                    } else {
                                        marked_dates.insert(day.date);
                                    }
                                }
                            }
                        });
                    }
                }
            });
            ui.add_space(3.0);
        }
    });
}