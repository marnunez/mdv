use std::process::Command;

use gtk4::graphene;
use gtk4::prelude::*;
use gtk4::{Align, Label, TextWindowType};

use crate::render::LinkInfo;
use crate::viewer::{AppState, LinkHintWidget, Ui};

pub(crate) fn selected_link_url(state: &AppState) -> Option<String> {
    let needle = state.link_hint_input.trim().to_ascii_lowercase();
    let filtered = filtered_links(state);

    filtered
        .iter()
        .find(|link| !needle.is_empty() && link.code == needle)
        .copied()
        .or_else(|| if filtered.len() == 1 { filtered.first().copied() } else { None })
        .map(|link| link.url.clone())
}

pub(crate) fn link_hint_match_count(state: &AppState) -> usize {
    filtered_links(state).len()
}

pub(crate) fn link_hint_status_message(state: &AppState) -> String {
    let matches = link_hint_match_count(state);
    let input = state.link_hint_input.trim();

    if input.is_empty() {
        format!(
            "Link hints — {} link{} visible",
            matches,
            if matches == 1 { "" } else { "s" }
        )
    } else if matches == 0 {
        format!("Link hints — {}: no matches", input)
    } else {
        format!(
            "Link hints — {}: {} match{}",
            input,
            matches,
            if matches == 1 { "" } else { "es" }
        )
    }
}

pub(crate) fn sync_link_hints(ui: &Ui, state: &AppState) {
    ensure_hint_widgets(ui, state);

    let visible = ui.text_view.visible_rect();
    let widgets = ui.link_hint_widgets.borrow();
    for (link, widget) in state.links.iter().zip(widgets.iter()) {
        let is_match = link_matches(link, &state.link_hint_input);
        widget.label.remove_css_class("match");
        widget.label.remove_css_class("dim");
        widget.label
            .add_css_class(if is_match { "match" } else { "dim" });

        let buffer = ui.text_view.buffer();
        let anchor_offset = (link.end - 1).max(link.start);
        let iter = buffer.iter_at_offset(anchor_offset);
        let location = ui.text_view.iter_location(&iter);

        let in_vertical_view = location.y() + location.height() >= visible.y()
            && location.y() <= visible.y() + visible.height();
        if !in_vertical_view {
            widget.label.set_visible(false);
            continue;
        }

        let (window_x, window_y) = ui.text_view.buffer_to_window_coords(
            TextWindowType::Widget,
            location.x() + location.width(),
            location.y(),
        );

        let point = graphene::Point::new((window_x + 1) as f32, (window_y - 10) as f32);
        if let Some(point) = ui.text_view.compute_point(&ui.hint_layer, &point) {
            ui.hint_layer
                .move_(&widget.label, point.x() as f64, point.y() as f64);
            widget.label.set_visible(true);
        } else {
            widget.label.set_visible(false);
        }
    }

    ui.hint_layer.set_visible(true);
}

pub(crate) fn hide_link_hints(ui: &Ui) {
    let mut widgets = ui.link_hint_widgets.borrow_mut();
    for widget in widgets.drain(..) {
        ui.hint_layer.remove(&widget.label);
    }
    ui.hint_layer.set_visible(false);
}

pub(crate) fn open_link_at_cursor(ui: &Ui, state: &AppState) -> bool {
    let buffer = ui.text_view.buffer();
    let iter = buffer.iter_at_mark(&buffer.get_insert());
    let offset = iter.offset();

    if let Some(link) = state
        .links
        .iter()
        .find(|link| offset >= link.start && offset <= link.end)
    {
        open_external_link(&link.url);
        true
    } else {
        false
    }
}

pub(crate) fn open_external_link(url: &str) {
    let _ = Command::new("xdg-open").arg(url).spawn();
}

fn ensure_hint_widgets(ui: &Ui, state: &AppState) {
    let needs_rebuild = {
        let widgets = ui.link_hint_widgets.borrow();
        widgets.len() != state.links.len()
    };

    if !needs_rebuild {
        return;
    }

    hide_link_hints(ui);

    let mut widgets = ui.link_hint_widgets.borrow_mut();
    for link in &state.links {
        let label = Label::new(Some(&link.code));
        label.add_css_class("mdv-link-hint");
        label.set_can_target(false);
        label.set_focusable(false);
        label.set_halign(Align::Start);
        label.set_valign(Align::Start);
        ui.hint_layer.put(&label, 0.0, 0.0);
        widgets.push(LinkHintWidget { label });
    }
}

fn filtered_links<'a>(state: &'a AppState) -> Vec<&'a LinkInfo> {
    let needle = state.link_hint_input.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return state.links.iter().collect();
    }

    state
        .links
        .iter()
        .filter(|link| link_matches(link, &needle))
        .collect()
}

fn link_matches(link: &LinkInfo, needle: &str) -> bool {
    let needle = needle.trim().to_ascii_lowercase();
    needle.is_empty() || link.code.starts_with(&needle)
}
