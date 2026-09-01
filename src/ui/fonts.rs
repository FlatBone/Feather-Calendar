use egui::{FontData, FontDefinitions, FontFamily};

const JAPANESE_WEEKDAY_FONT_NAME: &str = "feather_calendar_weekdays";

pub fn install_japanese_weekday_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        JAPANESE_WEEKDAY_FONT_NAME.to_owned(),
        FontData::from_static(include_bytes!(
            "../../assets/fonts/FeatherCalendarWeekdays.otf"
        )),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .expect("the default proportional font family must exist")
        .push(JAPANESE_WEEKDAY_FONT_NAME.to_owned());
    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{FontId, RawInput};

    #[test]
    fn embedded_font_contains_all_japanese_weekday_glyphs() {
        let ctx = egui::Context::default();
        install_japanese_weekday_font(&ctx);

        let _ = ctx.run(RawInput::default(), |ctx| {
            ctx.fonts(|fonts| {
                assert!(fonts.has_glyphs(&FontId::proportional(14.0), "日月火水木金土"));
            });
        });
    }
}
