use ahash::{HashMap, HashMapExt};
use vers_vecs::{trees::bp::BpTree, BitVec, RsVec, WaveletMatrix};

use crate::{
    error::Error,
    tag::{TagInfo, TagType},
    tagvec::{TagId, TagVec},
};

pub(crate) struct TagsBuilder {
    pub(crate) tags: Vec<TagInfo>,
    pub(crate) tag_lookup: HashMap<TagInfo, TagId>,
    pub(crate) parentheses: BitVec,
    // stores tag ids, but as u64 for convenience of later construction
    usage: Vec<u64>,
}

impl TagsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tags: Vec::new(),
            tag_lookup: HashMap::new(),
            parentheses: BitVec::new(),
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
mod tests {}
