use sucds::{int_vectors::CompactVector, Serializable};
use vers_vecs::{BitVec, SparseRSVec, WaveletMatrix};

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NodeInfoId(u64);

// we ensure we always register these first in any document
pub(crate) const NAMESPACES_NODE_INFO_ID: NodeInfoId = NodeInfoId(0);
pub(crate) const ATTRIBUTES_NODE_INFO_ID: NodeInfoId = NodeInfoId(1);

impl NodeInfoId {
    pub(crate) fn is_special(&self) -> bool {
        *self == NAMESPACES_NODE_INFO_ID || *self == ATTRIBUTES_NODE_INFO_ID
    }

    pub(crate) fn is_namespaces(&self) -> bool {
        *self == NAMESPACES_NODE_INFO_ID
    }

    pub(crate) fn is_attributes(&self) -> bool {
        *self == ATTRIBUTES_NODE_INFO_ID
    }
}

impl NodeInfoId {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }

    pub(crate) fn id(&self) -> u64 {
        self.0
    }
}
pub(crate) trait NodeInfoVec {
    /// heap size
    fn heap_size(&self) -> usize;

    /// Returns the node info id at position `i`.
    ///
    /// Returns `None` if `i` is out of bounds.
    fn get_node_info_id(&self, i: usize) -> Option<NodeInfoId>;

    // TODO: are out of bound queries something that happens in
    // practice? otherwise this API could be simplified maybe

    /// Returns the number of occurrences of `node_info_id` up to position `i`.
    ///
    /// If `i` is out of bounds, it returns `None`. The length itself is
    /// still considered in bounds.
    fn rank_node_info_id(&self, i: usize, node_info_id: NodeInfoId) -> Option<usize>;

    /// Returns the position of the `rank`-th occurrence of `node_info_id`.
    ///
    /// If the rankth occurrence of `node_info_id` does not exist, it returns `None`
    fn select_node_info_id(&self, rank: usize, node_info_id: NodeInfoId) -> Option<usize>;
}

// A wavelet matrix implementation, based on Vers' wavelet matrix
impl NodeInfoVec for WaveletMatrix {
    fn heap_size(&self) -> usize {
        self.heap_size()
    }

    fn get_node_info_id(&self, i: usize) -> Option<NodeInfoId> {
        self.get_u64(i).map(NodeInfoId::new)
    }

    fn rank_node_info_id(&self, i: usize, node_info_id: NodeInfoId) -> Option<usize> {
        self.rank_u64(i, node_info_id.id())
    }

    fn select_node_info_id(&self, rank: usize, node_info_id: NodeInfoId) -> Option<usize> {
        self.select_u64(rank, node_info_id.id())
    }
}

#[allow(dead_code)]
fn bits_per_element(amount: usize) -> usize {
    amount.next_power_of_two().trailing_zeros() as usize
}

#[allow(dead_code)]
pub(crate) fn make_wavelet_matrix_tag_vec(
    tags_usage: &[u64],
    tag_amount: usize,
) -> Result<WaveletMatrix, Error> {
    let bit_width = bits_per_element(tag_amount);
    let usage = BitVec::pack_sequence_u64(tags_usage, bit_width);
    let bit_width: u16 = bit_width
        .try_into()
        .map_err(|_| Error::TooManyBitsPerElement)?;
    Ok(WaveletMatrix::from_bit_vec(&usage, bit_width))
}

// a sarray-based implementation
// This uses sucds's SArray and CompactVector
#[derive(Debug)]
pub(crate) struct SArrayMatrix {
    tags: CompactVector,
    sarrays: Vec<SparseRSVec>,
    len: usize,
}

impl SArrayMatrix {
    pub(crate) fn new(tags_usage: &[u64], amount: usize) -> Result<SArrayMatrix, Error> {
        // we can unwrap as we know that u64 can be converted to usize
        let tags = CompactVector::from_slice(tags_usage).unwrap();
        let mut sarray_positions: Vec<Vec<u64>> = vec![vec![]; amount];
        for (i, entry) in tags_usage.iter().enumerate() {
            let positions = sarray_positions
                .get_mut(*entry as usize)
                .expect("entry should be present");
            positions.push(i as u64);
        }
        let sarrays = sarray_positions
            .into_iter()
            .map(|positions| SparseRSVec::new(&positions, tags_usage.len() as u64))
            .collect();
        Ok(SArrayMatrix {
            tags,
            sarrays,
            len: tags_usage.len(),
        })
    }
}

impl NodeInfoVec for SArrayMatrix {
    fn heap_size(&self) -> usize {
        self.tags.size_in_bytes() + self.sarrays.iter().map(|s| s.heap_size()).sum::<usize>()
    }

    fn get_node_info_id(&self, i: usize) -> Option<NodeInfoId> {
        self.tags.get_int(i).map(|i| NodeInfoId::new(i as u64))
    }

    fn rank_node_info_id(&self, i: usize, node_info_id: NodeInfoId) -> Option<usize> {
        if i <= self.len {
            Some(self.sarrays[node_info_id.id() as usize].rank1(i as u64) as usize)
        } else {
            None
        }
    }

    fn select_node_info_id(&self, rank: usize, node_info_id: NodeInfoId) -> Option<usize> {
        let s = self.sarrays[node_info_id.id() as usize].select1(rank) as usize;
        if self.len != s {
            Some(s)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wm_get_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 2, 3], 4).unwrap();
        assert_eq!(wm.get_node_info_id(0), Some(NodeInfoId::new(0)));
        assert_eq!(wm.get_node_info_id(1), Some(NodeInfoId::new(1)));
        assert_eq!(wm.get_node_info_id(2), Some(NodeInfoId::new(2)));
        assert_eq!(wm.get_node_info_id(10), None);
    }

    #[test]
    fn test_wm_rank_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.rank_node_info_id(0, NodeInfoId::new(0)), Some(0));
        assert_eq!(wm.rank_node_info_id(1, NodeInfoId::new(0)), Some(1));
        assert_eq!(wm.rank_node_info_id(2, NodeInfoId::new(1)), Some(1));
        assert_eq!(wm.rank_node_info_id(3, NodeInfoId::new(1)), Some(2));
        assert_eq!(wm.rank_node_info_id(6, NodeInfoId::new(3)), Some(2));
        assert_eq!(wm.rank_node_info_id(10, NodeInfoId::new(3)), None);
    }

    #[test]
    fn test_wm_select_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(0)), Some(0));
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(1)), Some(1));
        assert_eq!(wm.select_node_info_id(1, NodeInfoId::new(1)), Some(2));
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(3)), Some(3));
        assert_eq!(wm.select_node_info_id(1, NodeInfoId::new(3)), Some(5));
        assert_eq!(wm.select_node_info_id(2, NodeInfoId::new(3)), None);
    }

    #[test]
    fn test_sa_get_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 2, 3], 4).unwrap();
        assert_eq!(wm.get_node_info_id(0), Some(NodeInfoId::new(0)));
        assert_eq!(wm.get_node_info_id(1), Some(NodeInfoId::new(1)));
        assert_eq!(wm.get_node_info_id(2), Some(NodeInfoId::new(2)));
        assert_eq!(wm.get_node_info_id(10), None);
    }

    #[test]
    fn test_sa_rank_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.rank_node_info_id(0, NodeInfoId::new(0)), Some(0));
        assert_eq!(wm.rank_node_info_id(1, NodeInfoId::new(0)), Some(1));
        assert_eq!(wm.rank_node_info_id(2, NodeInfoId::new(1)), Some(1));
        assert_eq!(wm.rank_node_info_id(3, NodeInfoId::new(1)), Some(2));
        assert_eq!(wm.rank_node_info_id(6, NodeInfoId::new(3)), Some(2));
        assert_eq!(wm.rank_node_info_id(10, NodeInfoId::new(3)), None);
    }

    #[test]
    fn test_sa_select_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(0)), Some(0));
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(1)), Some(1));
        assert_eq!(wm.select_node_info_id(1, NodeInfoId::new(1)), Some(2));
        assert_eq!(wm.select_node_info_id(0, NodeInfoId::new(3)), Some(3));
        assert_eq!(wm.select_node_info_id(1, NodeInfoId::new(3)), Some(5));
        assert_eq!(wm.select_node_info_id(2, NodeInfoId::new(3)), None);
    }
}
