use gtk4::graphene;
use gtk4::prelude::*;
use gtk4::TextWindowType;

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
    scroll_to_offset_within_page(ui, offset, yalign);
}

pub(crate) fn scroll_to_offset_aligned(ui: &Ui, offset: i32, yalign: f64) {
    scroll_to_offset_within_page(ui, offset, yalign);
}

fn scroll_to_offset_within_page(ui: &Ui, offset: i32, yalign: f64) {
    let buffer = ui.text_view.buffer();
    let iter = buffer.iter_at_offset(offset);
    buffer.place_cursor(&iter);

    let Some(current_y) = offset_y_in_view(ui, offset) else {
        return;
    };

    let adj = ui.scrolled.vadjustment();
    let max = (adj.upper() - adj.page_size()).max(adj.lower());
    let target = (adj.value() + current_y - (adj.page_size() * yalign)).clamp(adj.lower(), max);
    adj.set_value(target);
}

fn offset_y_in_view(ui: &Ui, offset: i32) -> Option<f64> {
    let buffer = ui.text_view.buffer();
    let iter = buffer.iter_at_offset(offset);
    let location = ui.text_view.iter_location(&iter);
    let (window_x, window_y) = ui
        .text_view
        .buffer_to_window_coords(TextWindowType::Widget, location.x(), location.y());
    let point = graphene::Point::new(window_x as f32, window_y as f32);
    ui.text_view
        .compute_point(&ui.hint_layer, &point)
        .map(|point| point.y() as f64)
}
