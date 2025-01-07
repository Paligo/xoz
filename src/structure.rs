use ahash::HashMap;
use vers_vecs::{trees::bp::BpTree, RsVec};

use crate::{
    error::Error,
    tag::TagInfo,
    tags_builder::TagsBuilder,
    tagvec::{TagId, TagVec},
    text::TextId,
};

pub(crate) struct Structure<T: TagVec> {
    tags: Vec<TagInfo>,
    tag_lookup: HashMap<TagInfo, TagId>,
    text_opening_parens: RsVec,
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
            text_opening_parens: RsVec::from_bit_vec(tags_builder.text_opening_parens),
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

    // get text id based on location, given we already know this location has text
    pub(crate) fn text_id(&self, i: usize) -> TextId {
        let text_id = self.text_opening_parens.rank1(i);
        TextId::new(text_id)
    }

    pub(crate) fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.tag_vec.rank_tag(i, tag_id)
    }

    pub(crate) fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.tag_vec.select_tag(rank, tag_id)
    }

    // the number of occurrences of tag within the subtree rooted at i
    pub(crate) fn subtree_tags(&self, i: usize, tag_id: TagId) -> Option<usize> {
        if i == 0 {
            // root node has no parent
            Some(self.rank_tag(self.tree.close(i)?, tag_id)?)
        } else {
            Some(self.rank_tag(self.tree.close(i)?, tag_id)? - (self.rank_tag(i - 1, tag_id)?))
        }
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

    #[test]
    fn test_structure_multiple_text() {
        // <doc><a>A</a><b>B</b>/doc>
        let mut builder = TagsBuilder::new();
        // 0
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });
        // 1
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        // 2
        builder.open(TagType::Text);
        // 3
        builder.close(TagType::Text);
        // 4
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        });
        // 5
        builder.open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        });
        // 6
        builder.open(TagType::Text);
        // 7
        builder.close(TagType::Text);
        // 8
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        });
        // 9
        builder.close(TagType::Element {
            namespace: "".to_string(),
            local_name: "doc".to_string(),
        });

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(structure.text_id(2).id(), 0);
        assert_eq!(structure.text_id(6).id(), 1);
    }
}
