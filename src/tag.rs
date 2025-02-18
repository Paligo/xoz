#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace(Namespace),
    // under attributes. has associated text
    Attribute(TagName),
    // under document or element
    Element(TagName),
    // under document or element. has associated text
    Text,
    // since there are going to be a limited amount of prefix
    // declarations, we directly encode them as a tag type
    Comment,
    // TODO: this might have name information too
    ProcessingInstruction,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName {
    namespace: Vec<u8>,
    local_name: Vec<u8>,
}

impl TagName {
    // generically construct from either u8 or string
    pub fn new(namespace: impl AsRef<[u8]>, local_name: impl AsRef<[u8]>) -> Self {
        Self {
            namespace: namespace.as_ref().to_vec(),
            local_name: local_name.as_ref().to_vec(),
        }
    }

    pub fn namespace(&self) -> &[u8] {
        &self.namespace
    }

    pub fn local_name(&self) -> &[u8] {
        &self.local_name
    }
}

// TODO: this is an ugly conversion, it'd be nicer if we just stored the u8 vecs
fn to_string(bytes: impl AsRef<[u8]>) -> String {
    std::str::from_utf8(bytes.as_ref()).unwrap().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagInfo {
    tag_type: TagType,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    // open is true
    open_close: bool,
}

impl TagInfo {
    pub fn open(tag_type: TagType) -> Self {
        Self {
            tag_type,
            open_close: true,
        }
    }

    pub fn close(tag_type: TagType) -> Self {
        Self {
            tag_type,
            open_close: false,
        }
    }

    pub(crate) fn tag_type(&self) -> &TagType {
        &self.tag_type
    }

    pub(crate) fn is_open_tag(&self) -> bool {
        self.open_close
    }

    pub(crate) fn is_close_tag(&self) -> bool {
        !self.open_close
    }
}
