use gtk4::prelude::*;

use crate::viewer::Ui;

const SCROLL_STEP: f64 = 80.0;

pub(crate) fn line_down(ui: &Ui) {
    scroll_by(ui, SCROLL_STEP);
}

pub(crate) fn line_up(ui: &Ui) {
    scroll_by(ui, -SCROLL_STEP);
}

pub(crate) fn half_page_down(ui: &Ui) {
    page_scroll(ui, 0.5);
}

pub(crate) fn half_page_up(ui: &Ui) {
    page_scroll(ui, -0.5);
}

pub(crate) fn page_down(ui: &Ui) {
    page_scroll(ui, 1.0);
}

pub(crate) fn page_up(ui: &Ui) {
    page_scroll(ui, -1.0);
}

pub(crate) fn scroll_by(ui: &Ui, delta: f64) {
    let adj = ui.scrolled.vadjustment();
    let max = (adj.upper() - adj.page_size()).max(adj.lower());
    let value = (adj.value() + delta).clamp(adj.lower(), max);
    adj.set_value(value);
}

pub(crate) fn page_scroll(ui: &Ui, pages: f64) {
    let adj = ui.scrolled.vadjustment();
    scroll_by(ui, adj.page_size() * pages);
}

pub(crate) fn scroll_to_top(ui: &Ui) {
    let adj = ui.scrolled.vadjustment();
    adj.set_value(adj.lower());
    scroll_to_offset(ui, 0, 0.0);
}

pub(crate) fn scroll_to_bottom(ui: &Ui) {
    let adj = ui.scrolled.vadjustment();
    let max = (adj.upper() - adj.page_size()).max(adj.lower());
    adj.set_value(max);

    let buffer = ui.text_view.buffer();
    let mut end = buffer.end_iter();
    ui.text_view.scroll_to_iter(&mut end, 0.0, false, 0.0, 1.0);
    buffer.place_cursor(&end);
}

pub(crate) fn scroll_to_offset(ui: &Ui, offset: i32, yalign: f64) {
    let buffer = ui.text_view.buffer();
    let mut iter = buffer.iter_at_offset(offset);
    buffer.place_cursor(&iter);
    ui.text_view.scroll_to_iter(&mut iter, 0.15, false, 0.0, yalign);
}
