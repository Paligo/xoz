use crate::{NodeType, TagState};

use super::core::{Node, Xoz};

/// ## Comparison
///
/// Functions for comparing nodes. Comparison between different documents is
/// supported.
impl Xoz {
    /// XPath deep equal
    /// Comparison of two nodes as defined by the XPath deep-equal function:
    ///
    /// <https://www.w3.org/TR/xpath-functions-31/#func-deep-equal>
    ///
    /// We ignore anything about typed content in that definition.
    pub fn deep_equal_xpath(
        &self,
        a: Node,
        b: Node,
        text_compare: impl Fn(&str, &str) -> bool,
    ) -> bool {
        // the top level comparison needs to compare the node, even if
        // processing instruction or a comment, though for elements, we want to
        // compare the structure and filter comments and processing
        // instructions out.
        match (self.node_type(a), self.node_type(b)) {
            (NodeType::Element(_), NodeType::Element(_))
            | (NodeType::Document, NodeType::Document) => self.advanced_deep_equal(
                a,
                b,
                // we need to only consider elements and text nodes for
                // root/element content comparison
                |node| self.is_element(node) || self.is_text(node),
                text_compare,
            ),
            (a_type, b_type) => self.advanced_compare_node(a, a_type, b, b_type, text_compare),
        }
    }

    /// Compare two nodes for semantic equality with custom text compare and
    /// filtering.
    ///
    /// This is a deep comparison of the nodes and their children. The trees
    /// have to have the same structure.
    ///
    /// A name is considered to be semantically equal to another name if they
    /// have the same namespace and local name. Prefixes are ignored.
    ///
    /// Two elements are the same if their name and attributes are the same.
    /// Namespace declarations are ignored.
    ///
    /// You can include only the nodes that are relevant to the comparison
    /// using the filter function.
    ///
    /// Text nodes and attributes are compared using the provided comparison function.
    pub fn advanced_deep_equal<F, C>(&self, a: Node, b: Node, filter: F, text_compare: C) -> bool
    where
        F: Fn(Node) -> bool,
        C: Fn(&str, &str) -> bool,
    {
        let mut edges_a = self.traverse(a).filter(|(_, _, node)| filter(*node));
        let mut edges_b = self.traverse(b).filter(|(_, _, node)| filter(*node));
        for ((a_type, a_state, a_node), (b_type, b_state, b_node)) in
            edges_a.by_ref().zip(edges_b.by_ref())
        {
            match (a_state, b_state) {
                // an empty is never going to be represented as separate open/close,
                // so making sure empty is the same is safe.
                (TagState::Open, TagState::Open) | (TagState::Empty, TagState::Empty) => {
                    if !self.advanced_compare_node(a_node, a_type, b_node, b_type, &text_compare) {
                        return false;
                    }
                }
                (TagState::Close, TagState::Close) => {
                    // the structure is the same, so we can continue.
                    // if we had a different node type for close but the same node type
                    // for open, the tree would be unbalanced. XML cannot be unbalanced,
                    // so we don't need to compare the node type here
                }
                _ => {
                    // if there is a difference in structure, this will fire
                    return false;
                }
            }
        }
        // if we have leftover elements in the iterators, the trees are not equal
        if edges_a.next().is_some() || edges_b.next().is_some() {
            return false;
        }
        true
    }

    pub(crate) fn advanced_compare_node<C>(
        &self,
        a: Node,
        a_type: &NodeType,
        b: Node,
        b_type: &NodeType,
        text_compare: C,
    ) -> bool
    where
        C: Fn(&str, &str) -> bool,
    {
        match (a_type, b_type) {
            (NodeType::Document, NodeType::Document) => true,
            (NodeType::Element(a_name), NodeType::Element(b_name)) => {
                a_name == b_name && self.advanced_compare_attributes(a, b, text_compare)
            }
            (NodeType::Text, NodeType::Text) => {
                text_compare(self.node_str(a).unwrap(), self.node_str(b).unwrap())
            }
            (NodeType::Comment, NodeType::Comment) => {
                self.node_str(a).unwrap() == self.node_str(b).unwrap()
            }
            (NodeType::ProcessingInstruction, NodeType::ProcessingInstruction) => {
                let a_pi = self.processing_instruction(a).unwrap();
                let b_pi = self.processing_instruction(b).unwrap();
                // TODO: is a text compare really want we want here?
                a_pi.target() == b_pi.target()
                    && text_compare(
                        std::str::from_utf8(a_pi.content()).unwrap(),
                        std::str::from_utf8(b_pi.content()).unwrap(),
                    )
            }
            (NodeType::Attribute(a_name), NodeType::Attribute(b_name)) => {
                a_name == b_name
                    && text_compare(self.node_str(a).unwrap(), self.node_str(b).unwrap())
            }
            (NodeType::Namespace(a_ns), NodeType::Namespace(b_ns)) => a_ns == b_ns,
            _ => false,
        }
    }

    fn advanced_compare_attributes<C>(&self, a: Node, b: Node, text_compare: C) -> bool
    where
        C: Fn(&str, &str) -> bool,
    {
        let a_attributes_node = self.attributes_child(a);
        let b_attributes_node = self.attributes_child(b);

        match (a_attributes_node, b_attributes_node) {
            (Some(a_attributes_node), Some(b_attributes_node)) => {
                let a_size = self.subtree_size(a_attributes_node);
                let b_size = self.subtree_size(b_attributes_node);
                if a_size != b_size {
                    return false;
                }
                for (key, value_a) in self.attribute_entries(a) {
                    let value_b = self.attribute_value(b, key.clone());
                    if let Some(value_b) = value_b {
                        if !text_compare(value_a, value_b) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            }
            (None, None) => true,
            _ => false,
        }
    }
}
