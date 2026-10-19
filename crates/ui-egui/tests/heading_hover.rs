//! The row or column heading under the pointer is tinted, as in the desktop app.

use egui::{Event, Shape, pos2};
use gridcraft_engine::Session;
use gridcraft_ui_egui::{SheetApp, theme::Tokens};

fn tinted(h: &egui_kittest::Harness<'static, SheetApp>, color: egui::Color32) -> bool {
    h.output().shapes.iter().any(|s| matches!(&s.shape, Shape::Rect(r) if r.fill == color))
}

#[test]
fn the_heading_under_the_pointer_is_tinted() {
    let mut s = Session::new();
    s.new_workbook();
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        SheetApp::new(s, Default::default()),
    );
    h.run_steps(4);
    let hover = Tokens::light().header_hover;
    assert_ne!(hover, Tokens::light().header_bg);
    assert_ne!(Tokens::dark().header_hover, Tokens::dark().header_bg);
    assert!(!tinted(&h, hover), "nothing is tinted before the pointer arrives");
    let cells = h.state().grid.cells_rect.expect("grid laid out");
    // Over column C's heading, just above the cells.
    h.input_mut().events.push(Event::PointerMoved(pos2(cells.left() + 64.0 * 2.5, cells.top() - 8.0)));
    h.run_steps(2);
    assert!(tinted(&h, hover), "the hovered heading is tinted");
}
