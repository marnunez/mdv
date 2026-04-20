use gtk4::{CssProvider, Settings};

const BASE_FONT_SIZE: f64 = 16.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppTheme {
    EditorialNight,
    EditorialDay,
}

#[derive(Clone, Copy, Debug)]
pub struct ThemePalette {
    pub window_bg: &'static str,
    pub body_fg: &'static str,
    pub topbar_bg: &'static str,
    pub topbar_fg: &'static str,
    pub border: &'static str,
    pub overlay_title: &'static str,
    pub overlay_info: &'static str,
    pub entry_bg: &'static str,
    pub entry_fg: &'static str,
    pub entry_border: &'static str,
    pub hint_match: &'static str,
    pub hint_dim: &'static str,
    pub status_bg: &'static str,
    pub status_fg: &'static str,
    pub scrollbar: &'static str,
    pub heading_1: &'static str,
    pub heading_2: &'static str,
    pub heading_3: &'static str,
    pub heading_4: &'static str,
    pub heading_5: &'static str,
    pub heading_6: &'static str,
    pub inline_code_bg: &'static str,
    pub inline_code_fg: &'static str,
    pub code_block_bg: &'static str,
    pub code_block_fg: &'static str,
    pub code_language_fg: &'static str,
    pub link_fg: &'static str,
    pub link_hint_target_match: &'static str,
    pub link_hint_target_dim: &'static str,
    pub quote_fg: &'static str,
    pub muted_fg: &'static str,
    pub search_match_bg: &'static str,
    pub search_current_bg: &'static str,
    pub search_current_fg: &'static str,
    pub selection_bg: &'static str,
    pub selection_fg: &'static str,
}

impl AppTheme {
    pub fn toggle(self) -> Self {
        match self {
            Self::EditorialNight => Self::EditorialDay,
            Self::EditorialDay => Self::EditorialNight,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::EditorialNight => "Night",
            Self::EditorialDay => "Day",
        }
    }
}

pub fn palette(theme: AppTheme) -> ThemePalette {
    match theme {
        AppTheme::EditorialNight => ThemePalette {
            window_bg: "#19181d",
            body_fg: "#e6dfd1",
            topbar_bg: "#131219",
            topbar_fg: "#d8d2c7",
            border: "#302d36",
            overlay_title: "#b9a8ff",
            overlay_info: "#9e9ab0",
            entry_bg: "#23212a",
            entry_fg: "#ebe4d8",
            entry_border: "#403b48",
            hint_match: "#b9a8ff",
            hint_dim: "#646072",
            status_bg: "#131219",
            status_fg: "#9f9aad",
            scrollbar: "#4a4455",
            heading_1: "#b6cbff",
            heading_2: "#96b8ff",
            heading_3: "#c9d8ff",
            heading_4: "#d4c4ff",
            heading_5: "#aeb4cc",
            heading_6: "#9aa3bd",
            inline_code_bg: "#262431",
            inline_code_fg: "#f2cf9f",
            code_block_bg: "#14151d",
            code_block_fg: "#d9deea",
            code_language_fg: "#8ea3d6",
            link_fg: "#8fb3ff",
            link_hint_target_match: "#f0d8a8",
            link_hint_target_dim: "#6f6a7a",
            quote_fg: "#9aa3c7",
            muted_fg: "#6f6a7a",
            search_match_bg: "#3c3947",
            search_current_bg: "#f0d8a8",
            search_current_fg: "#17151c",
            selection_bg: "#5b6f99",
            selection_fg: "#f4efe6",
        },
        AppTheme::EditorialDay => ThemePalette {
            window_bg: "#f7f2e8",
            body_fg: "#2a2c33",
            topbar_bg: "#efe8db",
            topbar_fg: "#353742",
            border: "#d8cfbe",
            overlay_title: "#5c5bd6",
            overlay_info: "#6f7283",
            entry_bg: "#fffaf0",
            entry_fg: "#2a2c33",
            entry_border: "#cfc4b1",
            hint_match: "#6052d8",
            hint_dim: "#a29aa8",
            status_bg: "#efe8db",
            status_fg: "#6f7283",
            scrollbar: "#c6baa6",
            heading_1: "#365d9d",
            heading_2: "#4b74bb",
            heading_3: "#5e6f9b",
            heading_4: "#7b63a8",
            heading_5: "#6d758c",
            heading_6: "#7f6f63",
            inline_code_bg: "#ece5d8",
            inline_code_fg: "#9a5f1f",
            code_block_bg: "#eee8de",
            code_block_fg: "#29313d",
            code_language_fg: "#4f6b9a",
            link_fg: "#4169b0",
            link_hint_target_match: "#8c5a15",
            link_hint_target_dim: "#8d8692",
            quote_fg: "#68779b",
            muted_fg: "#8b8591",
            search_match_bg: "#dcd4c7",
            search_current_bg: "#f1cf8e",
            search_current_fg: "#2b2419",
            selection_bg: "#6b89c6",
            selection_fg: "#fdf9f1",
        },
    }
}

pub fn apply_css(provider: &CssProvider, zoom_level: f64, theme: AppTheme) {
    let font_size = BASE_FONT_SIZE * zoom_level;
    let palette = palette(theme);

    if let Some(settings) = Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(matches!(theme, AppTheme::EditorialNight));
    }

    provider.load_from_data(&format!(
        r#"
        @define-color theme_base_color {window_bg};
        @define-color theme_bg_color {window_bg};
        @define-color window_bg_color {window_bg};
        @define-color view_bg_color {window_bg};
        @define-color content_view_bg {window_bg};
        @define-color text_view_bg {window_bg};
        @define-color theme_text_color {body_fg};
        @define-color theme_fg_color {body_fg};
        @define-color view_fg_color {body_fg};
        @define-color text_view_fg {body_fg};

        window.mdv-window,
        .mdv-root,
        overlay.mdv-content,
        overlay.mdv-content > scrolledwindow.mdv-scroller,
        scrolledwindow.mdv-scroller,
        viewport.mdv-viewport,
        viewport.mdv-viewport > box.mdv-page,
        box.mdv-page {{
            background-color: {window_bg};
            background-image: none;
            color: {body_fg};
        }}

        textview.mdv-view,
        .mdv-view,
        textview.mdv-view *,
        .mdv-view *,
        textview.mdv-view text,
        textview.mdv-view border {{
            background: {window_bg};
            background-color: {window_bg};
            background-image: none;
            box-shadow: none;
            color: {body_fg};
        }}

        textview.mdv-view text selection,
        textview.mdv-view text selection:focus,
        .mdv-view text selection,
        .mdv-view text selection:focus {{
            background: {selection_bg};
            background-color: {selection_bg};
            color: {selection_fg};
        }}

        textview.mdv-view,
        textview.mdv-view text,
        .mdv-view,
        .mdv-view * {{
            font-family: "IBM Plex Sans", "Inter", "Noto Sans", system-ui, sans-serif;
            font-size: {font_size:.2}pt;
            line-height: 1.42;
            letter-spacing: 0.01em;
            caret-color: transparent;
        }}

        .mdv-topbar {{
            background: {topbar_bg};
            color: {topbar_fg};
            padding: 10px 14px;
            border-bottom: 1px solid {border};
        }}

        .mdv-overlay-title {{
            color: {overlay_title};
            font-weight: 700;
        }}

        .mdv-overlay-info {{
            color: {overlay_info};
        }}

        entry.mdv-overlay-entry {{
            background: {entry_bg};
            color: {entry_fg};
            border-radius: 6px;
            border: 1px solid {entry_border};
            padding: 6px 10px;
        }}

        label.mdv-link-hint {{
            font-family: JetBrains Mono, Fira Code, monospace;
            font-size: {hint_font:.2}pt;
            font-weight: 700;
            padding: 0 1px;
            margin: 0;
            min-height: 0;
            min-width: 0;
            opacity: 0.96;
        }}

        label.mdv-link-hint.match {{
            color: {hint_match};
        }}

        label.mdv-link-hint.dim {{
            color: {hint_dim};
        }}

        .mdv-status {{
            background: {status_bg};
            color: {status_fg};
            padding: 8px 14px;
            border-top: 1px solid {border};
        }}

        scrollbar slider {{
            background: {scrollbar};
            border-radius: 999px;
        }}
        "#,
        window_bg = palette.window_bg,
        body_fg = palette.body_fg,
        topbar_bg = palette.topbar_bg,
        topbar_fg = palette.topbar_fg,
        border = palette.border,
        overlay_title = palette.overlay_title,
        overlay_info = palette.overlay_info,
        entry_bg = palette.entry_bg,
        entry_fg = palette.entry_fg,
        entry_border = palette.entry_border,
        hint_match = palette.hint_match,
        hint_dim = palette.hint_dim,
        status_bg = palette.status_bg,
        status_fg = palette.status_fg,
        scrollbar = palette.scrollbar,
        selection_bg = palette.selection_bg,
        selection_fg = palette.selection_fg,
        font_size = font_size,
        hint_font = font_size * 0.72,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_toggle_round_trips() {
        assert_eq!(AppTheme::EditorialNight.toggle(), AppTheme::EditorialDay);
        assert_eq!(AppTheme::EditorialDay.toggle(), AppTheme::EditorialNight);
    }
}
