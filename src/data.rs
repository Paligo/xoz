use std::os::unix::ffi::OsStringExt;

use ahash::{HashMap, HashMapExt};
use vers_vecs::{trees::bp::BpTree, BitVec, RsVec, WaveletMatrix};

use crate::{
    error::Error,
    tag::{TagInfo, TagType},
    tagvec::TagVec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TagId(u64);

impl TagId {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }

    pub(crate) fn id(&self) -> u64 {
        self.0
    }
}

pub(crate) struct TagsBuilder {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    parentheses: BitVec,
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
        let idx = TagId(self.tags.len() as u64);
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
        self.usage.push(tag_id.0)
    }

    pub(crate) fn close(&mut self, tag_type: TagType) {
        self.parentheses.append(false);
        let tag_info = TagInfo::close(tag_type);
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.0)
    }
}

pub(crate) struct Structure<T: TagVec> {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    tree: BpTree,
    tag_vec: T,
}

impl<T: TagVec> Structure<T> {
    pub(crate) fn new(
        tags_builder: TagsBuilder,
        make_tag_vec: impl Fn(&TagsBuilder) -> Result<T, Error>,
    ) -> Result<Self, Error> {
        let tag_vec = make_tag_vec(&tags_builder)?;
        Ok(Self {
            tags: tags_builder.tags,
            tag_lookup: tags_builder.tag_lookup,
            tree: BpTree::from_bit_vector(tags_builder.parentheses),
            tag_vec,
        })
    }

    /// Given a tag info, return the tag id if it exists
    pub(crate) fn lookup_tag_id(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.tag_lookup.get(tag_info).copied()
    }

    /// Given a tag id, return the tag info.
    ///
    /// Should always succeed given a valid tag info.
    pub(crate) fn lookup_tag_info(&self, tag_id: TagId) -> &TagInfo {
        &self.tags[tag_id.0 as usize]
    }

    pub(crate) fn tree(&self) -> &BpTree {
        &self.tree
    }

    // fn get(&self, i: usize) -> Option<u64> {
    //     self.parentheses.get(i)
    // }

    // fn rank_open(&self, i: usize) -> usize {
    //     self.parentheses.rank1(i)
    // }

    // fn rank_close(&self, i: usize) -> usize {
    //     self.parentheses.rank0(i)
    // }

    // fn select_open(&self, rank: usize) -> usize {
    //     self.parentheses.select1(rank)
    // }

    // fn select_close(&self, rank: usize) -> usize {
    //     self.parentheses.select0(rank)
    // }

    pub(crate) fn get_tag(&self, i: usize) -> Option<&TagInfo> {
        self.tag_vec.get_tag(i).map(|id| self.lookup_tag_info(id))
    }

    // TODO: for efficiency we want to skip lookup_tag_id, so we probably
    // want to make this work in terms of tag_id directly
    pub(crate) fn rank_tag(&self, i: usize, tag_info: &TagInfo) -> Option<usize> {
        let tag_id = self.lookup_tag_id(tag_info)?;
        self.tag_vec.rank_tag(i, tag_id)
    }

    pub(crate) fn select_tag(&self, rank: usize, tag_info: &TagInfo) -> Option<usize> {
        let tag_id = self.lookup_tag_id(tag_info)?;
        self.tag_vec.select_tag(rank, tag_id)
    }
}

#[cfg(test)]
mod tests {}
