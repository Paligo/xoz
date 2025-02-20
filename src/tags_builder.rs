use ahash::{HashMap, HashMapExt};
use vers_vecs::BitVec;

use crate::{
    node::{NodeInfo, NodeType},
    tagvec::{NodeInfoId, ATTRIBUTES_NODE_INFO_ID, NAMESPACES_NODE_INFO_ID},
};

pub(crate) struct NodeInfoLookup {
    pub(crate) node_infos: Vec<NodeInfo<'static>>,
    pub(crate) node_info_lookup: HashMap<NodeInfo<'static>, NodeInfoId>,
}

impl NodeInfoLookup {
    pub(crate) fn new() -> Self {
        Self {
            node_infos: Vec::new(),
            node_info_lookup: HashMap::new(),
        }
    }

    fn register(&mut self, node_info: NodeInfo) -> NodeInfoId {
        if let Some(&idx) = self.node_info_lookup.get(&node_info) {
            return idx;
        }
        let idx = NodeInfoId::new(self.node_infos.len() as u64);
        let owned_node_info = node_info.into_owned();
        self.node_infos.push(owned_node_info.clone());
        self.node_info_lookup.insert(owned_node_info, idx);
        idx
    }

    pub(crate) fn by_node_info(&self, node_info: &NodeInfo) -> Option<NodeInfoId> {
        self.node_info_lookup.get(node_info).copied()
    }

    pub(crate) fn by_node_info_id(&self, node_info_id: NodeInfoId) -> &NodeInfo {
        self.node_infos
            .get(node_info_id.id() as usize)
            .expect("Tag id does not exist in this document")
    }

    pub(crate) fn len(&self) -> usize {
        self.node_infos.len()
    }
}

pub(crate) struct TagsBuilder {
    pub(crate) node_info_lookup: NodeInfoLookup,

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
        let mut node_info_lookup = NodeInfoLookup::new();
        // we ensure these always exist, so that we quickly compare with tag id
        let namespaces_node_info_id =
            node_info_lookup.register(NodeInfo::open(NodeType::Namespaces));
        let attributes_node_info_id =
            node_info_lookup.register(NodeInfo::open(NodeType::Attributes));
        debug_assert_eq!(namespaces_node_info_id.id(), NAMESPACES_NODE_INFO_ID.id());
        debug_assert_eq!(attributes_node_info_id.id(), ATTRIBUTES_NODE_INFO_ID.id());
        Self {
            node_info_lookup,
            parentheses: BitVec::new(),
            text_opening_parens: BitVec::new(),
            usage: Vec::new(),
        }
    }

    fn register_node_info(&mut self, node_info: NodeInfo) -> NodeInfoId {
        self.node_info_lookup.register(node_info)
    }

    pub(crate) fn bits_per_element(&self) -> usize {
        self.node_info_lookup
            .len()
            .next_power_of_two()
            .trailing_zeros() as usize
    }

    pub(crate) fn tags_amount(&self) -> usize {
        self.node_info_lookup.len()
    }

    pub(crate) fn usage(&self) -> &[u64] {
        &self.usage
    }

    pub(crate) fn open(&mut self, node_type: NodeType) {
        self.parentheses.append(true);

        match node_type {
            NodeType::Attribute { .. } | NodeType::Text => {
                self.text_opening_parens.append(true);
            }
            _ => {
                self.text_opening_parens.append(false);
            }
        }
        let node_info = NodeInfo::open(node_type);
        let tag_id = self.register_node_info(node_info);
        self.usage.push(tag_id.id())
    }

    pub(crate) fn close(&mut self, tag_type: NodeType) {
        self.parentheses.append(false);
        self.text_opening_parens.append(false);
        let tag_info = NodeInfo::close(tag_type);
        let tag_id = self.register_node_info(tag_info);
        self.usage.push(tag_id.id())
    }
}

#[cfg(test)]
mod tests {
    use crate::node::NodeName;

    use super::*;

    #[test]
    fn test_tags_builder() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><b/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let usage = builder.usage();
        // starts at 2 because of the namespaces and attributes tags
        assert_eq!(usage, &[2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_tags_builder_multiple_a() {
        let mut builder = TagsBuilder::new();
        // <doc><a/><a/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let usage = builder.usage();
        assert_eq!(usage, &[2, 3, 4, 3, 4, 5]);
    }
}
