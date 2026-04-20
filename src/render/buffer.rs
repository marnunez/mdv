use gtk4::pango;
use gtk4::prelude::*;
use gtk4::TextBuffer;
use pulldown_cmark::HeadingLevel;

use super::RenderedDoc;

pub fn build_buffer(rendered: &RenderedDoc) -> TextBuffer {
    let buffer = TextBuffer::new(None::<&gtk4::TextTagTable>);

    let _ = buffer.create_tag(
        Some("heading_1"),
        &[
            ("weight", &700i32),
            ("scale", &1.9f64),
            ("foreground", &"#cba6f7"),
            ("pixels-above-lines", &18i32),
            ("pixels-below-lines", &8i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_2"),
        &[
            ("weight", &700i32),
            ("scale", &1.55f64),
            ("foreground", &"#89b4fa"),
            ("pixels-above-lines", &16i32),
            ("pixels-below-lines", &7i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_3"),
        &[
            ("weight", &700i32),
            ("scale", &1.3f64),
            ("foreground", &"#a6e3a1"),
            ("pixels-above-lines", &14i32),
            ("pixels-below-lines", &6i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_4"),
        &[
            ("weight", &700i32),
            ("scale", &1.15f64),
            ("foreground", &"#f9e2af"),
            ("pixels-above-lines", &12i32),
            ("pixels-below-lines", &5i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_5"),
        &[
            ("weight", &700i32),
            ("foreground", &"#f5c2e7"),
            ("pixels-above-lines", &10i32),
            ("pixels-below-lines", &4i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("heading_6"),
        &[
            ("weight", &700i32),
            ("foreground", &"#fab387"),
            ("pixels-above-lines", &10i32),
            ("pixels-below-lines", &4i32),
        ],
    );
    let _ = buffer.create_tag(Some("emphasis"), &[("style", &pango::Style::Italic)]);
    let _ = buffer.create_tag(Some("strong"), &[("weight", &700i32)]);
    let _ = buffer.create_tag(Some("strikethrough"), &[("strikethrough", &true)]);
    let _ = buffer.create_tag(
        Some("inline_code"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("background", &"#313244"),
            ("foreground", &"#fab387"),
        ],
    );
    let _ = buffer.create_tag(
        Some("code_block"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("background", &"#181825"),
            ("foreground", &"#cdd6f4"),
            ("left-margin", &18i32),
            ("right-margin", &18i32),
            ("pixels-above-lines", &8i32),
            ("pixels-below-lines", &8i32),
        ],
    );
    let _ = buffer.create_tag(
        Some("link"),
        &[
            ("underline", &pango::Underline::Single),
            ("foreground", &"#89b4fa"),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_code_match"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("weight", &700i32),
            ("scale", &0.72f64),
            ("rise", &7000i32),
            ("foreground", &"#cba6f7"),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_code_dim"),
        &[
            ("family", &"JetBrains Mono, Fira Code, monospace"),
            ("weight", &700i32),
            ("scale", &0.72f64),
            ("rise", &7000i32),
            ("foreground", &"#6c7086"),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_target_match"),
        &[
            ("weight", &700i32),
            ("underline", &pango::Underline::Single),
            ("foreground", &"#f9e2af"),
        ],
    );
    let _ = buffer.create_tag(
        Some("link_hint_target_dim"),
        &[
            ("foreground", &"#6c7086"),
        ],
    );
    let _ = buffer.create_tag(
        Some("quote"),
        &[
            ("foreground", &"#a6adc8"),
            ("left-margin", &16i32),
        ],
    );
    let _ = buffer.create_tag(Some("muted"), &[("foreground", &"#6c7086")]);
    let _ = buffer.create_tag(Some("search_match"), &[("background", &"#45475a")]);
    let _ = buffer.create_tag(
        Some("search_current"),
        &[
            ("background", &"#f9e2af"),
            ("foreground", &"#1e1e2e"),
        ],
    );

    buffer.set_text(&rendered.text);

    for span in &rendered.spans {
        let start = buffer.iter_at_offset(span.start);
        let end = buffer.iter_at_offset(span.end);
        buffer.apply_tag_by_name(span.tag, &start, &end);
    }

    buffer
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
