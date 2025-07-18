use egui::{Align, Color32, Layout, RichText, Ui};
use chrono::{Datelike, NaiveDate, Weekday};
use crate::logic::calendar_logic::{generate_calendar_days, CalendarDay};

pub fn calendar_view(ui: &mut Ui, year: i32, month: u32) {
    ui.vertical(|ui| {
        ui.add_space(10.0);
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading(format!("{}/{}", year, month));
        });
        ui.add_space(10.0);

        // Weekday headers
        ui.columns(7, |columns| {
            let weekdays = ["日", "月", "火", "水", "木", "金", "土"];
            for (i, column) in columns.iter_mut().enumerate() {
                column.with_layout(Layout::top_down(Align::Center), |ui| {
                    let text_color = match i {
                        0 => Color32::RED,   // Sunday
                        6 => Color32::BLUE,  // Saturday
                        _ => Color32::WHITE, // Weekdays
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
                                Color32::WHITE
                            } else {
                                Color32::DARK_GRAY
                            };
                            let mut text = RichText::new(format!("{}", day.date.day())).color(text_color);

                            // Highlight today's date
                            if day.date == chrono::Local::now().date_naive() {
                                text = text.background_color(Color32::from_rgb(50, 50, 100));
                            }

                            ui.label(text);
                        });
                    }
                }
            });
        }
    });
}