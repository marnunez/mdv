use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use super::{HeadingInfo, LinkInfo, RenderedDoc, Span};

const HINT_ALPHABET: &[u8] = b"asdfghjklqwertyuiopzxcvbnm";

#[derive(Clone, Debug)]
struct ActiveHeading {
    level: HeadingLevel,
    start: i32,
    title: String,
}

#[derive(Clone, Debug)]
struct ActiveLink {
    url: String,
    start: i32,
    label: String,
}

#[derive(Clone, Debug)]
struct ListState {
    ordered: bool,
    next_number: u64,
}

struct RenderBuilder {
    text: String,
    offset: i32,
    line_start: bool,
    spans: Vec<Span>,
    links: Vec<LinkInfo>,
    headings: Vec<HeadingInfo>,
    emphasis_starts: Vec<i32>,
    strong_starts: Vec<i32>,
    strike_starts: Vec<i32>,
    heading_stack: Vec<ActiveHeading>,
    link_stack: Vec<ActiveLink>,
    code_block_starts: Vec<i32>,
    quote_starts: Vec<i32>,
    list_stack: Vec<ListState>,
    table_cell_index: usize,
}

impl RenderBuilder {
    fn new() -> Self {
        Self {
            text: String::new(),
            offset: 0,
            line_start: true,
            spans: Vec::new(),
            links: Vec::new(),
            headings: Vec::new(),
            emphasis_starts: Vec::new(),
            strong_starts: Vec::new(),
            strike_starts: Vec::new(),
            heading_stack: Vec::new(),
            link_stack: Vec::new(),
            code_block_starts: Vec::new(),
            quote_starts: Vec::new(),
            list_stack: Vec::new(),
            table_cell_index: 0,
        }
    }

    fn finish(mut self) -> RenderedDoc {
        self.trim_trailing_blank_lines();

        for (index, link) in self.links.iter_mut().enumerate() {
            link.code = hint_code(index);
            link.label = collapse_whitespace(&link.label);
            if link.label.is_empty() {
                link.label = link.url.clone();
            }
        }

        for heading in &mut self.headings {
            heading.title = collapse_whitespace(&heading.title);
        }

        RenderedDoc {
            text: self.text,
            spans: self.spans,
            links: self.links,
            headings: self.headings,
        }
    }

    fn append_raw(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        self.text.push_str(text);
        self.offset += text.chars().count() as i32;
        self.line_start = text.ends_with('\n');
    }

    fn append_captured(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        if self.line_start && !self.quote_starts.is_empty() {
            let prefix = "│ ".repeat(self.quote_starts.len());
            self.append_raw(&prefix);
        }

        self.text.push_str(text);
        self.offset += text.chars().count() as i32;
        self.line_start = text.ends_with('\n');

        if let Some(heading) = self.heading_stack.last_mut() {
            heading.title.push_str(text);
        }
        if let Some(link) = self.link_stack.last_mut() {
            link.label.push_str(text);
        }
    }

    fn ensure_newline(&mut self) {
        if self.text.is_empty() || self.text.ends_with('\n') {
            return;
        }
        self.append_raw("\n");
    }

    fn ensure_blank_line(&mut self) {
        if self.text.is_empty() {
            return;
        }
        if self.text.ends_with("\n\n") {
            self.line_start = true;
            return;
        }
        if self.text.ends_with('\n') {
            self.append_raw("\n");
        } else {
            self.append_raw("\n\n");
        }
    }

    fn trim_trailing_blank_lines(&mut self) {
        while self.text.ends_with("\n\n\n") {
            self.text.pop();
            self.offset -= 1;
        }
    }

    fn push_span(&mut self, start: i32, end: i32, tag: &'static str) {
        if end > start {
            self.spans.push(Span { start, end, tag });
        }
    }

    fn start_item_prefix(&mut self) {
        if !self.line_start {
            self.ensure_newline();
        }

        if !self.quote_starts.is_empty() {
            let prefix = "│ ".repeat(self.quote_starts.len());
            self.append_raw(&prefix);
        }

        if let Some(list) = self.list_stack.last_mut() {
            let marker = if list.ordered {
                format!("{}. ", list.next_number)
            } else {
                "• ".to_string()
            };
            self.append_raw(&marker);
        }
    }
}

pub fn render_markdown(markdown: &str) -> RenderedDoc {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);
    let mut builder = RenderBuilder::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    if !builder.line_start {
                        builder.ensure_blank_line();
                    }
                }
                Tag::Heading { level, .. } => {
                    builder.ensure_blank_line();
                    if builder.line_start && !builder.quote_starts.is_empty() {
                        let prefix = "│ ".repeat(builder.quote_starts.len());
                        builder.append_raw(&prefix);
                    }
                    builder.heading_stack.push(ActiveHeading {
                        level,
                        start: builder.offset,
                        title: String::new(),
                    });
                }
                Tag::BlockQuote(_) => {
                    builder.ensure_blank_line();
                    builder.quote_starts.push(builder.offset);
                }
                Tag::CodeBlock(kind) => {
                    builder.ensure_blank_line();
                    if let CodeBlockKind::Fenced(lang) = kind {
                        let lang = lang.trim();
                        if !lang.is_empty() {
                            builder.append_raw(&format!("{}\n", lang));
                            let start = builder.offset - lang.chars().count() as i32 - 1;
                            builder.push_span(start, start + lang.chars().count() as i32, "muted");
                        }
                    }
                    builder.code_block_starts.push(builder.offset);
                }
                Tag::List(first) => {
                    if builder.list_stack.is_empty() {
                        builder.ensure_blank_line();
                    }
                    builder.list_stack.push(ListState {
                        ordered: first.is_some(),
                        next_number: first.unwrap_or(1),
                    });
                }
                Tag::Item => builder.start_item_prefix(),
                Tag::Table(_) => builder.ensure_blank_line(),
                Tag::TableHead => {}
                Tag::TableRow => {
                    if !builder.line_start {
                        builder.ensure_newline();
                    }
                    if builder.line_start && !builder.quote_starts.is_empty() {
                        let prefix = "│ ".repeat(builder.quote_starts.len());
                        builder.append_raw(&prefix);
                    }
                    builder.table_cell_index = 0;
                }
                Tag::TableCell => {
                    if builder.table_cell_index > 0 {
                        builder.append_raw(" │ ");
                    }
                }
                Tag::Emphasis => builder.emphasis_starts.push(builder.offset),
                Tag::Strong => builder.strong_starts.push(builder.offset),
                Tag::Strikethrough => builder.strike_starts.push(builder.offset),
                Tag::Link { dest_url, .. } => {
                    builder.link_stack.push(ActiveLink {
                        url: dest_url.to_string(),
                        start: builder.offset,
                        label: String::new(),
                    });
                }
                Tag::Image { dest_url, .. } => {
                    builder.append_raw("🖼 ");
                    builder.link_stack.push(ActiveLink {
                        url: dest_url.to_string(),
                        start: builder.offset,
                        label: String::new(),
                    });
                }
                Tag::DefinitionList => builder.ensure_blank_line(),
                Tag::DefinitionListTitle => {
                    if !builder.line_start {
                        builder.ensure_blank_line();
                    }
                }
                Tag::DefinitionListDefinition => builder.append_raw("  "),
                Tag::HtmlBlock | Tag::FootnoteDefinition(_) | Tag::MetadataBlock(_) => {
                    builder.ensure_blank_line();
                }
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => builder.ensure_blank_line(),
                TagEnd::Heading(_) => {
                    if let Some(active) = builder.heading_stack.pop() {
                        let tag_name = heading_tag_name(active.level);
                        builder.push_span(active.start, builder.offset, tag_name);
                        builder.headings.push(HeadingInfo {
                            title: active.title,
                        });
                    }
                    builder.ensure_blank_line();
                }
                TagEnd::BlockQuote(_) => {
                    if let Some(start) = builder.quote_starts.pop() {
                        builder.push_span(start, builder.offset, "quote");
                    }
                    builder.ensure_blank_line();
                }
                TagEnd::CodeBlock => {
                    if let Some(start) = builder.code_block_starts.pop() {
                        builder.push_span(start, builder.offset, "code_block");
                    }
                    builder.ensure_blank_line();
                }
                TagEnd::List(_) => {
                    builder.list_stack.pop();
                    builder.ensure_blank_line();
                }
                TagEnd::Item => {
                    builder.ensure_newline();
                    if let Some(list) = builder.list_stack.last_mut() {
                        if list.ordered {
                            list.next_number += 1;
                        }
                    }
                }
                TagEnd::Table => builder.ensure_blank_line(),
                TagEnd::TableHead => builder.ensure_newline(),
                TagEnd::TableRow => builder.ensure_newline(),
                TagEnd::TableCell => builder.table_cell_index += 1,
                TagEnd::Emphasis => {
                    if let Some(start) = builder.emphasis_starts.pop() {
                        builder.push_span(start, builder.offset, "emphasis");
                    }
                }
                TagEnd::Strong => {
                    if let Some(start) = builder.strong_starts.pop() {
                        builder.push_span(start, builder.offset, "strong");
                    }
                }
                TagEnd::Strikethrough => {
                    if let Some(start) = builder.strike_starts.pop() {
                        builder.push_span(start, builder.offset, "strikethrough");
                    }
                }
                TagEnd::Link | TagEnd::Image => {
                    if let Some(link) = builder.link_stack.pop() {
                        builder.push_span(link.start, builder.offset, "link");
                        builder.links.push(LinkInfo {
                            code: String::new(),
                            label: link.label,
                            url: link.url,
                            start: link.start,
                            end: builder.offset,
                        });
                    }
                }
                TagEnd::DefinitionList => builder.ensure_blank_line(),
                TagEnd::DefinitionListTitle => builder.ensure_newline(),
                TagEnd::DefinitionListDefinition => builder.ensure_blank_line(),
                TagEnd::HtmlBlock | TagEnd::FootnoteDefinition | TagEnd::MetadataBlock(_) => {
                    builder.ensure_blank_line();
                }
            },
            Event::Text(text) => builder.append_captured(&text),
            Event::Code(text) => {
                let start = builder.offset;
                builder.append_captured(&text);
                builder.push_span(start, builder.offset, "inline_code");
            }
            Event::InlineMath(text) | Event::DisplayMath(text) => {
                let start = builder.offset;
                builder.append_captured(&text);
                builder.push_span(start, builder.offset, "inline_code");
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let start = builder.offset;
                builder.append_captured(&html);
                builder.push_span(start, builder.offset, "muted");
            }
            Event::FootnoteReference(label) => {
                let start = builder.offset;
                builder.append_captured(&format!("[{}]", label));
                builder.push_span(start, builder.offset, "muted");
            }
            Event::SoftBreak | Event::HardBreak => builder.append_raw("\n"),
            Event::Rule => {
                builder.ensure_blank_line();
                let start = builder.offset;
                builder.append_raw("────────────────────────");
                builder.push_span(start, builder.offset, "muted");
                builder.ensure_blank_line();
            }
            Event::TaskListMarker(done) => {
                builder.append_raw(if done { "[x] " } else { "[ ] " });
            }
        }
    }

    builder.finish()
}

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

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
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
