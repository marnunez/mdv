mod buffer;
mod markdown;
mod model;

pub use buffer::build_buffer;
pub use markdown::render_markdown;
pub use model::{HeadingInfo, LinkInfo, RenderedDoc, Span};
