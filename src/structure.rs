use std::ops::Range;

use vers_vecs::{
    trees::{bp::BpTree, Tree},
    RsVec,
};

use crate::{
    error::Error,
    tag::NodeInfo,
    tags_builder::{TagsBuilder, TagsLookup},
    tagvec::{TagId, TagVec},
    text::TextId,
};

pub(crate) struct Structure<T: TagVec> {
    tags_lookup: TagsLookup,
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
            tags_lookup: tags_builder.tags_lookup,
            text_opening_parens: RsVec::from_bit_vec(tags_builder.text_opening_parens),
            tree: BpTree::from_bit_vector(tags_builder.parentheses),
            tag_vec,
        })
    }

    /// Given a tag info, return the tag id if it exists
    pub(crate) fn lookup_tag_id(&self, tag_info: &NodeInfo) -> Option<TagId> {
        self.tags_lookup.by_tag_info(tag_info)
    }

    /// Given a tag id, return the tag info.
    ///
    /// Should always succeed given a valid tag info.
    pub(crate) fn lookup_tag_info(&self, tag_id: TagId) -> &NodeInfo {
        self.tags_lookup.by_tag_id(tag_id)
    }

    pub(crate) fn tree(&self) -> &BpTree {
        &self.tree
    }

    pub(crate) fn get_tag(&self, i: usize) -> &NodeInfo {
        let id = self.tag_id(i);
        self.lookup_tag_info(id)
    }

    pub(crate) fn tag_id(&self, i: usize) -> TagId {
        self.tag_vec.get_tag(i).expect("Tag information to exist")
    }

    // get text id based on location, given we already know this location has text
    pub(crate) fn text_id(&self, i: usize) -> TextId {
        let text_id = self.text_opening_parens.rank1(i);
        TextId::new(text_id)
    }

    // paper calls this xml id text
    // TODO: write a test for this inverse operation
    pub(crate) fn text_index(&self, text_id: TextId) -> usize {
        // TODO: is node_index really needed? don't we get the index if we simply do select?
        self.tree()
            .node_index(self.text_opening_parens.select1(text_id.id()))
    }

    pub(crate) fn leaf_number(&self, i: usize) -> usize {
        self.text_opening_parens.rank1(i)
    }

    // TODO: write tests
    pub(crate) fn text_ids(&self, i: usize) -> Range<usize> {
        // TODO: what if i is 0, the root
        let start = self.leaf_number(i - 1) + 1;
        let end = self.leaf_number(self.tree.close(i).unwrap());
        start..end
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

    // The first node (in preorder) labeled tag strictly within the subtree
    // rooted at i. If there is no such node the function returns None.
    pub(crate) fn tagged_descendant(&self, i: usize, tag_id: TagId) -> Option<usize> {
        // Note: the "Fast in-memory XPath search using compressed trees" contains
        // a bug where the i is added to the result of rank, but that doesn't work.
        let d = self.select_tag(self.rank_tag(i + 1, tag_id)?, tag_id)?;
        if d <= self.tree.close(i)? {
            Some(d)
        } else {
            println!("not within close");
            None
        }
    }

    // The last node labeled tag with preorder smaller than that of node i, and
    // not an ancestor of i. Returns None if no such node exists.
    pub(crate) fn tagged_preceding(&self, i: usize, tag_id: TagId) -> Option<usize> {
        todo!()
    }

    // The first node labeled tag with preorder larger than that of node i,
    // and not in the subtree of i. Returns None if there is no such node
    pub(crate) fn tagged_following(&self, i: usize, tag_id: TagId) -> Option<usize> {
        // TODO: no tests yet
        self.select_tag(self.rank_tag(self.tree.close(i)?, tag_id)? + 1, tag_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        tag::{NodeName, NodeType},
        tagvec::SArrayMatrix,
    };

    use super::*;

    #[test]
    fn test_structure() {
        let mut builder = TagsBuilder::new();

        // <doc><a/><b/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(
            structure.get_tag(0),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "doc")))
        );
        assert_eq!(
            structure.get_tag(1),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(2),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(3),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "b")))
        );
        assert_eq!(
            structure.get_tag(4),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "b")))
        );
        assert_eq!(
            structure.get_tag(5),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "doc")))
        );
    }

    #[test]
    fn test_structure_multiple_a() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><a/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(
            structure.get_tag(0),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "doc")))
        );
        assert_eq!(
            structure.get_tag(1),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(2),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(3),
            &NodeInfo::open(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(4),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "a")))
        );
        assert_eq!(
            structure.get_tag(5),
            &NodeInfo::close(NodeType::Element(NodeName::new("", "doc")))
        );
    }

    #[test]
    fn test_structure_multiple_text() {
        // <doc><a>A</a><b>B</b>/doc>
        let mut builder = TagsBuilder::new();
        // 0
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        // 1
        builder.open(NodeType::Element(NodeName::new("", "a")));
        // 2
        builder.open(NodeType::Text);
        // 3
        builder.close(NodeType::Text);
        // 4
        builder.close(NodeType::Element(NodeName::new("", "a")));
        // 5
        builder.open(NodeType::Element(NodeName::new("", "b")));
        // 6
        builder.open(NodeType::Text);
        // 7
        builder.close(NodeType::Text);
        // 8
        builder.close(NodeType::Element(NodeName::new("", "b")));
        // 9
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let structure = Structure::new(builder, |builder| {
            SArrayMatrix::new(builder.usage(), builder.tags_amount())
        })
        .unwrap();

        assert_eq!(structure.text_id(2).id(), 0);
        assert_eq!(structure.text_id(6).id(), 1);
    }
}
