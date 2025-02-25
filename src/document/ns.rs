use crate::{iter::NamespacesIter, NodeType};

use super::{Document, Node};

const XML_NAMESPACE: &[u8] = b"http://www.w3.org/XML/1998/namespace";

impl Document {
    /// Get a node which contains the namespace declarations ("xmlns") children of
    /// of this node.
    ///
    /// This node has tag type `TagType::Namespaces`.
    ///
    /// If this is not an element node, or there are no namespace declarations,
    /// it returns `None`.
    pub(crate) fn namespaces_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            let node_info_id = self.node_info_id_for_node(node);
            if node_info_id.is_namespaces() {
                // the first child is the namespaces node
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get an iterator over the namespace declarations of this node.
    ///
    /// This iterates over prefix, uri tuples.
    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + use<'_> {
        NamespacesIter::new(self, node).map(move |n| match self.node_type(n) {
            NodeType::Namespace(namespace) => (namespace.prefix(), namespace.uri()),
            _ => unreachable!(),
        })
    }

    /// Given a namespace URI, return the prefix for this node
    ///
    /// This walks up the tree to find the first namespace declaration
    /// that has the given URI. If an element declares multiple prefixes for the
    /// same URI then an empty prefix is preferred over non-empty prefix.
    pub fn prefix_for_namespace(&self, node: Node, uri: &[u8]) -> Option<&[u8]> {
        for ancestor in self.ancestors_or_self(node) {
            let mut found_prefix = None;
            for (prefix, namespace_uri) in self.namespace_entries(ancestor) {
                if namespace_uri == uri {
                    if prefix.is_empty() {
                        return Some(prefix);
                    }
                    found_prefix = Some(prefix);
                }
            }
            if let Some(prefix) = found_prefix {
                return Some(prefix);
            }
        }
        if uri == XML_NAMESPACE {
            Some(b"xml")
        } else {
            None
        }
    }
}
