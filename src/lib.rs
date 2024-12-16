use ahash::{HashMap, HashMapExt};
use vers_vecs::{BitVec, RsVec, WaveletMatrix};

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

type TagId = u64;

struct TagsUsage {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    parentheses: BitVec,
    usage: Vec<TagId>,
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
        let idx = self.tags.len() as u64;
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
        self.usage.push(tag_id)
    }

    fn close(&mut self, tag_type: TagType) {
        self.parentheses.append(false);
        let tag_info = TagInfo {
            tag_type,
            open_close: false,
        };
        let tag_id = self.register_tag(tag_info);
        self.usage.push(tag_id)
    }
}

struct Tags {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    parentheses: RsVec,
    usage: WaveletMatrix,
}

enum Error {
    TooManyBitsPerElement,
}

impl Tags {
    fn new(tags_usage: TagsUsage) -> Result<Self, Error> {
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
            parentheses: RsVec::from_bit_vec(tags_usage.parentheses),
            usage,
        })
    }

    fn tag_id(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.tag_lookup.get(tag_info).copied()
    }

    fn tag_info(&self, tag_id: TagId) -> Option<&TagInfo> {
        self.tags.get(tag_id as usize)
    }

    fn rank(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.usage.rank_u64(i, tag_id)
    }

    fn select(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.usage.select_u64(rank, tag_id)
    }
}

#[cfg(test)]
mod tests {}
