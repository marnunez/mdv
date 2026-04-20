use std::process::Command;

use gtk4::graphene;
use gtk4::prelude::*;
use gtk4::{Align, Label, TextWindowType};

use crate::navigation::scroll_to_offset_aligned;
use crate::render::LinkInfo;
use crate::viewer::{AppState, LinkHintWidget, Ui, VisibleLinkHint};

const HINT_ALPHABET: &[u8] = b"asdfghjklqwertyuiopzxcvbnm";

pub(crate) fn selected_link_url(state: &AppState) -> Option<String> {
    let needle = state.link_hint_input.trim().to_ascii_lowercase();
    let filtered = filtered_visible_hints(state);

    filtered
        .iter()
        .find(|hint| !needle.is_empty() && hint.code == needle)
        .copied()
        .or_else(|| {
            if filtered.len() == 1 {
                filtered.first().copied()
            } else {
                None
            }
        })
        .map(|hint| state.links[hint.link_index].url.clone())
}

pub(crate) fn link_hint_match_count(state: &AppState) -> usize {
    filtered_visible_hints(state).len()
}

pub(crate) fn link_hint_status_message(state: &AppState) -> String {
    let matches = link_hint_match_count(state);
    let input = state.link_hint_input.trim();

    if input.is_empty() {
        format!(
            "Link hints — {} visible link{}",
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

pub(crate) fn sync_link_hints(ui: &Ui, state: &mut AppState) {
    state.visible_link_hints = collect_visible_link_hints(ui, &state.links);
    ensure_hint_widgets(ui, state.visible_link_hints.len());

    let widgets = ui.link_hint_widgets.borrow();
    for (hint, widget) in state.visible_link_hints.iter().zip(widgets.iter()) {
        let is_match = hint_matches(hint, &state.link_hint_input);
        widget.label.set_text(&hint.code);
        widget.label.remove_css_class("match");
        widget.label.remove_css_class("dim");
        widget
            .label
            .add_css_class(if is_match { "match" } else { "dim" });

        let link = &state.links[hint.link_index];
        if let Some((x, y)) = hint_position(ui, link) {
            ui.hint_layer.move_(&widget.label, x, y);
            widget.label.set_visible(true);
        } else {
            widget.label.set_visible(false);
        }
    }

    for widget in widgets.iter().skip(state.visible_link_hints.len()) {
        widget.label.set_visible(false);
    }

    ui.hint_layer.set_visible(!state.visible_link_hints.is_empty());
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
        activate_link(ui, state, &link.url)
    } else {
        false
    }
}

pub(crate) fn activate_link(ui: &Ui, state: &AppState, url: &str) -> bool {
    if let Some(anchor) = internal_anchor(url) {
        return scroll_to_heading_anchor(ui, state, anchor);
    }

    open_external_link(url);
    true
}

pub(crate) fn open_external_link(url: &str) {
    let _ = Command::new("xdg-open").arg(url).spawn();
}

fn scroll_to_heading_anchor(ui: &Ui, state: &AppState, anchor: &str) -> bool {
    if let Some(heading) = state.headings.iter().find(|heading| heading.anchor == anchor) {
        scroll_to_offset_aligned(ui, heading.offset, 0.02);
        true
    } else {
        false
    }
}

fn internal_anchor(url: &str) -> Option<&str> {
    url.strip_prefix('#').filter(|anchor| !anchor.is_empty())
}

fn collect_visible_link_hints(ui: &Ui, links: &[LinkInfo]) -> Vec<VisibleLinkHint> {
    links
        .iter()
        .enumerate()
        .filter_map(|(index, link)| {
            if hint_position(ui, link).is_some() {
                Some(VisibleLinkHint {
                    link_index: index,
                    code: hint_code(index_for_visible_link(links, ui, index)),
                })
            } else {
                None
            }
        })
        .collect()
}

fn index_for_visible_link(links: &[LinkInfo], ui: &Ui, target_index: usize) -> usize {
    let mut visible_index = 0;
    for (index, link) in links.iter().enumerate() {
        if hint_position(ui, link).is_some() {
            if index == target_index {
                return visible_index;
            }
            visible_index += 1;
        }
    }
    0
}

fn hint_position(ui: &Ui, link: &LinkInfo) -> Option<(f64, f64)> {
    let buffer = ui.text_view.buffer();
    let anchor_offset = (link.end - 1).max(link.start);
    let iter = buffer.iter_at_offset(anchor_offset);
    let location = ui.text_view.iter_location(&iter);

    let (window_x, window_y) = ui.text_view.buffer_to_window_coords(
        TextWindowType::Widget,
        location.x() + location.width(),
        location.y(),
    );

    let point = graphene::Point::new((window_x + 1) as f32, (window_y - 10) as f32);
    let mapped = ui.text_view.compute_point(&ui.hint_layer, &point)?;

    let top = mapped.y() as f64;
    let bottom = top + f64::from(location.height());
    let view_height = f64::from(ui.hint_layer.allocated_height().max(ui.scrolled.allocated_height()));

    if bottom < 0.0 || top > view_height {
        return None;
    }

    Some((mapped.x() as f64, top))
}

fn ensure_hint_widgets(ui: &Ui, visible_count: usize) {
    let mut widgets = ui.link_hint_widgets.borrow_mut();
    while widgets.len() < visible_count {
        let label = Label::new(None);
        label.add_css_class("mdv-link-hint");
        label.set_can_target(false);
        label.set_focusable(false);
        label.set_halign(Align::Start);
        label.set_valign(Align::Start);
        ui.hint_layer.put(&label, 0.0, 0.0);
        widgets.push(LinkHintWidget { label });
    }
}

fn filtered_visible_hints<'a>(state: &'a AppState) -> Vec<&'a VisibleLinkHint> {
    let needle = state.link_hint_input.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return state.visible_link_hints.iter().collect();
    }

    state
        .visible_link_hints
        .iter()
        .filter(|hint| hint_matches(hint, &needle))
        .collect()
}

fn hint_matches(hint: &VisibleLinkHint, needle: &str) -> bool {
    let needle = needle.trim().to_ascii_lowercase();
    needle.is_empty() || hint.code.starts_with(&needle)
}

fn hint_code(mut index: usize) -> String {
    let base = HINT_ALPHABET.len();
    let mut chars = Vec::new();
    index += 1;

    while index > 0 {
        let rem = (index - 1) % base;
        chars.push(HINT_ALPHABET[rem] as char);
        index = (index - 1) / base;
    }

    chars.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn link(code: &str, url: &str) -> LinkInfo {
        LinkInfo {
            code: code.to_string(),
            label: code.to_string(),
            url: url.to_string(),
            start: 0,
            end: 1,
        }
    }

    fn visible_hint(link_index: usize, code: &str) -> VisibleLinkHint {
        VisibleLinkHint {
            link_index,
            code: code.to_string(),
        }
    }

    fn state_with_visible_hints(input: &str, links: Vec<LinkInfo>, hints: Vec<VisibleLinkHint>) -> AppState {
        let mut state = AppState::new(PathBuf::from("test.md"));
        state.link_hint_input = input.to_string();
        state.links = links;
        state.visible_link_hints = hints;
        state
    }

    #[test]
    fn hint_matches_prefixes_case_insensitively() {
        let hint = visible_hint(0, "abc");

        assert!(hint_matches(&hint, "a"));
        assert!(hint_matches(&hint, " AB "));
        assert!(hint_matches(&hint, ""));
        assert!(!hint_matches(&hint, "bc"));
    }

    #[test]
    fn filtered_hints_return_all_visible_hints_when_input_is_blank() {
        let state = state_with_visible_hints(
            "  ",
            vec![
                link("aa", "https://example.com/aa"),
                link("ab", "https://example.com/ab"),
            ],
            vec![visible_hint(0, "a"), visible_hint(1, "s")],
        );

        let filtered = filtered_visible_hints(&state);
        let codes = filtered
            .iter()
            .map(|hint| hint.code.as_str())
            .collect::<Vec<_>>();

        assert_eq!(codes, vec!["a", "s"]);
        assert_eq!(link_hint_match_count(&state), 2);
    }

    #[test]
    fn selected_link_url_prefers_exact_visible_hint_match() {
        let state = state_with_visible_hints(
            "a",
            vec![
                link("ignored-a", "https://example.com/exact"),
                link("ignored-aa", "https://example.com/prefix"),
            ],
            vec![visible_hint(0, "a"), visible_hint(1, "aa")],
        );

        assert_eq!(
            selected_link_url(&state),
            Some("https://example.com/exact".to_string())
        );
    }

    #[test]
    fn selected_link_url_auto_selects_when_only_one_visible_match_remains() {
        let state = state_with_visible_hints(
            "ab",
            vec![
                link("ignored-aa", "https://example.com/aa"),
                link("ignored-ab", "https://example.com/ab"),
                link("ignored-ba", "https://example.com/ba"),
            ],
            vec![
                visible_hint(0, "aa"),
                visible_hint(1, "ab"),
                visible_hint(2, "ba"),
            ],
        );

        assert_eq!(
            selected_link_url(&state),
            Some("https://example.com/ab".to_string())
        );
    }

    #[test]
    fn selected_link_url_stays_none_while_visible_matches_are_ambiguous() {
        let state = state_with_visible_hints(
            "a",
            vec![
                link("ignored-aa", "https://example.com/aa"),
                link("ignored-ab", "https://example.com/ab"),
            ],
            vec![visible_hint(0, "aa"), visible_hint(1, "ab")],
        );

        assert_eq!(selected_link_url(&state), None);
    }

    #[test]
    fn link_hint_status_message_reports_visible_links_and_matches() {
        let links = vec![
            link("ignored-aa", "https://example.com/aa"),
            link("ignored-ab", "https://example.com/ab"),
        ];
        let hints = vec![visible_hint(0, "a"), visible_hint(1, "s")];

        let blank = state_with_visible_hints("", links.clone(), hints.clone());
        assert_eq!(
            link_hint_status_message(&blank),
            "Link hints — 2 visible links"
        );

        let one_match = state_with_visible_hints("a", links.clone(), hints.clone());
        assert_eq!(link_hint_status_message(&one_match), "Link hints — a: 1 match");

        let no_matches = state_with_visible_hints("zz", links, hints);
        assert_eq!(
            link_hint_status_message(&no_matches),
            "Link hints — zz: no matches"
        );
    }

    #[test]
    fn hint_codes_are_assigned_for_visible_order() {
        assert_eq!(hint_code(0), "a");
        assert_eq!(hint_code(1), "s");
        assert_eq!(hint_code(25), "m");
        assert_eq!(hint_code(26), "aa");
    }
}
