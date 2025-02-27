//! Error types

#[derive(Debug)]
pub(crate) enum Error {
    TooManyBitsPerElement,
}

/// Re-exports of QuickXML error types. These can occur during parsing.
pub mod quickxml {
    pub use quick_xml::encoding::EncodingError;
    pub use quick_xml::errors::Error;
    pub use quick_xml::errors::IllFormedError;
    pub use quick_xml::errors::Result;
    pub use quick_xml::errors::SyntaxError;
    pub use quick_xml::escape::{EscapeError, ParseCharRefError};
    pub use quick_xml::events::attributes::AttrError;
    pub use quick_xml::name::NamespaceError;
}
