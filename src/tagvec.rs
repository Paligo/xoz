use sucds::{
    bit_vectors::{Rank, SArray, Select},
    int_vectors::CompactVector,
};
use vers_vecs::{BitVec, WaveletMatrix};

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagId(u64);

// we ensure we always register these first in any document
pub(crate) const NAMESPACES_TAG_ID: TagId = TagId(0);
pub(crate) const ATTRIBUTES_TAG_ID: TagId = TagId(1);

impl TagId {
    pub(crate) fn is_special(&self) -> bool {
        *self == NAMESPACES_TAG_ID || *self == ATTRIBUTES_TAG_ID
    }

    pub(crate) fn is_namespaces(&self) -> bool {
        *self == NAMESPACES_TAG_ID
    }

    pub(crate) fn is_attributes(&self) -> bool {
        *self == ATTRIBUTES_TAG_ID
    }
}

impl TagId {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }

    pub(crate) fn id(&self) -> u64 {
        self.0
    }
}
pub(crate) trait TagVec {
    /// Returns the tag at position `i`.
    ///
    /// Returns `None` if `i` is out of bounds.
    fn get_tag(&self, i: usize) -> Option<TagId>;

    /// Returns the number of occurrences of `tag_id` up to position `i`.
    ///
    /// If `i` is out of bounds, it returns `None`.
    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize>;

    /// Returns the position of the `rank`-th occurrence of `tag_id`.
    ///
    /// Of the rankth occurrence of `tag_id` does not exist, it returns `None`
    fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize>;
}

// A wavelet matrix implementation, based on Vers' wavelet matrix
impl TagVec for WaveletMatrix {
    fn get_tag(&self, i: usize) -> Option<TagId> {
        self.get_u64(i).map(TagId::new)
    }

    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.rank_u64(i, tag_id.id())
    }

    fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.select_u64(rank, tag_id.id())
    }
}

fn bits_per_element(amount: usize) -> usize {
    amount.next_power_of_two().trailing_zeros() as usize
}

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
pub(crate) struct SArrayMatrix {
    tags: CompactVector,
    sarrays: Vec<SArray>,
}

impl SArrayMatrix {
    pub(crate) fn new(tags_usage: &[u64], amount: usize) -> Result<SArrayMatrix, Error> {
        // we can unwrap as we know that u64 can be converted to usize
        let tags = CompactVector::from_slice(tags_usage).unwrap();
        let sarrays = (0..amount)
            .map(|id| {
                SArray::from_bits(BitIterator {
                    tags_usage,
                    id: id as u64,
                    index: 0,
                })
                .enable_rank()
            })
            .collect();
        Ok(SArrayMatrix { tags, sarrays })
    }
}

impl TagVec for SArrayMatrix {
    fn get_tag(&self, i: usize) -> Option<TagId> {
        self.tags.get_int(i).map(|i| TagId::new(i as u64))
    }

    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.sarrays[tag_id.id() as usize].rank1(i)
    }

    fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize> {
        self.sarrays[tag_id.id() as usize].select1(rank)
    }
}

struct BitIterator<'a> {
    tags_usage: &'a [u64],
    id: u64,
    index: usize,
}

impl Iterator for BitIterator<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tags_usage.len() {
            return None;
        }
        let bit = self.tags_usage[self.index] == self.id;
        self.index += 1;
        Some(bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wm_get_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 2, 3], 4).unwrap();
        assert_eq!(wm.get_tag(0), Some(TagId::new(0)));
        assert_eq!(wm.get_tag(1), Some(TagId::new(1)));
        assert_eq!(wm.get_tag(2), Some(TagId::new(2)));
        assert_eq!(wm.get_tag(10), None);
    }

    #[test]
    fn test_wm_rank_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.rank_tag(0, TagId::new(0)), Some(0));
        assert_eq!(wm.rank_tag(1, TagId::new(0)), Some(1));
        assert_eq!(wm.rank_tag(2, TagId::new(1)), Some(1));
        assert_eq!(wm.rank_tag(3, TagId::new(1)), Some(2));
        assert_eq!(wm.rank_tag(6, TagId::new(3)), Some(2));
        assert_eq!(wm.rank_tag(10, TagId::new(3)), None);
    }

    #[test]
    fn test_wm_select_tag() {
        let wm = make_wavelet_matrix_tag_vec(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.select_tag(0, TagId::new(0)), Some(0));
        assert_eq!(wm.select_tag(0, TagId::new(1)), Some(1));
        assert_eq!(wm.select_tag(1, TagId::new(1)), Some(2));
        assert_eq!(wm.select_tag(0, TagId::new(3)), Some(3));
        assert_eq!(wm.select_tag(1, TagId::new(3)), Some(5));
        assert_eq!(wm.select_tag(2, TagId::new(3)), None);
    }

    #[test]
    fn test_sa_get_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 2, 3], 4).unwrap();
        assert_eq!(wm.get_tag(0), Some(TagId::new(0)));
        assert_eq!(wm.get_tag(1), Some(TagId::new(1)));
        assert_eq!(wm.get_tag(2), Some(TagId::new(2)));
        assert_eq!(wm.get_tag(10), None);
    }

    #[test]
    fn test_sa_rank_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.rank_tag(0, TagId::new(0)), Some(0));
        assert_eq!(wm.rank_tag(1, TagId::new(0)), Some(1));
        assert_eq!(wm.rank_tag(2, TagId::new(1)), Some(1));
        assert_eq!(wm.rank_tag(3, TagId::new(1)), Some(2));
        assert_eq!(wm.rank_tag(6, TagId::new(3)), Some(2));
        assert_eq!(wm.rank_tag(10, TagId::new(3)), None);
    }

    #[test]
    fn test_sa_select_tag() {
        let wm = SArrayMatrix::new(&[0, 1, 1, 3, 2, 3], 4).unwrap();
        assert_eq!(wm.select_tag(0, TagId::new(0)), Some(0));
        assert_eq!(wm.select_tag(0, TagId::new(1)), Some(1));
        assert_eq!(wm.select_tag(1, TagId::new(1)), Some(2));
        assert_eq!(wm.select_tag(0, TagId::new(3)), Some(3));
        assert_eq!(wm.select_tag(1, TagId::new(3)), Some(5));
        assert_eq!(wm.select_tag(2, TagId::new(3)), None);
    }
}
