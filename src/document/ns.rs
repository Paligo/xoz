use crate::{iter::NamespacesIter, NodeType};

use super::{Document, Node};

const XML_NAMESPACE: &[u8] = b"http://www.w3.org/XML/1998/namespace";

impl Document {
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

    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + use<'_> {
        NamespacesIter::new(self, node).map(move |n| match self.node_type(n) {
            NodeType::Namespace(namespace) => (namespace.prefix(), namespace.uri()),
            _ => unreachable!(),
        })
    }

    pub fn prefix_for_namespace(&self, node: Node, uri: &[u8]) -> Option<&[u8]> {
        if uri.is_empty() {
            return Some(b"");
        }
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

    pub fn node_prefix(&self, node: Node) -> Option<&[u8]> {
        let name = self.node_name(node)?;
        self.prefix_for_namespace(node, name.namespace())
    }

    pub fn node_full_name(&self, node: Node) -> Option<String> {
        let name = self.node_name(node)?;
        let prefix = self.prefix_for_namespace(node, name.namespace())?;
        if prefix.is_empty() {
            Some(std::str::from_utf8(name.local_name()).unwrap().to_string())
        } else {
            Some(format!(
                "{}:{}",
                std::str::from_utf8(prefix).unwrap(),
                std::str::from_utf8(name.local_name()).unwrap()
            ))
        }
    }
}
