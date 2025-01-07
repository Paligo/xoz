use ahash::{HashMap, HashMapExt};
use vers_vecs::BitVec;

use crate::{
    tag::{TagInfo, TagType},
    tagvec::TagId,
};

pub(crate) struct TagsBuilder {
    pub(crate) tags: Vec<TagInfo>,
    pub(crate) tag_lookup: HashMap<TagInfo, TagId>,
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
        Self {
            tags: Vec::new(),
            tag_lookup: HashMap::new(),
            parentheses: BitVec::new(),
            text_opening_parens: BitVec::new(),
            usage: Vec::new(),
        }
    }

    fn register_tag(&mut self, tag: TagInfo) -> TagId {
        if let Some(&idx) = self.tag_lookup.get(&tag) {
            return idx;
        }
        let idx = TagId::new(self.tags.len() as u64);
        self.tags.push(tag.clone());
        self.tag_lookup.insert(tag, idx);
        idx
    }

    pub(crate) fn bits_per_element(&self) -> usize {
        self.tags.len().next_power_of_two().trailing_zeros() as usize
    }

    pub(crate) fn tags_amount(&self) -> usize {
        self.tags.len()
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
        let tag_info = TagInfo::close(tag_type);
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags_builder() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><b/></doc>
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });

        let usage = builder.usage();
        assert_eq!(usage, &[0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_tags_builder_multiple_a() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><a/></doc>
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });

        let usage = builder.usage();
        assert_eq!(usage, &[0, 1, 2, 1, 2, 3]);
    }
}
