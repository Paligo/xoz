use crate::ProcessingInstruction;

use super::core::{Node, Xoz};

impl Xoz {
    // str

    /// Text node string.
    ///
    /// If the node is not a text node, this returns `None`.
    pub fn text_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.text_str(node.document_node)
    }

    /// Comment node string.
    ///
    /// If the node is not a comment node, this returns `None`.
    pub fn comment_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.comment_str(node.document_node)
    }

    /// Processing instruction node string.
    ///
    /// This includes both target and content information.
    ///
    /// If the node is not a processing instruction node, this returns `None`.
    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.processing_instruction_str(node.document_node)
    }

    /// Get [`ProcessingInstruction`] if this is a processing instruction node.
    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        let document = self.document(node.document_id);
        document.processing_instruction(node.document_node)
    }

    /// Given a node, give back a string representation.
    ///
    /// For the root node and element nodes this gives back all text node
    /// descendant content, concatenated.
    ///
    /// For text nodes, it gives back the text.
    ///
    /// For comments, it gives back the comment text.
    ///
    /// For processing instructions, it gives back their content (data).
    ///
    /// For attribute nodes, it gives back the attribute value.
    ///
    /// For namespace nodes, it gives back the namespace URI.
    ///
    /// This is defined by the `string-value` property in
    /// <https://www.w3.org/TR/xpath-datamodel-31>
    pub fn string_value(&self, node: Node) -> String {
        let document = self.document(node.document_id);
        document.string_value(node.document_node)
    }

    /// Get the string content of a node.
    pub fn node_str(&self, node: Node) -> Option<&str> {
        let document = self.document(node.document_id);
        document.node_str(node.document_node)
    }
}
