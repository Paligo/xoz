#[cfg(doc)]
use crate::xozdata::Xoz;

use crate::{Namespace, NodeName};

/// Which type of node we are in the XML tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType<'a> {
    /// Document node: the root of an XML document
    Document,
    /// Namespace declaration node.
    ///
    /// Example: `xmlns:prefix="http://example.com"`
    ///
    /// Contains the prefix and namespace URI.
    Namespace(Namespace<'a>),
    /// Attribute node.
    ///
    /// Example: `name="value"`
    ///
    /// Has a node name. Value is in associated text data.
    ///
    /// Access it via [`Xoz::attribute_str`] or [`Xoz::node_str`] if you
    /// already know it's an attribute node.
    Attribute(NodeName<'a>),
    /// Element node.
    ///
    /// Example: `<element>...</element>`
    ///
    /// Has a node name.
    Element(NodeName<'a>),
    /// Text node.
    ///
    /// Example: `This is a text node.`
    ///
    /// Has associated text data. Access it via [`Xoz::text_str`] or
    /// [`Xoz::node_str`] if you already know it's a text node.
    Text,
    /// Comment node.
    ///
    /// Example: `<!-- This is a comment -->`
    ///
    /// Has associated text data. Access it via [`Xoz::comment_str`] or
    /// [`Xoz::node_str`] if you already know it's a comment node..
    Comment,
    /// Processing instruction node.
    ///
    /// Example: `<?target data?>`
    ///
    /// Access contents via [`Xoz::processing_instruction`] or [`Xoz::node_str`] to
    /// get the raw data.
    ProcessingInstruction,
    /// Namespaces holder node.
    ///
    /// Internal marker of all namespace declarations in an element. It should
    /// never occur in the public API.
    Namespaces,
    /// Attributes holder node.
    ///
    /// Internal marker of all attributes in an element. It should never occur
    /// in the public API.
    Attributes,
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

    /// Convenience method to create an attribute node.
    ///
    /// You can pass in a string for an attribute outside of a namespace,
    /// or a `NodeName` if the attribute has a namespace URI.
    pub fn attribute(name: impl Into<NodeName<'a>>) -> Self {
        NodeType::Attribute(name.into())
    }

    /// Convenience method to create an element node.
    ///
    /// You can pass in a string for an element outside of a namespace,
    /// or a `NodeName` if the element has a namespace URI.
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
