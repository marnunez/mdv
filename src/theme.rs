use gtk4::CssProvider;

const BASE_FONT_SIZE: f64 = 16.0;

pub fn apply_css(provider: &CssProvider, zoom_level: f64) {
    let font_size = BASE_FONT_SIZE * zoom_level;

    provider.load_from_data(&format!(
        r#"
        .mdv-window {{
            background: #1e1e2e;
            color: #cdd6f4;
        }}

        textview.mdv-view, textview.mdv-view text {{
            background: #1e1e2e;
            color: #cdd6f4;
            font-family: Inter, "Noto Sans", system-ui, sans-serif;
            font-size: {:.2}pt;
            line-height: 1.7;
            caret-color: transparent;
        }}

        .mdv-topbar {{
            background: #181825;
            color: #cdd6f4;
            padding: 10px 14px;
            border-bottom: 1px solid #313244;
        }}

        .mdv-overlay-title {{
            color: #cba6f7;
            font-weight: 700;
        }}

        .mdv-overlay-info {{
            color: #a6adc8;
        }}

        entry.mdv-overlay-entry {{
            background: #313244;
            color: #cdd6f4;
            border-radius: 6px;
            border: 1px solid #45475a;
            padding: 6px 10px;
        }}

        label.mdv-link-hint {{
            font-family: JetBrains Mono, Fira Code, monospace;
            font-size: {:.2}pt;
            font-weight: 700;
            padding: 0;
            margin: 0;
            min-height: 0;
            min-width: 0;
        }}

        label.mdv-link-hint.match {{
            color: #cba6f7;
        }}

        label.mdv-link-hint.dim {{
            color: #6c7086;
        }}

        .mdv-status {{
            background: #181825;
            color: #a6adc8;
            padding: 8px 14px;
            border-top: 1px solid #313244;
        }}

        scrollbar slider {{
            background: #45475a;
            border-radius: 999px;
        }}
        "#,
        font_size,
        font_size * 0.72
    ));
}
