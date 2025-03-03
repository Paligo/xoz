use ahash::{HashMap, HashMapExt};
use vers_vecs::BitVec;

use crate::{
    node::{NodeInfo, NodeType},
    node_info_vec::{
        NodeInfoId, ATTRIBUTES_NODE_INFO_CLOSE_ID, ATTRIBUTES_NODE_INFO_OPEN_ID,
        COMMENT_NODE_INFO_CLOSE_ID, COMMENT_NODE_INFO_OPEN_ID, DOCUMENT_NODE_INFO_CLOSE_ID,
        DOCUMENT_NODE_INFO_OPEN_ID, NAMESPACES_NODE_INFO_CLOSE_ID, NAMESPACES_NODE_INFO_OPEN_ID,
        PROCESSING_INSTRUCTION_NODE_INFO_CLOSE_ID, PROCESSING_INSTRUCTION_NODE_INFO_OPEN_ID,
        TEXT_NODE_INFO_CLOSE_ID, TEXT_NODE_INFO_OPEN_ID,
    },
};

#[derive(Debug)]
pub(crate) struct NodeInfoLookup {
    pub(crate) node_infos: Vec<NodeInfo<'static>>,
    pub(crate) node_info_lookup: HashMap<NodeInfo<'static>, NodeInfoId>,
}

impl NodeInfoLookup {
    pub(crate) fn new() -> Self {
        let mut o = Self {
            node_infos: Vec::new(),
            node_info_lookup: HashMap::new(),
        };

        // we ensure the node ids we can recognize quickly without
        // hash lookup are always the same

        let document_node_info_open_id = o.register_hash_map(NodeInfo::open(NodeType::Document));
        debug_assert_eq!(
            document_node_info_open_id.id(),
            DOCUMENT_NODE_INFO_OPEN_ID.id()
        );
        let document_node_info_close_id = o.register_hash_map(NodeInfo::close(NodeType::Document));
        debug_assert_eq!(
            document_node_info_close_id.id(),
            DOCUMENT_NODE_INFO_CLOSE_ID.id()
        );

        let text_node_info_open_id = o.register_hash_map(NodeInfo::open(NodeType::Text));
        debug_assert_eq!(text_node_info_open_id.id(), TEXT_NODE_INFO_OPEN_ID.id());
        let text_node_info_close_id = o.register_hash_map(NodeInfo::close(NodeType::Text));
        debug_assert_eq!(text_node_info_close_id.id(), TEXT_NODE_INFO_CLOSE_ID.id());

        let comment_node_info_open_id = o.register_hash_map(NodeInfo::open(NodeType::Comment));
        debug_assert_eq!(
            comment_node_info_open_id.id(),
            COMMENT_NODE_INFO_OPEN_ID.id()
        );
        let comment_node_info_close_id = o.register_hash_map(NodeInfo::close(NodeType::Comment));
        debug_assert_eq!(
            comment_node_info_close_id.id(),
            COMMENT_NODE_INFO_CLOSE_ID.id()
        );

        let processing_instruction_node_info_open_id =
            o.register_hash_map(NodeInfo::open(NodeType::ProcessingInstruction));
        debug_assert_eq!(
            processing_instruction_node_info_open_id.id(),
            PROCESSING_INSTRUCTION_NODE_INFO_OPEN_ID.id()
        );
        let processing_instruction_node_info_close_id =
            o.register_hash_map(NodeInfo::close(NodeType::ProcessingInstruction));
        debug_assert_eq!(
            processing_instruction_node_info_close_id.id(),
            PROCESSING_INSTRUCTION_NODE_INFO_CLOSE_ID.id()
        );

        let namespaces_node_info_open_id =
            o.register_hash_map(NodeInfo::open(NodeType::Namespaces));
        debug_assert_eq!(
            namespaces_node_info_open_id.id(),
            NAMESPACES_NODE_INFO_OPEN_ID.id()
        );
        let namespaces_node_info_close_id =
            o.register_hash_map(NodeInfo::close(NodeType::Namespaces));
        debug_assert_eq!(
            namespaces_node_info_close_id.id(),
            NAMESPACES_NODE_INFO_CLOSE_ID.id()
        );

        let attributes_node_info_open_id =
            o.register_hash_map(NodeInfo::open(NodeType::Attributes));
        debug_assert_eq!(
            attributes_node_info_open_id.id(),
            ATTRIBUTES_NODE_INFO_OPEN_ID.id()
        );
        let attributes_node_info_close_id =
            o.register_hash_map(NodeInfo::close(NodeType::Attributes));
        debug_assert_eq!(
            attributes_node_info_close_id.id(),
            ATTRIBUTES_NODE_INFO_CLOSE_ID.id()
        );

        o
    }

    pub(crate) fn heap_size(&self) -> usize {
        // TODO: node_info_lookup heap size is tricky as it has a hashmap and contains
        // (owned) cows. it's not going to be most of the data though - the size of the
        // tags and the hashmap itself
        // TODO: we should go through every element in the vector and add the heap
        // size of the cows
        self.node_infos.len() * std::mem::size_of::<NodeInfo>()
    }

    fn register_fast_path(&mut self, node_info: &NodeInfo) -> Option<NodeInfoId> {
        // first we use the fast path
        match (node_info.is_open_tag(), node_info.node_type()) {
            (true, NodeType::Document) => Some(DOCUMENT_NODE_INFO_OPEN_ID),
            (false, NodeType::Document) => Some(DOCUMENT_NODE_INFO_CLOSE_ID),
            (true, NodeType::Text) => Some(TEXT_NODE_INFO_OPEN_ID),
            (false, NodeType::Text) => Some(TEXT_NODE_INFO_CLOSE_ID),
            (true, NodeType::Comment) => Some(COMMENT_NODE_INFO_OPEN_ID),
            (false, NodeType::Comment) => Some(COMMENT_NODE_INFO_CLOSE_ID),
            (true, NodeType::ProcessingInstruction) => {
                Some(PROCESSING_INSTRUCTION_NODE_INFO_OPEN_ID)
            }
            (false, NodeType::ProcessingInstruction) => {
                Some(PROCESSING_INSTRUCTION_NODE_INFO_CLOSE_ID)
            }
            (true, NodeType::Namespaces) => Some(NAMESPACES_NODE_INFO_OPEN_ID),
            (false, NodeType::Namespaces) => Some(NAMESPACES_NODE_INFO_CLOSE_ID),
            (true, NodeType::Attributes) => Some(ATTRIBUTES_NODE_INFO_OPEN_ID),
            (false, NodeType::Attributes) => Some(ATTRIBUTES_NODE_INFO_CLOSE_ID),
            _ => None,
        }
    }

    fn register_hash_map(&mut self, node_info: NodeInfo) -> NodeInfoId {
        if let Some(&idx) = self.node_info_lookup.get(&node_info) {
            return idx;
        }
        let idx = NodeInfoId::new(self.node_infos.len() as u64);
        let owned_node_info = node_info.into_owned();
        self.node_infos.push(owned_node_info.clone());
        self.node_info_lookup.insert(owned_node_info, idx);
        idx
    }

    fn register(&mut self, node_info: NodeInfo) -> NodeInfoId {
        if let Some(idx) = self.register_fast_path(&node_info) {
            return idx;
        }
        self.register_hash_map(node_info)
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

pub(crate) struct TreeBuilder {
    pub(crate) node_info_lookup: NodeInfoLookup,

    pub(crate) parentheses: BitVec,
    // store the opening parens of all text content, i.e. text nodes
    // and attribute values. We store this separately even though there's
    // some overlap with the tag table as it's more convenient to calculate text id
    pub(crate) text_opening_parens: BitVec,
    // stores tag ids, but as u64 for convenience of later construction
    usage: Vec<u64>,
}

impl TreeBuilder {
    pub(crate) fn new() -> Self {
        Self {
            node_info_lookup: NodeInfoLookup::new(),
            parentheses: BitVec::new(),
            text_opening_parens: BitVec::new(),
            usage: Vec::new(),
        }
    }

    fn register_node_info(&mut self, node_info: NodeInfo) -> NodeInfoId {
        self.node_info_lookup.register(node_info)
    }

    pub(crate) fn node_info_amount(&self) -> usize {
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
        let node_info_id = self.register_node_info(node_info);
        self.usage.push(node_info_id.id())
    }

    pub(crate) fn close(&mut self, node_type: NodeType) {
        self.parentheses.append(false);
        self.text_opening_parens.append(false);
        let node_info = NodeInfo::close(node_type);
        let node_info_id = self.register_node_info(node_info);
        self.usage.push(node_info_id.id())
    }
}

#[cfg(test)]
mod tests {
    use crate::name::NodeName;

    use super::*;

    #[test]
    fn test_tags_builder() {
        let mut builder = TreeBuilder::new();
        // <doc><a/><b/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "b")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let usage = builder.usage();
        // starts at 12 because of the namespaces and attributes tags
        assert_eq!(usage, &[12, 13, 14, 15, 16, 17]);
    }

    #[test]
    fn test_tags_builder_multiple_a() {
        let mut builder = TreeBuilder::new();
        // <doc><a/><a/></doc>
        builder.open(NodeType::Element(NodeName::new("", "doc")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.open(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "a")));
        builder.close(NodeType::Element(NodeName::new("", "doc")));

        let usage = builder.usage();
        assert_eq!(usage, &[12, 13, 14, 13, 14, 15]);
    }
}
