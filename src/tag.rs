use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType<'a> {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace(Namespace),
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

impl NodeType<'_> {
    pub(crate) fn into_owned(self) -> NodeType<'static> {
        match self {
            NodeType::Document => NodeType::Document,
            NodeType::Namespaces => NodeType::Namespaces,
            NodeType::Attributes => NodeType::Attributes,
            NodeType::Namespace(namespace) => NodeType::Namespace(namespace.clone()),
            NodeType::Attribute(tag_name) => NodeType::Attribute(tag_name.into_owned()),
            NodeType::Element(tag_name) => NodeType::Element(tag_name.into_owned()),
            NodeType::Text => NodeType::Text,
            NodeType::Comment => NodeType::Comment,
            NodeType::ProcessingInstruction => NodeType::ProcessingInstruction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    prefix: Vec<u8>,
    uri: Vec<u8>,
}

impl Namespace {
    // generically construct from either u8 or string
    pub fn new(prefix: impl AsRef<[u8]>, uri: impl AsRef<[u8]>) -> Self {
        Self {
            prefix: prefix.as_ref().to_vec(),
            uri: uri.as_ref().to_vec(),
        }
    }

    pub fn prefix(&self) -> &[u8] {
        &self.prefix
    }

    pub fn uri(&self) -> &[u8] {
        &self.uri
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeName<'a> {
    namespace: Cow<'a, [u8]>,
    local_name: Cow<'a, [u8]>,
}

impl<'a> From<&'a str> for NodeName<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            namespace: Cow::Borrowed(&[]),
            local_name: Cow::Borrowed(s.as_bytes()),
        }
    }
}

impl<'a> NodeName<'a> {
    pub fn new(namespace: &'a str, local_name: &'a str) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace.as_bytes()),
            local_name: Cow::Borrowed(local_name.as_bytes()),
        }
    }

    pub fn from_u8(namespace: &'a [u8], local_name: &'a [u8]) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace),
            local_name: Cow::Borrowed(local_name),
        }
    }

    pub(crate) fn into_owned(self) -> NodeName<'static> {
        NodeName {
            namespace: Cow::Owned(self.namespace.into_owned()),
            local_name: Cow::Owned(self.local_name.into_owned()),
        }
    }

    pub fn namespace(&self) -> &[u8] {
        &self.namespace
    }

    pub fn local_name(&self) -> &[u8] {
        &self.local_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeInfo<'a> {
    tag_type: NodeType<'a>,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    // open is true
    open_close: bool,
}

impl<'a> NodeInfo<'a> {
    pub fn open(tag_type: NodeType<'a>) -> Self {
        Self {
            tag_type,
            open_close: true,
        }
    }

    pub fn close(tag_type: NodeType<'a>) -> Self {
        Self {
            tag_type,
            open_close: false,
        }
    }

    pub(crate) fn into_owned(self) -> NodeInfo<'static> {
        NodeInfo {
            tag_type: self.tag_type.into_owned(),
            open_close: self.open_close,
        }
    }

    pub(crate) fn tag_type(&self) -> &NodeType {
        &self.tag_type
    }

    pub(crate) fn is_open_tag(&self) -> bool {
        self.open_close
    }

    pub(crate) fn is_close_tag(&self) -> bool {
        !self.open_close
    }
}
