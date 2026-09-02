use crate::app::DisplaySettings;
use crate::logic::calendar_logic::{CalendarDay, week_number_for_row};
use chrono::{Datelike, NaiveDate, Weekday};
use egui::{Align, Button, Color32, Layout, RichText, Ui, Vec2, Visuals};
use std::collections::HashSet;

const WEEK_NUMBER_COLUMN_WIDTH: f32 = 24.0;
const DAY_CELL_HEIGHT: f32 = 32.0;
const CALENDAR_COLUMN_SPACING: f32 = 2.0;
const CALENDAR_HORIZONTAL_PADDING: f32 = 2.0;

pub fn calendar_view(
    ui: &mut Ui,
    year: i32,
    month: u32,
    days: &[CalendarDay],
    marked_dates: &mut HashSet<NaiveDate>,
    visuals: &Visuals,
    settings: DisplaySettings,
) {
    ui.vertical(|ui| {
        ui.add_space(16.0);
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.label(RichText::new(format!("{year}/{month}")).size(20.0).strong());
        });
        ui.add_space(14.0);

        let day_cell_width = day_cell_width(ui, settings.show_week_numbers);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = CALENDAR_COLUMN_SPACING;
            ui.add_space(CALENDAR_HORIZONTAL_PADDING);
            if settings.show_week_numbers {
                week_number_cell(ui, RichText::new("W").color(Color32::GRAY), 18.0);
            }
            for weekday in settings.week_start.weekdays() {
                ui.allocate_ui_with_layout(
                    Vec2::new(day_cell_width, 18.0),
                    Layout::top_down(Align::Center),
                    |ui| {
                        ui.label(
                            RichText::new(settings.weekday_language.label(weekday))
                                .color(weekday_color(weekday, visuals)),
                        );
                    },
                );
            }
        });

        ui.separator();
        ui.add_space(6.0);

        for week in days.chunks(7) {
            debug_assert_eq!(week.len(), 7);
            let week_number_text = settings.show_week_numbers.then(|| {
                let week_number = week_number_for_row(week, settings.week_number_rule)
                    .expect("a complete calendar row contains every weekday");
                RichText::new(format!("{week_number:02}")).color(Color32::GRAY)
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = CALENDAR_COLUMN_SPACING;
                ui.add_space(CALENDAR_HORIZONTAL_PADDING);
                if let Some(text) = week_number_text {
                    week_number_cell(ui, text, DAY_CELL_HEIGHT);
                }
                for day in week {
                    day_cell(ui, day, marked_dates, visuals, day_cell_width);
                }
            });
            ui.add_space(3.0);
        }
    });
}

fn day_cell_width(ui: &Ui, show_week_numbers: bool) -> f32 {
    day_cell_width_for_available_width(ui.available_width(), show_week_numbers)
}

fn day_cell_width_for_available_width(available_width: f32, show_week_numbers: bool) -> f32 {
    let fixed_width = if show_week_numbers {
        WEEK_NUMBER_COLUMN_WIDTH
    } else {
        0.0
    };
    let gap_count = if show_week_numbers { 7.0 } else { 6.0 };
    ((available_width
        - fixed_width
        - gap_count * CALENDAR_COLUMN_SPACING
        - CALENDAR_HORIZONTAL_PADDING * 2.0)
        / 7.0)
        .max(1.0)
}

fn week_number_cell(ui: &mut Ui, text: RichText, height: f32) {
    ui.allocate_ui_with_layout(
        Vec2::new(WEEK_NUMBER_COLUMN_WIDTH, height),
        Layout::top_down(Align::Center),
        |ui| {
            ui.label(text);
        },
    );
}

fn day_cell(
    ui: &mut Ui,
    day: &CalendarDay,
    marked_dates: &mut HashSet<NaiveDate>,
    visuals: &Visuals,
    width: f32,
) {
    let text_color = if day.is_current_month {
        weekday_color(day.date.weekday(), visuals)
    } else {
        Color32::DARK_GRAY
    };
    let mut text = RichText::new(day.date.day().to_string()).color(text_color);
    let is_marked = marked_dates.contains(&day.date);

    if day.date == chrono::Local::now().date_naive() {
        text = text
            .background_color(visuals.selection.bg_fill)
            .color(Color32::WHITE);
    } else if is_marked {
        text = text
            .background_color(visuals.warn_fg_color)
            .color(Color32::WHITE);
    }

    let response = ui
        .add_sized(
            Vec2::new(width, DAY_CELL_HEIGHT),
            Button::new(text).rounding(4.0).frame(true),
        )
        .on_hover_cursor(egui::CursorIcon::PointingHand);

    if response.clicked() {
        toggle_marked_date(marked_dates, day.date);
    }
}

fn toggle_marked_date(marked_dates: &mut HashSet<NaiveDate>, date: NaiveDate) {
    if !marked_dates.remove(&date) {
        marked_dates.insert(date);
    }
}

fn weekday_color(weekday: Weekday, visuals: &Visuals) -> Color32 {
    match weekday {
        Weekday::Sun => Color32::from_rgb(220, 50, 50),
        Weekday::Sat => Color32::from_rgb(50, 100, 200),
        _ => visuals.text_color(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekend_colors_follow_the_weekday_not_the_column() {
        let visuals = Visuals::dark();

        assert_eq!(
            weekday_color(Weekday::Sun, &visuals),
            Color32::from_rgb(220, 50, 50)
        );
        assert_eq!(
            weekday_color(Weekday::Sat, &visuals),
            Color32::from_rgb(50, 100, 200)
        );
        assert_eq!(weekday_color(Weekday::Mon, &visuals), visuals.text_color());
    }

    #[test]
    fn adjacent_month_dates_can_be_marked_and_unmarked() {
        let adjacent_month_date = NaiveDate::from_ymd_opt(2026, 9, 1).unwrap();
        let mut marked_dates = HashSet::new();

        toggle_marked_date(&mut marked_dates, adjacent_month_date);
        assert!(marked_dates.contains(&adjacent_month_date));

        toggle_marked_date(&mut marked_dates, adjacent_month_date);
        assert!(!marked_dates.contains(&adjacent_month_date));
    }

    #[test]
    fn week_number_column_keeps_day_cells_balanced_in_single_month_view() {
        let without_week_numbers = day_cell_width_for_available_width(280.0, false);
        let with_week_numbers = day_cell_width_for_available_width(306.0, true);

        assert_eq!(without_week_numbers, with_week_numbers);
        assert!(without_week_numbers > 37.0);
    }
}
