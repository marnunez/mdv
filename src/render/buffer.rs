use std::collections::HashMap;

use gtk4::gdk;
use gtk4::pango;
use gtk4::prelude::*;
use gtk4::{TextBuffer, TextTag};
use pulldown_cmark::HeadingLevel;

use super::{HighlightStyle, RenderedDoc, SpanKind};
use crate::theme::ThemePalette;

pub fn build_buffer(rendered: &RenderedDoc, palette: ThemePalette) -> TextBuffer {
    let buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);

    let _ = buffer.create_tag(
        Some("heading_1"),
        &[
            ("weight", &700i32),
            ("scale", &1.72f64),
            ("foreground", &palette.heading_1),
            ("pixels-above-lines", &12i32),
            ("pixels-below-lines", &3i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_2"),
        &[
            ("weight", &700i32),
            ("scale", &1.42f64),
            ("foreground", &palette.heading_2),
            ("pixels-above-lines", &10i32),
            ("pixels-below-lines", &3i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_3"),
        &[
            ("weight", &700i32),
            ("scale", &1.22f64),
            ("foreground", &palette.heading_3),
            ("pixels-above-lines", &9i32),
            ("pixels-below-lines", &2i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_4"),
        &[
            ("weight", &700i32),
            ("scale", &1.1f64),
            ("foreground", &palette.heading_4),
            ("pixels-above-lines", &8i32),
            ("pixels-below-lines", &2i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_5"),
        &[
            ("weight", &700i32),
            ("foreground", &palette.heading_5),
            ("pixels-above-lines", &7i32),
            ("pixels-below-lines", &2i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_6"),
        &[
            ("weight", &700i32),
            ("foreground", &palette.heading_6),
            ("pixels-above-lines", &7i32),
            ("pixels-below-lines", &2i32),
        ],
    );
    let _ = buffer.create_tag(Some("body"), &[("foreground", &palette.body_fg)]);
    let _ = buffer.create_tag(Some("emphasis"), &[("style", &pango::Style::Italic)]);
    let _ = buffer.create_tag(Some("strong"), &[("weight", &700i32)]);
    let _ = buffer.create_tag(Some("strikethrough"), &[("strikethrough", &true)]);
    let _ = buffer.create_tag(
        Some("inline_code"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("background", &palette.inline_code_bg),
            ("foreground", &palette.inline_code_fg),
            ("weight", &500i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("code_block"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("paragraph-background", &palette.code_block_bg),
            ("foreground", &palette.code_block_fg),
            ("left-margin", &16i32),
            ("right-margin", &16i32),
            ("indent", &8i32),
            ("pixels-above-lines", &4i32),
            ("pixels-below-lines", &4i32),
            ("pixels-inside-wrap", &4i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("code_language"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("foreground", &palette.code_language_fg),
            ("weight", &700i32),
            ("scale", &0.82f64),
            ("pixels-above-lines", &2i32),
            ("pixels-below-lines", &1i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("link"),
        &[
            ("underline", &pango::Underline::Single),
            ("foreground", &palette.link_fg),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_code_match"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("weight", &700i32),
            ("scale", &0.72f64),
            ("rise", &7000i32),
            ("foreground", &palette.overlay_title),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_code_dim"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("weight", &700i32),
            ("scale", &0.72f64),
            ("rise", &7000i32),
            ("foreground", &palette.hint_dim),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_target_match"),
        &[
            ("weight", &700i32),
            ("underline", &pango::Underline::Single),
            ("foreground", &palette.link_hint_target_match),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_target_dim"),
        &[("foreground", &palette.link_hint_target_dim)],
    );
    let _ = buffer.create_tag(
        Some("quote"),
        &[
            ("foreground", &palette.quote_fg),
            ("left-margin", &12i32),
            ("indent", &4i32),
            ("style", &pango::Style::Italic),
        ],
    );
    let _ = buffer.create_tag(
        Some("muted"),
        &[("foreground", &palette.muted_fg), ("scale", &0.92f64)],
    );
    let _ = buffer.create_tag(
        Some("search_match"),
        &[("background", &palette.search_match_bg)],
    );
    let _ = buffer.create_tag(
        Some("search_current"),
        &[
            ("background", &palette.search_current_bg),
            ("foreground", &palette.search_current_fg),
        ],
    );

    buffer.set_text(&rendered.text);

    let start = buffer.start_iter();
    let end = buffer.end_iter();
    buffer.apply_tag_by_name("body", &start, &end);

    let mut highlight_tags: HashMap<HighlightStyle, String> = HashMap::new();

    for span in &rendered.spans {
        let start = buffer.iter_at_offset(span.start);
        let end = buffer.iter_at_offset(span.end);

        match &span.kind {
            SpanKind::Tag(tag) => buffer.apply_tag_by_name(tag, &start, &end),
            SpanKind::Highlight(style) => {
                let tag_name = highlight_tags
                    .entry(style.clone())
                    .or_insert_with(|| create_highlight_tag(&buffer, style));
                buffer.apply_tag_by_name(tag_name, &start, &end);
            }
        }
    }

    if let Some(code_block_tag) = buffer.tag_table().lookup("code_block") {
        code_block_tag.set_priority(0);
    }

    buffer
}

fn create_highlight_tag(buffer: &TextBuffer, style: &HighlightStyle) -> String {
    let tag_name = format!(
        "hl-{:02x}{:02x}{:02x}-{}{}{}",
        style.foreground.0,
        style.foreground.1,
        style.foreground.2,
        if style.bold { 'b' } else { '-' },
        if style.italic { 'i' } else { '-' },
        if style.underline { 'u' } else { '-' }
    );

    let tag = TextTag::new(Some(&tag_name));
    tag.set_foreground_rgba(Some(&rgba(style.foreground)));
    if style.bold {
        tag.set_weight(700);
    }
    if style.italic {
        tag.set_style(pango::Style::Italic);
    }
    if style.underline {
        tag.set_underline(pango::Underline::Single);
    }

    buffer.tag_table().add(&tag);
    tag_name
}

fn rgba((r, g, b): (u8, u8, u8)) -> gdk::RGBA {
    gdk::RGBA::new(
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
        1.0,
    )
}

#[allow(dead_code)]
fn heading_tag_name(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "heading_1",
        HeadingLevel::H2 => "heading_2",
        HeadingLevel::H3 => "heading_3",
        HeadingLevel::H4 => "heading_4",
        HeadingLevel::H5 => "heading_5",
        HeadingLevel::H6 => "heading_6",
    }
}
