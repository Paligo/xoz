#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace {
        prefix: String,
        uri: String,
    },
    // under attributes. contains content node
    Attribute {
        namespace: String,
        local_name: String,
    },
    // under document or element
    Element {
        namespace: String,
        local_name: String,
    },
    // under document or element. contains content
    Text,
    // since there are going to be a limited amount of prefix
    // declarations, we directly encode them as a tag type
    Comment,
    // TODO: this might have name information too
    ProcessingInstruction,
    // text content node
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TagInfo {
    tag_type: TagType,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    // open is true
    open_close: bool,
}

impl TagInfo {
    pub(crate) fn open(tag_type: TagType) -> Self {
        Self {
            tag_type,
            open_close: true,
        }
    }

    pub(crate) fn close(tag_type: TagType) -> Self {
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
