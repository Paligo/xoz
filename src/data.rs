use ahash::HashMap;
use vers_vecs::trees::bp::BpTree;

use crate::{
    error::Error,
    tag::TagInfo,
    tags_builder::TagsBuilder,
    tagvec::{TagId, TagVec},
};

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
        &self.tags[tag_id.id() as usize]
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
mod tests {
    use crate::{tag::TagType, tagvec::SArrayMatrix};

    use super::*;

    #[test]
    fn test_structure() {
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

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(
            structure.get_tag(0),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "doc".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(1),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(2),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(3),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "b".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(4),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "b".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(5),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "doc".to_string()
            }))
        );
    }

    #[test]
    fn test_structure_multiple_a() {
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

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(
            structure.get_tag(0),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "doc".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(1),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(2),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(3),
            Some(&TagInfo::open(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(4),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "a".to_string()
            }))
        );
        assert_eq!(
            structure.get_tag(5),
            Some(&TagInfo::close(TagType::Element {
                namespace: "".to_string(),
                local_name: "doc".to_string()
            }))
        );
    }
}
