use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use gtk4::glib;
use gtk4::prelude::*;

use crate::links::{hide_link_hints, sync_link_hints};
use crate::navigation::scroll_to_top;
use crate::render::{build_buffer, render_markdown, RenderedDoc};
use crate::search::update_search_matches;
use crate::theme::{apply_css, palette};

use super::{AppState, OverlayMode, Ui};

pub(crate) fn apply_initial_theme(ui: &Ui, state: &Rc<RefCell<AppState>>) {
    let state = state.borrow();
    apply_css(&ui.css_provider, state.zoom_level, state.theme);
}

pub(crate) fn update_status(ui: &Ui, state: &AppState, message: Option<&str>) {
    let name = state
        .file_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| state.file_path.display().to_string());

    let mut status = format!(
        "{} — {} heading{} — {} link{} — {:.0}% — {}",
        name,
        state.headings.len(),
        if state.headings.len() == 1 { "" } else { "s" },
        state.links.len(),
        if state.links.len() == 1 { "" } else { "s" },
        state.zoom_level * 100.0,
        state.theme.label(),
    );

    if let Some(message) = message {
        status.push_str(&format!(" — {}", message));
    }

    ui.status_label.set_text(&status);
}

pub(crate) fn reload_document(ui: &Ui, state: &Rc<RefCell<AppState>>, preserve_scroll: bool) {
    let borrowed = state.borrow();
    let path = borrowed.file_path.clone();
    let theme = borrowed.theme;
    drop(borrowed);

    let (rendered, message, clear_search) = match fs::read_to_string(&path) {
        Ok(markdown) => (render_markdown(&markdown, theme), Some("Reloaded"), false),
        Err(error) => (
            RenderedDoc {
                text: format!("Failed to read {}\n\n{}", path.display(), error),
                ..RenderedDoc::default()
            },
            Some("Read error"),
            true,
        ),
    };

    let mut state = state.borrow_mut();
    state.base_rendered = rendered;
    if clear_search {
        state.search_query.clear();
    }
    render_current_view(ui, &mut state, message, preserve_scroll);
    ui.window
        .set_title(Some(&format!("mdv — {}", path.display())));
}

pub(crate) fn render_current_view(
    ui: &Ui,
    state: &mut AppState,
    message: Option<&str>,
    preserve_scroll: bool,
) {
    let adj = ui.scrolled.vadjustment();
    let previous_scroll = adj.value();
    let previous_max = (adj.upper() - adj.page_size()).max(adj.lower());
    let previous_fraction = if previous_max > adj.lower() {
        (previous_scroll - adj.lower()) / (previous_max - adj.lower())
    } else {
        0.0
    };

    let rendered = state.base_rendered.clone();
    let buffer = build_buffer(&rendered, palette(state.theme));
    ui.text_view.set_buffer(Some(&buffer));

    state.links = rendered.links;
    state.visible_link_hints.clear();
    state.headings = rendered.headings;
    state.search_matches.clear();
    state.current_match = None;

    apply_css(&ui.css_provider, state.zoom_level, state.theme);
    if !state.search_query.is_empty() && state.overlay_mode != OverlayMode::LinkHints {
        update_search_matches(ui, state);
    } else {
        update_status(ui, state, message);
    }

    if state.overlay_mode == OverlayMode::LinkHints {
        sync_link_hints(ui, state);
    } else {
        hide_link_hints(ui);
    }

    if preserve_scroll {
        let ui = ui.clone();
        glib::idle_add_local_once(move || {
            let adj = ui.scrolled.vadjustment();
            let max = (adj.upper() - adj.page_size()).max(adj.lower());
            let target = if max > adj.lower() {
                adj.lower() + (max - adj.lower()) * previous_fraction
            } else {
                previous_scroll
            };
            adj.set_value(target.clamp(adj.lower(), max));
        });
    } else {
        scroll_to_top(ui);
    }
}
