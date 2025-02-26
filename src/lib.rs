//! Xoz is library that stores XML data in a compact way and allows jumping
//! navigation.
//!
//! Xoz stores parsed documents in read-only fashion and allows tree access and
//! navigation. It uses [succinct data
//! structures](https://en.wikipedia.org/wiki/Succinct_data_structure) to store
//! the XML data in a much more compact way than typical DOM data structures.
//!
//! Xoz allows "jumping" navigation, jumping directly to a node of a certain
//! node type (such as a named element) without having to traverse the tree.
//! See [`Xoz::typed_descendant`] for more information.
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
mod xozdata;

pub use document::ProcessingInstruction;
pub use name::{Namespace, NodeName};
pub use node::NodeType;
pub use parser::QuickXMLError;
pub use traverse::TraverseState;
pub use xozdata::{Node, Xoz};
