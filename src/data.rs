use ahash::{HashMap, HashMapExt};
use vers_vecs::{trees::bp::BpTree, BitVec, RsVec, WaveletMatrix};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TagType {
    Root,
    Namespaces,
    Attributes,
    Text,
    Comment,
    // TODO: this might have name information too
    ProcessingInstruction,
    Element { namespace: String, name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TagInfo {
    tag_type: TagType,
    // this would seem to be redundant as we already store it in the
    // balanced parentheses structure, but we want to be able to
    // look quickly for specifically opening tags, so we need it
    open_close: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TagId(u64);

struct TagsUsage {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    parentheses: BitVec,
    // stores tag ids, but as u64 for convenience of later construction
    usage: Vec<u64>,
}

impl TagsUsage {
    fn new() -> Self {
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

    fn bits_per_element(&self) -> usize {
        self.tags.len().next_power_of_two().trailing_zeros() as usize
    }

    fn open(&mut self, tag_type: TagType) {
        self.parentheses.append(true);
        let tag_info = TagInfo {
            tag_type,
            open_close: true,
        };
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.0)
    }

    fn close(&mut self, tag_type: TagType) {
        self.parentheses.append(false);
        let tag_info = TagInfo {
            tag_type,
            open_close: false,
        };
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id.0)
    }
}

enum Error {
    TooManyBitsPerElement,
}

pub(crate) struct Structure {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    tree: BpTree,
    usage: WaveletMatrix,
}

impl Structure {
    pub(crate) fn new(tags_usage: TagsUsage) -> Result<Self, Error> {
        let usage = BitVec::pack_sequence_u64(&tags_usage.usage, tags_usage.bits_per_element());
        // fallible conversion of usize to u16
        let bits_per_element: u16 = tags_usage
            .bits_per_element()
            .try_into()
            .map_err(|_| Error::TooManyBitsPerElement)?;
        let usage = WaveletMatrix::from_bit_vec(&usage, bits_per_element);
        Ok(Self {
            tags: tags_usage.tags,
            tag_lookup: tags_usage.tag_lookup,
            tree: BpTree::from_bit_vector(tags_usage.parentheses),
            usage,
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
        self.usage.get_u64(i).map(TagId)
    }

    pub(crate) fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.usage.rank_u64(i, tag_id.0)
    }

    pub(crate) fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.usage.select_u64(rank, tag_id.0)
    }
}

#[cfg(test)]
mod tests {}
