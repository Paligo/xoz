mod access;
mod builder;
mod document;
mod error;
mod iter;
mod mta;
mod mta_compiler;
mod parser;
mod serializer;
mod structure;
mod tag;
mod tags_builder;
mod tagvec;
mod text;
mod textsearch;

pub use parser::{parse_document, QuickXMLError};
// pub use builder::parse_document;
pub use document::Name;
pub use tag::{Namespace, TagInfo, TagName, TagType};
pub use tagvec::TagId;
