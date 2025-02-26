use quick_xml::Error as QuickXMLError;

use crate::document::{Document, DocumentId, Node as DocumentNode};
use crate::parser::parse_document_with_id;

use crate::{NodeName, NodeType, ProcessingInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    document_id: DocumentId,
    document_node: DocumentNode,
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
    fn new_node(&self, document_node: DocumentNode) -> Node {
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

    fn new_document_id(&self) -> DocumentId {
        DocumentId::new(self.documents.len())
    }

    fn document(&self, id: DocumentId) -> &Document {
        &self.documents[id.index()]
    }

    fn wrap(&self, node: Node, f: impl Fn(&Document, DocumentNode) -> DocumentNode) -> Node {
        let document = self.document(node.document_id);
        document.new_node(f(document, node.document_node))
    }

    fn wrap_option(
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

    // nav

    pub fn document_element(&self, root: Node) -> Node {
        let document = self.document(root.document_id);
        document.new_node(document.document_element())
    }

    pub fn parent(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.parent(n))
    }

    pub fn first_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.first_child(n))
    }

    pub fn last_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.last_child(n))
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.next_sibling(n))
    }

    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.previous_sibling(n))
    }

    pub fn is_ancestor(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor(ancestor.document_node, descendant.document_node)
    }

    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor_or_self(ancestor.document_node, descendant.document_node)
    }

    pub fn top_element(&self, node: Node) -> Node {
        self.wrap(node, |doc, n| doc.top_element(n))
    }

    pub fn is_directly_under_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_directly_under_document(node.document_node)
    }

    pub fn is_document_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document_element(node.document_node)
    }

    pub fn child_index(&self, parent: Node, node: Node) -> Option<usize> {
        let parent_document_id = parent.document_id;
        let node_document_id = node.document_id;
        if parent_document_id != node_document_id {
            return None;
        }
        let document = self.document(node_document_id);
        document.child_index(parent.document_node, node.document_node)
    }

    pub fn typed_descendant(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_descendant(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }

    pub fn typed_foll(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_foll(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }

    // info

    pub fn preorder(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.preorder(node.document_node)
    }

    pub fn sort_key(&self, node: Node) -> (usize, usize) {
        let document = self.document(node.document_id);
        (document.id.index(), document.preorder(node.document_node))
    }

    pub fn node_name(&self, node: Node) -> Option<&NodeName> {
        let document = self.document(node.document_id);
        document.node_name(node.document_node)
    }

    pub fn node_type(&self, node: Node) -> &NodeType {
        let document = self.document(node.document_id);
        document.node_type(node.document_node)
    }

    pub fn is_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document(node.document_node)
    }

    pub fn is_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_element(node.document_node)
    }

    pub fn is_text(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_text(node.document_node)
    }

    pub fn is_comment(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_comment(node.document_node)
    }

    pub fn is_processing_instruction(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_processing_instruction(node.document_node)
    }

    pub fn is_attribute(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_attribute(node.document_node)
    }

    pub fn is_namespace(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_namespace(node.document_node)
    }

    pub fn subtree_count(&self, node: Node, node_type: NodeType) -> usize {
        let document = self.document(node.document_id);
        document.subtree_count(node.document_node, node_type)
    }

    pub fn subtree_size(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.subtree_size(node.document_node)
    }

    // attr
    pub fn attribute_node<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attribute_node(node.document_node, name)
            .map(|n| document.new_node(n))
    }

    pub fn attributes_child(&self, node: Node) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attributes_child(node.document_node)
            .map(|n| document.new_node(n))
    }

    pub fn attribute_value<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<&str> {
        let document = self.document(node.document_id);
        document.attribute_value(node.document_node, name)
    }

    pub fn attribute_entries<'a>(
        &'a self,
        node: Node,
    ) -> impl Iterator<Item = (&'a NodeName<'a>, &'a str)> + 'a {
        let document = self.document(node.document_id);
        document.attribute_entries(node.document_node)
    }

    // str
    pub fn text_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.text_str(node.document_node)
    }

    pub fn comment_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.comment_str(node.document_node)
    }

    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.processing_instruction_str(node.document_node)
    }

    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        let document = self.document(node.document_id);
        document.processing_instruction(node.document_node)
    }

    pub fn string_value(&self, node: Node) -> String {
        let document = self.document(node.document_id);
        document.string_value(node.document_node)
    }

    pub fn node_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.node_str(node.document_node)
    }
}
