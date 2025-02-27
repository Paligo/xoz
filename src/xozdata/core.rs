use crate::document::{Document, DocumentId, Node as DocumentNode};
use crate::error::quickxml::Result;
use crate::parser::parse_document_with_id;

/// A node in the Xoz structure.
///
/// A node can be in any document load into the Xoz structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub(crate) document_id: DocumentId,
    pub(crate) document_node: DocumentNode,
}

/// The Xoz structure holds all XML documents and is used for accessing them.
///
/// All operations on nodes are done through this structure. Combining nodes
/// from different Xoz structures is not supported and will result in undefined
/// behavior.
///
/// You can add documents to the pool but otherwise the documents are
/// immutable.
///
/// Xoz is implemented in several sections focusing on different aspects of
/// accessing XML data.
///
/// The Xoz struct documentation is divided into different sections:
///
/// * [Core](#core)
/// * [Information](#information)
/// * [Navigation](#navigation)
/// * [Text](#text)
/// * [Namespace](#namespace)
/// * [Attribute](#attribute)
/// * [Iteration](#iteration)
/// * [Comparison](#comparison)
pub struct Xoz {
    documents: Vec<Document>,
}

impl Document {
    pub(crate) fn new_node(&self, document_node: DocumentNode) -> Node {
        Node {
            document_id: self.id,
            document_node,
        }
    }
}

impl Default for Xoz {
    fn default() -> Self {
        Self::new()
    }
}

/// ## Core
/// Creation of the Xoz structure.
impl Xoz {
    /// Create a new empty Xoz structure.
    pub fn new() -> Self {
        Xoz {
            documents: Vec::new(),
        }
    }

    /// Heap size used by the Xoz structure.
    pub fn heap_size(&self) -> usize {
        self.documents.iter().map(|d| d.heap_size()).sum()
    }

    pub(crate) fn new_document_id(&self) -> DocumentId {
        DocumentId::new(self.documents.len())
    }

    pub(crate) fn document(&self, id: DocumentId) -> &Document {
        &self.documents[id.index()]
    }

    pub(crate) fn wrap(
        &self,
        node: Node,
        f: impl Fn(&Document, DocumentNode) -> DocumentNode,
    ) -> Node {
        let document = self.document(node.document_id);
        document.new_node(f(document, node.document_node))
    }

    pub(crate) fn wrap_option(
        &self,
        node: Node,
        f: impl Fn(&Document, DocumentNode) -> Option<DocumentNode>,
    ) -> Option<Node> {
        let document = self.document(node.document_id);
        f(document, node.document_node).map(|n| document.new_node(n))
    }

    /// Parse a string slice into a document and return the root node.
    pub fn parse_str(&mut self, xml: &str) -> Result<Node> {
        let document = parse_document_with_id(self.new_document_id(), xml)?;
        let root = document.root();
        let root = document.new_node(root);
        self.documents.push(document);
        Ok(root)
    }

    /// Serialize node to a string.
    pub fn serialize_to_string(&self, node: Node) -> String {
        let document = self.document(node.document_id);
        document.serialize_node_to_string(node.document_node)
    }
}
