use std::borrow::Cow;

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
