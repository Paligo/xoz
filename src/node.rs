use crate::{Namespace, NodeName};

/// Which type of node we are in the XML tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType<'a> {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace(Namespace<'a>),
    // under attributes. has associated text
    Attribute(NodeName<'a>),
    // under document. contains namespaces, attributes, children
    Element(NodeName<'a>),
    // child node, has associated text
    Text,
    // child node, has associated text
    Comment,
    // child node, has associated text
    ProcessingInstruction,
}

impl<'a> NodeType<'a> {
    pub(crate) fn into_owned(self) -> NodeType<'static> {
        match self {
            NodeType::Document => NodeType::Document,
            NodeType::Namespaces => NodeType::Namespaces,
            NodeType::Attributes => NodeType::Attributes,
            NodeType::Namespace(namespace) => NodeType::Namespace(namespace.into_owned()),
            NodeType::Attribute(node_name) => NodeType::Attribute(node_name.into_owned()),
            NodeType::Element(node_name) => NodeType::Element(node_name.into_owned()),
            NodeType::Text => NodeType::Text,
            NodeType::Comment => NodeType::Comment,
            NodeType::ProcessingInstruction => NodeType::ProcessingInstruction,
        }
    }

    pub fn attribute(name: impl Into<NodeName<'a>>) -> Self {
        NodeType::Attribute(name.into())
    }

    pub fn element(name: impl Into<NodeName<'a>>) -> Self {
        NodeType::Element(name.into())
    }
}

/// Information about a node in the document.
///
/// It's a combination of [`NodeType`] and whether it's an opening or closing tag.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeInfo<'a> {
    node_type: NodeType<'a>,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    // open is true
    open_close: bool,
}

impl<'a> NodeInfo<'a> {
    pub(crate) fn open(node_type: NodeType<'a>) -> Self {
        Self {
            node_type,
            open_close: true,
        }
    }

    pub(crate) fn close(node_type: NodeType<'a>) -> Self {
        Self {
            node_type,
            open_close: false,
        }
    }

    pub(crate) fn into_owned(self) -> NodeInfo<'static> {
        NodeInfo {
            node_type: self.node_type.into_owned(),
            open_close: self.open_close,
        }
    }

    pub(crate) fn node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub(crate) fn is_open_tag(&self) -> bool {
        self.open_close
    }
}
