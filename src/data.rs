use ahash::{HashMap, HashMapExt};
use vers_vecs::{trees::bp::BpTree, BitVec, RsVec, WaveletMatrix};

use crate::{error::Error, tagvec::TagVec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum TagType {
    // contains namespaces, elements, other nodes
    Document,
    // holds namespace nodes
    Namespaces,
    // holds attribute nodes
    Attributes,
    // under namespaces
    Namespace { prefix: String, uri: String },
    // under attributes. contains content node
    Attribute { namespace: String, name: String },
    // under document or element
    Element { namespace: String, name: String },
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
    text_open_parentheses: BitVec,
    // stores tag ids, but as u64 for convenience of later construction
    usage: Vec<u64>,
}

impl TagsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tags: Vec::new(),
            tag_lookup: HashMap::new(),
            parentheses: BitVec::new(),
            text_open_parentheses: BitVec::new(),
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
        let tag_info = TagInfo {
            tag_type,
            open_close: true,
        };

        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.0)
    }

    pub(crate) fn close(&mut self, tag_type: TagType) {
        self.parentheses.append(false);
        let tag_info = TagInfo {
            tag_type,
            open_close: false,
        };
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

    pub(crate) fn get_tag(&self, i: usize) -> Option<TagId> {
        self.tag_vec.get_tag(i)
    }

    pub(crate) fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.tag_vec.rank_tag(i, tag_id)
    }

    pub(crate) fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.tag_vec.select_tag(rank, tag_id)
    }
}

#[cfg(test)]
mod tests {}
