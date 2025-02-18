use ahash::{HashMap, HashMapExt};
use vers_vecs::BitVec;

use crate::{
    tag::{TagInfo, TagType},
    tagvec::{TagId, ATTRIBUTES_TAG_ID, NAMESPACES_TAG_ID},
};

pub(crate) struct TagsLookup {
    pub(crate) tags: Vec<TagInfo<'static>>,
    pub(crate) tag_lookup: HashMap<TagInfo<'static>, TagId>,
}

impl TagsLookup {
    pub(crate) fn new() -> Self {
        Self {
            tags: Vec::new(),
            tag_lookup: HashMap::new(),
        }
    }

    fn register<'a>(&mut self, tag_info: TagInfo<'a>) -> TagId {
        if let Some(&idx) = self.tag_lookup.get(&tag_info) {
            return idx;
        }
        let idx = TagId::new(self.tags.len() as u64);
        let owned_tag_info = tag_info.into_owned();
        self.tags.push(owned_tag_info.clone());
        self.tag_lookup.insert(owned_tag_info, idx);
        idx
    }

    pub(crate) fn by_tag_info(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.tag_lookup.get(tag_info).copied()
    }

    pub(crate) fn by_tag_id(&self, tag_id: TagId) -> &TagInfo {
        self.tags
            .get(tag_id.id() as usize)
            .expect("Tag id does not exist in this document")
    }

    pub(crate) fn len(&self) -> usize {
        self.tags.len()
    }
}

pub(crate) struct TagsBuilder {
    pub(crate) tags_lookup: TagsLookup,

    pub(crate) parentheses: BitVec,
    // store the opening parens of all text content, i.e. text nodes
    // and attribute values. We store this separately even though there's
    // some overlap with the tag table as it's more convenient to calculate text id
    pub(crate) text_opening_parens: BitVec,
    // stores tag ids, but as u64 for convenience of later construction
    usage: Vec<u64>,
}

impl TagsBuilder {
    pub(crate) fn new() -> Self {
        let mut tags_lookup = TagsLookup::new();
        // we ensure these always exist, so that we quickly compare with tag id
        let namespaces_tag_id = tags_lookup.register(TagInfo::open(TagType::Namespaces));
        let attributes_tag_id = tags_lookup.register(TagInfo::open(TagType::Attributes));
        debug_assert_eq!(namespaces_tag_id.id(), NAMESPACES_TAG_ID.id());
        debug_assert_eq!(attributes_tag_id.id(), ATTRIBUTES_TAG_ID.id());
        Self {
            tags_lookup,

            parentheses: BitVec::new(),
            text_opening_parens: BitVec::new(),
            usage: Vec::new(),
        }
    }

    fn register_tag(&mut self, tag_info: TagInfo) -> TagId {
        self.tags_lookup.register(tag_info)
    }

    pub(crate) fn bits_per_element(&self) -> usize {
        self.tags_lookup.len().next_power_of_two().trailing_zeros() as usize
    }

    pub(crate) fn tags_amount(&self) -> usize {
        self.tags_lookup.len()
    }

    pub(crate) fn usage(&self) -> &[u64] {
        &self.usage
    }

    pub(crate) fn open(&mut self, tag_type: TagType) {
        self.parentheses.append(true);

        match tag_type {
            TagType::Attribute { .. } | TagType::Text => {
                self.text_opening_parens.append(true);
            }
            _ => {
                self.text_opening_parens.append(false);
            }
        }
        let tag_info = TagInfo::open(tag_type);
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.id())
    }

    pub(crate) fn close(&mut self, tag_type: TagType) {
        self.parentheses.append(false);
        self.text_opening_parens.append(false);
        let tag_info = TagInfo::close(tag_type);
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.id())
    }
}

#[cfg(test)]
mod tests {
    use crate::tag::TagName;

    use super::*;

    #[test]
    fn test_tags_builder() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><b/></doc>
        builder.open(TagType::Element(TagName::new("", "doc")));
        builder.open(TagType::Element(TagName::new("", "a")));
        builder.close(TagType::Element(TagName::new("", "a")));
        builder.open(TagType::Element(TagName::new("", "b")));
        builder.close(TagType::Element(TagName::new("", "b")));
        builder.close(TagType::Element(TagName::new("", "doc")));

        let usage = builder.usage();
        // starts at 2 because of the namespaces and attributes tags
        assert_eq!(usage, &[2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_tags_builder_multiple_a() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><a/></doc>
        builder.open(TagType::Element(TagName::new("", "doc")));
        builder.open(TagType::Element(TagName::new("", "a")));
        builder.close(TagType::Element(TagName::new("", "a")));
        builder.open(TagType::Element(TagName::new("", "a")));
        builder.close(TagType::Element(TagName::new("", "a")));
        builder.close(TagType::Element(TagName::new("", "doc")));

        let usage = builder.usage();
        assert_eq!(usage, &[2, 3, 4, 3, 4, 5]);
    }
}
