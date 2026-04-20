use gtk4::prelude::*;
use gtk4::{TextBuffer, TextSearchFlags};

use crate::navigation::scroll_to_offset;
use crate::viewer::{AppState, SearchMatch, Ui, update_status};

pub(crate) fn update_search_matches(ui: &Ui, state: &mut AppState) {
    let buffer = ui.text_view.buffer();
    clear_search_tags(&buffer);
    state.search_matches.clear();

    if state.search_query.is_empty() {
        ui.overlay_info.set_text("Type to search");
        update_status(ui, state, Some("Search cleared"));
        return;
    }

    let mut iter = buffer.start_iter();
    while let Some((start, end)) = iter.forward_search(
        &state.search_query,
        TextSearchFlags::CASE_INSENSITIVE | TextSearchFlags::TEXT_ONLY,
        None,
    ) {
        buffer.apply_tag_by_name("search_match", &start, &end);
        state.search_matches.push(SearchMatch {
            start: start.offset(),
            end: end.offset(),
        });
        iter = end;
    }

    if state.search_matches.is_empty() {
        state.current_match = None;
        ui.overlay_info.set_text("No matches");
        update_status(ui, state, Some("No matches"));
        return;
    }

    let current = state.current_match.unwrap_or(0).min(state.search_matches.len() - 1);
    state.current_match = Some(current);
    apply_current_search_match(ui, state);
}

pub(crate) fn search_next(ui: &Ui, state: &mut AppState, direction: isize) {
    if state.search_matches.is_empty() {
        if state.search_query.is_empty() {
            update_status(ui, state, Some("No active search"));
        } else {
            update_status(ui, state, Some("No matches"));
        }
        return;
    }

    let len = state.search_matches.len() as isize;
    let current = state.current_match.unwrap_or(0) as isize;
    let next = (current + direction).rem_euclid(len) as usize;
    state.current_match = Some(next);
    apply_current_search_match(ui, state);
}

fn apply_current_search_match(ui: &Ui, state: &mut AppState) {
    let buffer = ui.text_view.buffer();
    clear_search_tags(&buffer);

    for matched in &state.search_matches {
        let start = buffer.iter_at_offset(matched.start);
        let end = buffer.iter_at_offset(matched.end);
        buffer.apply_tag_by_name("search_match", &start, &end);
    }

    if let Some(current) = state.current_match {
        if let Some(matched) = state.search_matches.get(current) {
            let start = buffer.iter_at_offset(matched.start);
            let end = buffer.iter_at_offset(matched.end);
            buffer.apply_tag_by_name("search_current", &start, &end);
            buffer.place_cursor(&start);
            scroll_to_offset(ui, matched.start, 0.2);
            ui.overlay_info.set_text(&format!(
                "Match {} of {}",
                current + 1,
                state.search_matches.len()
            ));
            update_status(
                ui,
                state,
                Some(&format!("Match {} of {}", current + 1, state.search_matches.len())),
            );
        }
    }
}

fn clear_search_tags(buffer: &TextBuffer) {
    let (start, end) = buffer.bounds();
    buffer.remove_tag_by_name("search_match", &start, &end);
    buffer.remove_tag_by_name("search_current", &start, &end);
}
