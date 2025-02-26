use quick_xml::Error as QuickXMLError;

use crate::document::{Document, DocumentId, Node as DocumentNode};
use crate::parser::parse_document_with_id;

use crate::{NodeName, NodeType, ProcessingInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub(crate) document_id: DocumentId,
    pub(crate) document_node: DocumentNode,
}

/// The Xoz structure is a pool of multiple documents in your application.
///
/// All operations on nodes are done through this structure. Behavior of
/// nodes from different Xoz structures is undefined.
///
/// You can add documents to the pool but otherwise the documents are immutable.
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

impl Xoz {
    pub fn new() -> Self {
        Xoz {
            documents: Vec::new(),
        }
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

    // parse
    pub fn parse_str(&mut self, xml: &str) -> Result<Node, QuickXMLError> {
        let document = parse_document_with_id(self.new_document_id(), xml)?;
        let root = document.root();
        let root = document.new_node(root);
        self.documents.push(document);
        Ok(root)
    }
}
