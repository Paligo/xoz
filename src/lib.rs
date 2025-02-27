#![deny(missing_docs)]

//! Xoz is library that stores read-only XML data in a compact way and allows
//! jumping navigation.
//!
//! ## Succinct representation
//!
//! Xoz stores parsed documents in read-only fashion and allows tree access and
//! navigation. It uses [succinct data
//! structures](https://en.wikipedia.org/wiki/Succinct_data_structure) to store
//! the XML data much more compactly than typical DOM data structures.
//!
//! ## Jumping Navigation
//!
//! Xoz allows "jumping" navigation, jumping directly to a node of a certain
//! node type (such as a named element) without having to traverse the tree.
//! This is extremely efficient.
//!
//! [`Xoz::typed_descendant`] and [`Xoz::typed_foll`] are the basic jumping
//! operations. Built on top of this are various iterators that allow direct
//! iteration of nodes of a certain type: [`Xoz::typed_descendants`],
//! [`Xoz::typed_descendants_or_self`], [`Xoz::typed_following`].
//!
//! ## API
//!
//! The main API is exposed through the [`Xoz`] struct. The struct is used to
//! parse documents and has a large number of methods in order to navigate and
//! access the parsed documents.
//!
//! ## Example
//!
//! Here we parse a document, navigate to some of its node, and use
//! [`Xoz::typed_descendants`] to iterate over all elements with the name `a`.
//!
//! ```rust
//! use xoz::{Xoz, NodeType};
//!
//! // create the Xoz data structure
//! let mut xoz = Xoz::new();
//!
//! // parse a document into it. We have access to its root (document) node.
//! let root = xoz.parse_str("<p><a/><b><a/></b><a/></p>").unwrap();
//!
//! // now obtain node references to various nodes using simple navigation
//! let p = xoz.document_element(root);
//! let a1 = xoz.first_child(p).unwrap();
//! let b = xoz.next_sibling(a1).unwrap();
//! let a2 = xoz.first_child(b).unwrap();
//! let a3 = xoz.next_sibling(b).unwrap();
//!
//! // iterate over all `a` elements in the document using typed (jumping)
//! // navigation
//! let a_elements: Vec<_> = xoz.typed_descendants(root, NodeType::element("a")).collect();
//!
//! // assert that we found all `a` elements
//! assert_eq!(a_elements, vec![a1, a2, a3]);
//! ```

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
/// Re-export of the parser error from the [`quick_xml`] crate used for parsing.
pub use quick_xml::errors::Error as QuickXMLError;
pub use traverse::TraverseState;
pub use xozdata::{Node, Xoz};
