use quick_xml::events::BytesPI;
use vers_vecs::trees::Tree;

use crate::{node_info_vec::NodeInfoId, NodeType};

use super::{Document, Node};

impl Document {
    pub fn text_str(&self, node: Node) -> Option<&str> {
        if matches!(self.node_type(node), NodeType::Text) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn comment_str(&self, node: Node) -> Option<&str> {
        if matches!(self.node_type(node), NodeType::Comment) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        if matches!(self.node_type(node), NodeType::ProcessingInstruction) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        if matches!(self.node_type(node), NodeType::ProcessingInstruction) {
            let s = self.node_str(node).expect("Missing PI data");
            Some(ProcessingInstruction { data: s })
        } else {
            None
        }
    }

    pub fn string_value(&self, node: Node) -> String {
        match self.node_type(node) {
            NodeType::Document | NodeType::Element(_) => self.descendants_to_string(node),
            NodeType::Text | NodeType::Comment | NodeType::Attribute(_) => {
                self.node_str(node).unwrap().to_string()
            }
            NodeType::ProcessingInstruction => self.processing_instruction(node).unwrap().content(),
            NodeType::Namespace(namespace) => {
                let uri = namespace.uri();
                String::from_utf8(uri.to_vec()).expect("Namespace URI is not utf8")
            }
            NodeType::Namespaces | NodeType::Attributes => {
                panic!("Cannot use this with namespaces or attribute node")
            }
        }
    }

    pub(crate) fn node_str(&self, node: Node) -> Option<&str> {
        let text_id = self.structure.text_id(node.get());
        Some(self.text_usage.text_value(text_id))
    }

    fn descendants_to_string(&self, node: Node) -> String {
        let texts = self.descendants(node).filter_map(|n| self.text_str(n));
        let (lower_bound, _) = texts.size_hint();
        let mut r = String::with_capacity(lower_bound);
        for text in texts {
            r.push_str(text);
        }
        r
    }
}

/// Represents the text content of a processing instruction node.
pub struct ProcessingInstruction<'a> {
    data: &'a str,
}

impl ProcessingInstruction<'_> {
    /// The target of the processing instruction.
    ///
    /// Given a `<?foo bar?>` processing instruction, this is
    /// the string `"foo"`.
    pub fn target(&self) -> String {
        let bytes_pi = BytesPI::new(self.data);
        let target = std::str::from_utf8(bytes_pi.target()).expect("PI target is not utf8");
        target.to_string()
    }

    /// The content of the processing instruction.
    ///
    /// Given a `<?foo bar?>` processing instruction, this is
    /// the string `" bar"` including the space character.
    pub fn content(&self) -> String {
        let bytes_pi = BytesPI::new(self.data);
        let content = std::str::from_utf8(bytes_pi.content()).expect("PI content is not utf8");
        content.to_string()
    }
}
