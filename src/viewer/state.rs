use std::path::PathBuf;

use crate::render::{HeadingInfo, LinkInfo, RenderedDoc};
use crate::theme::AppTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OverlayMode {
    None,
    Search,
    LinkHints,
}

#[derive(Clone, Debug)]
pub(crate) struct SearchMatch {
    pub(crate) start: i32,
    pub(crate) end: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct VisibleLinkHint {
    pub(crate) link_index: usize,
    pub(crate) code: String,
}

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) file_path: PathBuf,
    pub(crate) pending_g: bool,
    pub(crate) zoom_level: f64,
    pub(crate) theme: AppTheme,
    pub(crate) overlay_mode: OverlayMode,
    pub(crate) search_query: String,
    pub(crate) search_matches: Vec<SearchMatch>,
    pub(crate) current_match: Option<usize>,
    pub(crate) link_hint_input: String,
    pub(crate) links: Vec<LinkInfo>,
    pub(crate) visible_link_hints: Vec<VisibleLinkHint>,
    pub(crate) headings: Vec<HeadingInfo>,
    pub(crate) base_rendered: RenderedDoc,
}

impl AppState {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            file_path: path,
            pending_g: false,
            zoom_level: 1.0,
            theme: AppTheme::EditorialNight,
            overlay_mode: OverlayMode::None,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: None,
            link_hint_input: String::new(),
            links: Vec::new(),
            visible_link_hints: Vec::new(),
            headings: Vec::new(),
            base_rendered: RenderedDoc::default(),
        }
    }
}
