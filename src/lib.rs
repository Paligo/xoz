mod access;
mod builder;
mod document;
mod error;
mod iter;
mod mta;
mod mta_compiler;
mod name;
mod node;
mod node_info_vec;
mod parser;
mod serializer;
mod structure;
mod text;
mod text_fm;
mod textsearch;
mod traverse;
mod tree_builder;

pub use document::{Document, Node};
pub use parser::{parse_document, QuickXMLError};
// pub use builder::parse_document;
pub use name::{Namespace, NodeName};
pub use node::{NodeInfo, NodeType};
pub use node_info_vec::NodeInfoId;
pub use traverse::TagState;
