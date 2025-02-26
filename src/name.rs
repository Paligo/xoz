use std::borrow::Cow;

/// A namespace declaration.
///
/// This consists of a prefix and the namespace URI it maps to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    prefix: Vec<u8>,
    uri: Vec<u8>,
}

impl Namespace {
    /// Create a new namespace declaration node, either from u8 or from &str
    pub fn new(prefix: impl AsRef<[u8]>, uri: impl AsRef<[u8]>) -> Self {
        Self {
            prefix: prefix.as_ref().to_vec(),
            uri: uri.as_ref().to_vec(),
        }
    }

    /// The namespace prefix. This is represented as a bytes slice.
    pub fn prefix(&self) -> &[u8] {
        &self.prefix
    }

    /// The namespace URI. This is represented as a bytes slice.
    pub fn uri(&self) -> &[u8] {
        &self.uri
    }
}

/// The name of a node.
///
/// This consists of the local name and the namespace URI used.
///
/// This struct has been designed to be efficiently cloned.
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
    /// Construct a new NodeName from a namespace URI and a local name.
    ///
    /// This borrows the input strings.
    pub fn new(namespace: &'a str, local_name: &'a str) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace.as_bytes()),
            local_name: Cow::Borrowed(local_name.as_bytes()),
        }
    }

    /// Construct a new NodeName from namespace URI bytes and local name bytes.
    ///
    /// This borrows the input slices.
    pub fn from_bytes(namespace: &'a [u8], local_name: &'a [u8]) -> Self {
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

    /// The namespace URI. This is represented as a bytes slice.
    pub fn namespace(&self) -> &[u8] {
        &self.namespace
    }

    /// The local name. This is represented as a bytes slice.
    pub fn local_name(&self) -> &[u8] {
        &self.local_name
    }
}
