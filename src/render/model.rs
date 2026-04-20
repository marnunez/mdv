#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HighlightStyle {
    pub foreground: (u8, u8, u8),
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpanKind {
    Tag(&'static str),
    Highlight(HighlightStyle),
}

#[derive(Clone, Debug)]
pub struct Span {
    pub start: i32,
    pub end: i32,
    pub kind: SpanKind,
}

#[derive(Clone, Debug)]
pub struct LinkInfo {
    pub code: String,
    pub label: String,
    pub url: String,
    pub start: i32,
    pub end: i32,
}

#[derive(Clone, Debug)]
pub struct HeadingInfo {
    pub title: String,
    pub anchor: String,
    pub offset: i32,
}

#[derive(Clone, Debug)]
pub struct RenderedDoc {
    pub text: String,
    pub spans: Vec<Span>,
    pub links: Vec<LinkInfo>,
    pub headings: Vec<HeadingInfo>,
}

impl Default for RenderedDoc {
    fn default() -> Self {
        Self {
            text: String::new(),
            spans: Vec::new(),
            links: Vec::new(),
            headings: Vec::new(),
        }
    }
}
