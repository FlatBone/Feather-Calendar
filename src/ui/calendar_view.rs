use egui::{Align, Color32, Layout, RichText, Ui, Vec2, Visuals};
use chrono::{Datelike, NaiveDate};
use crate::logic::calendar_logic::{generate_calendar_days};
use std::collections::HashSet;

pub fn calendar_view(ui: &mut Ui, year: i32, month: u32, marked_dates: &mut HashSet<NaiveDate>, visuals: &Visuals) {
    ui.vertical(|ui| {
        ui.add_space(10.0);
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading(format!("{}/{}", year, month));
        });
        ui.add_space(10.0);

        // Weekday headers
        ui.columns(7, |columns| {
            let weekdays = ["Sun.", "Mon.", "Tue.", "Wed.", "Thu.", "Fri.", "Sat."];
            for (i, column) in columns.iter_mut().enumerate() {
                column.with_layout(Layout::top_down(Align::Center), |ui| {
                    let text_color = match i {
                        0 => Color32::RED,   // Sunday
                        6 => Color32::BLUE,  // Saturday
                        _ => visuals.text_color(), // Weekdays
                    };
                    ui.label(RichText::new(weekdays[i]).color(text_color));
                });
            }
        });

        ui.separator();

        // Calendar days
        let days = generate_calendar_days(year, month);
        let mut day_iter = days.iter();

        for _week in 0..6 { // Max 6 weeks in a month view
            ui.columns(7, |columns| {
                for column in columns.iter_mut() {
                    if let Some(day) = day_iter.next() {
                        column.with_layout(Layout::top_down(Align::Center), |ui| {
                            let text_color = if day.is_current_month {
                                visuals.text_color()
                            } else {
                                Color32::DARK_GRAY
                            };
                            let mut text = RichText::new(format!("{}", day.date.day())).color(text_color);

                            let is_marked = marked_dates.contains(&day.date);

                            // Highlight today's date
                            if day.date == chrono::Local::now().date_naive() {
                                text = text.background_color(Color32::from_rgb(50, 50, 100)).color(Color32::WHITE);
                            } else if is_marked {
                                text = text.background_color(Color32::from_rgb(100, 50, 50)).color(Color32::WHITE);
                            }

                            let available_width = ui.available_width();
                            let cell_size = Vec2::new(available_width, 30.0);

                            let response = ui.scope(|ui| {
                                let mut style = ui.style().clone();
                                style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
                                style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
                                style.visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
                                style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, Color32::GRAY);
                                style.visuals.widgets.active.bg_fill = Color32::from_gray(64);
                                style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, Color32::GRAY);
                                ui.set_style(style);

                                let button = egui::Button::new(text).min_size(cell_size);
                                ui.add(button)
                            }).inner;

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
        }
    });
}