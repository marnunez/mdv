#[derive(Clone, Debug)]
pub struct Span {
    pub start: i32,
    pub end: i32,
    pub tag: &'static str,
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
