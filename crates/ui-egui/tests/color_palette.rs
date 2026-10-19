//! The colour palette: theme, tint and standard swatches, the no-colour row, and the Colors
//! window opened by More Colors…, whose answer reaches the caller through `take_result`.

use egui_kittest::kittest::Queryable;
use gridcraft_ui_egui::{color_picker, theme};

#[derive(Default)]
struct State {
    fonts: bool,
    picked: Option<String>,
    more: Option<String>,
}

const COLORS: [u32; 12] = [0xFFFFFF, 0x000000, 0xE7E6E6, 0x44546A, 0x4472C4, 0xED7D31, 0xA5A5A5, 0xFFC000, 0x5B9BD5, 0x70AD47, 0x0563C1, 0x954F72];

fn harness(none_label: &'static str) -> egui_kittest::Harness<'static, State> {
    egui_kittest::Harness::builder().with_size(egui::vec2(900.0, 700.0)).build_ui_state(
        move |ui, s: &mut State| {
            let ctx = ui.ctx().clone();
            if !s.fonts {
                // The interface fonts take effect from the next frame.
                ctx.set_fonts(theme::font_definitions());
                s.fonts = true;
                return;
            }
            if let Some(c) = color_picker::palette(ui, &COLORS, none_label, "test") {
                s.picked = Some(c);
            }
            color_picker::window(&ctx);
            if let Some(c) = color_picker::take_result(&ctx, "test") {
                s.more = Some(c);
            }
        },
        State::default(),
    )
}

#[test]
fn swatches_answer_their_colour() {
    let mut h = harness("No Fill");
    h.run_steps(2);
    h.get_by_label("Accent 1").click();
    h.run_steps(2);
    assert_eq!(h.state().picked.as_deref(), Some("#4472C4"));
    h.get_by_label("Red").click();
    h.run_steps(2);
    assert_eq!(h.state().picked.as_deref(), Some("#FF0000"));
    // Tints are lighter versions of the theme colour.
    h.get_by_label("Accent 1, Lighter 80%").click();
    h.run_steps(2);
    let tint = h.state().picked.clone().unwrap_or_default();
    assert!(tint.starts_with('#') && tint != "#4472C4", "{tint}");
    h.get_by_label("No Fill").click();
    h.run_steps(2);
    assert_eq!(h.state().picked.as_deref(), Some("none"));
}

#[test]
fn more_colors_answers_through_take_result_and_cancel_answers_nothing() {
    let mut h = harness("Automatic");
    h.run_steps(2);
    h.get_by_label("Automatic").click();
    h.run_steps(2);
    assert_eq!(h.state().picked.as_deref(), Some("none"));
    for ok in [false, true] {
        h.get_by_label("More Colors…").click();
        h.run_steps(3);
        h.get_by_label(if ok { "OK" } else { "Cancel" }).click();
        h.run_steps(3);
        assert_eq!(h.state().more.is_some(), ok, "OK answers a colour, Cancel nothing");
    }
    assert!(h.state().more.as_deref().is_some_and(|c| c.starts_with('#') && c.len() == 7));
}
