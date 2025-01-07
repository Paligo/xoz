mod access;
mod builder;
mod document;
mod error;
mod serializer;
mod structure;
mod tag;
mod tags_builder;
mod tagvec;
mod text;

pub use builder::parse_document;
pub use document::Name;
pub use tag::{TagInfo, TagType};
pub use tagvec::TagId;
