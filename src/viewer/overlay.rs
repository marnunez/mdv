use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;

use crate::links::{
    activate_link, hide_link_hints, link_hint_match_count, link_hint_status_message,
    selected_link_url, sync_link_hints,
};
use crate::search::update_search_matches;

use super::document::update_status;
use super::{AppState, OverlayMode, Ui};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchEntryChange {
    Updated,
    IgnoredNotSearching,
    IgnoredWhileBusy,
}

fn apply_search_entry_change(state: &RefCell<AppState>, query: &str) -> SearchEntryChange {
    let Ok(mut state) = state.try_borrow_mut() else {
        return SearchEntryChange::IgnoredWhileBusy;
    };

    if state.overlay_mode != OverlayMode::Search {
        return SearchEntryChange::IgnoredNotSearching;
    }

    state.search_query = query.to_string();
    state.current_match = Some(0);
    SearchEntryChange::Updated
}

pub(crate) fn wire_overlay_events(ui: &Ui, state: &Rc<RefCell<AppState>>) {
    let ui_changed = ui.clone();
    let state_changed = state.clone();
    ui.overlay_entry.connect_changed(move |entry| {
        let query = entry.text();
        if apply_search_entry_change(state_changed.as_ref(), query.as_ref())
            == SearchEntryChange::Updated
        {
            let mut state = state_changed.borrow_mut();
            update_search_matches(&ui_changed, &mut state);
        }
    });

    let ui_activate = ui.clone();
    let state_activate = state.clone();
    ui.overlay_entry.connect_activate(move |_| {
        let mut state = state_activate.borrow_mut();
        if state.overlay_mode == OverlayMode::Search {
            hide_overlay(&ui_activate, &mut state);
            ui_activate.text_view.grab_focus();
        }
    });
}

pub(crate) fn handle_overlay_key(
    ui: &Ui,
    state: &mut AppState,
    keyval: gdk::Key,
) -> glib::Propagation {
    match state.overlay_mode {
        OverlayMode::Search => match keyval {
            gdk::Key::Escape => {
                hide_overlay(ui, state);
                ui.text_view.grab_focus();
                glib::Propagation::Stop
            }
            gdk::Key::Return | gdk::Key::KP_Enter => {
                hide_overlay(ui, state);
                ui.text_view.grab_focus();
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        },
        OverlayMode::LinkHints => handle_link_hint_key(ui, state, keyval),
        OverlayMode::None => glib::Propagation::Proceed,
    }
}

fn handle_link_hint_key(ui: &Ui, state: &mut AppState, keyval: gdk::Key) -> glib::Propagation {
    match keyval {
        gdk::Key::Escape => {
            hide_overlay(ui, state);
            ui.text_view.grab_focus();
            glib::Propagation::Stop
        }
        gdk::Key::BackSpace => {
            state.link_hint_input.pop();
            sync_link_hints(ui, state);
            update_status(ui, state, Some(&link_hint_status_message(state)));
            glib::Propagation::Stop
        }
        gdk::Key::Return | gdk::Key::KP_Enter => {
            if let Some(url) = selected_link_url(state) {
                if activate_link(ui, state, &url) {
                    hide_overlay(ui, state);
                    update_status(ui, state, Some(&format!("Opened {}", url)));
                    ui.text_view.grab_focus();
                } else {
                    update_status(ui, state, Some(&format!("Missing anchor {}", url)));
                }
            } else {
                update_status(ui, state, Some("Link hints — type until one match remains"));
            }
            glib::Propagation::Stop
        }
        _ => {
            if let Some(ch) = keyval.to_unicode() {
                if ch.is_ascii_alphanumeric() {
                    state.link_hint_input.push(ch.to_ascii_lowercase());
                    sync_link_hints(ui, state);

                    if let Some(url) = selected_link_url(state) {
                        if activate_link(ui, state, &url) {
                            hide_overlay(ui, state);
                            update_status(ui, state, Some(&format!("Opened {}", url)));
                            ui.text_view.grab_focus();
                        } else {
                            update_status(ui, state, Some(&format!("Missing anchor {}", url)));
                        }
                    } else if link_hint_match_count(state) == 0 {
                        update_status(ui, state, Some("Link hints — no matches"));
                    } else {
                        update_status(ui, state, Some(&link_hint_status_message(state)));
                    }

                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Stop
                }
            } else {
                glib::Propagation::Stop
            }
        }
    }
}

pub(crate) fn show_search_overlay(ui: &Ui, state: &mut AppState) {
    hide_link_hints(ui);
    state.overlay_mode = OverlayMode::Search;
    ui.overlay_title.set_text("Search");
    ui.overlay_entry.set_text(&state.search_query);
    ui.overlay_entry.set_position(-1);
    ui.overlay_entry.set_placeholder_text(None);
    ui.overlay_revealer.set_reveal_child(true);
    ui.overlay_entry.grab_focus();
    update_search_matches(ui, state);
}

pub(crate) fn show_link_hint_overlay(ui: &Ui, state: &mut AppState) {
    state.overlay_mode = OverlayMode::LinkHints;
    state.link_hint_input.clear();
    ui.overlay_revealer.set_reveal_child(false);
    ui.overlay_entry.set_text("");
    ui.overlay_entry.set_placeholder_text(None);
    ui.overlay_info.set_text("");
    sync_link_hints(ui, state);
    update_status(ui, state, Some(&link_hint_status_message(state)));
    ui.text_view.grab_focus();
}

pub(crate) fn hide_overlay(ui: &Ui, state: &mut AppState) {
    state.overlay_mode = OverlayMode::None;
    state.link_hint_input.clear();
    ui.overlay_revealer.set_reveal_child(false);
    ui.overlay_entry.set_text("");
    ui.overlay_entry.set_placeholder_text(None);
    ui.overlay_info.set_text("");
    hide_link_hints(ui);
    update_status(ui, state, None);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn search_state() -> RefCell<AppState> {
        let mut state = AppState::new(PathBuf::from("test.md"));
        state.overlay_mode = OverlayMode::Search;
        RefCell::new(state)
    }

    #[test]
    fn search_entry_change_updates_query_and_resets_match_index() {
        let state = search_state();
        {
            let mut state = state.borrow_mut();
            state.search_query = "before".to_string();
            state.current_match = Some(3);
        }

        assert_eq!(
            apply_search_entry_change(&state, "after"),
            SearchEntryChange::Updated
        );

        let state = state.borrow();
        assert_eq!(state.search_query, "after");
        assert_eq!(state.current_match, Some(0));
    }

    #[test]
    fn search_entry_change_ignores_non_search_modes() {
        let state = RefCell::new(AppState::new(PathBuf::from("test.md")));

        assert_eq!(
            apply_search_entry_change(&state, "needle"),
            SearchEntryChange::IgnoredNotSearching
        );

        let state = state.borrow();
        assert!(state.search_query.is_empty());
        assert_eq!(state.current_match, None);
    }

    #[test]
    fn search_entry_change_ignores_reentrant_updates_while_overlay_is_hiding() {
        let state = search_state();
        {
            let mut state = state.borrow_mut();
            state.search_query = "needle".to_string();
            state.current_match = Some(2);
        }

        let active_borrow = state.borrow_mut();
        assert_eq!(
            apply_search_entry_change(&state, ""),
            SearchEntryChange::IgnoredWhileBusy
        );
        drop(active_borrow);

        let state = state.borrow();
        assert_eq!(state.search_query, "needle");
        assert_eq!(state.current_match, Some(2));
    }
}
