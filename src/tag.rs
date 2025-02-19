use std::{borrow::Cow, hash::Hasher};

use quick_xml::name::QName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType<'a> {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace(Namespace),
    // under attributes. has associated text
    Attribute(TagName<'a>),
    // under document or element
    Element(TagName<'a>),
    // under document or element. has associated text
    Text,
    // since there are going to be a limited amount of prefix
    // declarations, we directly encode them as a tag type
    Comment,
    // TODO: this might have name information too
    ProcessingInstruction,
}

impl TagType<'_> {
    pub(crate) fn into_owned(self) -> TagType<'static> {
        match self {
            TagType::Document => TagType::Document,
            TagType::Namespaces => TagType::Namespaces,
            TagType::Attributes => TagType::Attributes,
            TagType::Namespace(namespace) => TagType::Namespace(namespace.clone()),
            TagType::Attribute(tag_name) => TagType::Attribute(tag_name.into_owned()),
            TagType::Element(tag_name) => TagType::Element(tag_name.into_owned()),
            TagType::Text => TagType::Text,
            TagType::Comment => TagType::Comment,
            TagType::ProcessingInstruction => TagType::ProcessingInstruction,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName<'a> {
    namespace: Cow<'a, [u8]>,
    local_name: Cow<'a, [u8]>,
}

impl<'a> TagName<'a> {
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

    pub(crate) fn into_owned(self) -> TagName<'static> {
        TagName {
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
pub struct TagInfo<'a> {
    tag_type: TagType<'a>,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    // open is true
    open_close: bool,
}

impl<'a> TagInfo<'a> {
    pub fn open(tag_type: TagType<'a>) -> Self {
        Self {
            tag_type,
            open_close: true,
        }
    }

    pub fn close(tag_type: TagType<'a>) -> Self {
        Self {
            tag_type,
            open_close: false,
        }
    }

    pub(crate) fn into_owned(self) -> TagInfo<'static> {
        TagInfo {
            tag_type: self.tag_type.into_owned(),
            open_close: self.open_close,
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
