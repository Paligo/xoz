use vers_vecs::{BitVec, WaveletMatrix};

use crate::{
    data::{TagId, TagsUsage},
    error::Error,
};

pub(crate) trait TagVec {
    /// Returns the tag at position `i`.
    ///
    /// Returns `None` if `i` is out of bounds.
    fn get_tag(&self, i: usize) -> Option<TagId>;

    /// Returns the number of occurrences of `tag_id` up to position `i`.
    ///
    /// If `i` is out of bounds, it returns `None`.
    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize>;

    /// Returns the position of the `rank`-th occurrence of `tag_id`, starting from `offset`.
    ///
    /// If `offset` is out of bounds, or if the rankkth occurrence of `tag_id` does not exist,
    /// it returns `None`
    fn select_tag(&self, offset: usize, rank: usize, tag_id: TagId) -> Option<usize>;
}

impl TagVec for WaveletMatrix {
    fn get_tag(&self, i: usize) -> Option<TagId> {
        self.get_u64(i).map(TagId::new)
    }

    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize> {
        self.rank_u64(i, tag_id.id())
    }

    fn select_tag(&self, offset: usize, rank: usize, tag_id: TagId) -> Option<usize> {
        self.select_offset_u64(offset, rank, tag_id.id())
    }
}

pub(crate) fn make_wavelet_matrix_usage(tags_usage: &[u64]) -> Result<WaveletMatrix, Error> {
    let bits_per_element = tags_usage.len().next_power_of_two().trailing_zeros() as usize;
    let usage = BitVec::pack_sequence_u64(tags_usage, bits_per_element);
    let bits_per_element: u16 = bits_per_element
        .try_into()
        .map_err(|_| Error::TooManyBitsPerElement)?;
    Ok(WaveletMatrix::from_bit_vec(&usage, bits_per_element))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wm_get_tag() {
        let wm = make_wavelet_matrix_usage(&[0, 1, 2, 3]).unwrap();
        assert_eq!(wm.get_tag(0), Some(TagId::new(0)));
        assert_eq!(wm.get_tag(1), Some(TagId::new(1)));
        assert_eq!(wm.get_tag(2), Some(TagId::new(2)));
        assert_eq!(wm.get_tag(10), None);
    }

    #[test]
    fn test_wm_rank_tag() {
        let wm = make_wavelet_matrix_usage(&[0, 1, 1, 3, 2, 3]).unwrap();
        assert_eq!(wm.rank_tag(0, TagId::new(0)), Some(0));
        assert_eq!(wm.rank_tag(1, TagId::new(0)), Some(1));
        assert_eq!(wm.rank_tag(2, TagId::new(1)), Some(1));
        assert_eq!(wm.rank_tag(3, TagId::new(1)), Some(2));
        assert_eq!(wm.rank_tag(6, TagId::new(3)), Some(2));
        assert_eq!(wm.rank_tag(10, TagId::new(3)), None);
    }

    #[test]
    fn test_wm_select_tag() {
        let wm = make_wavelet_matrix_usage(&[0, 1, 1, 3, 2, 3]).unwrap();
        assert_eq!(wm.select_tag(0, 0, TagId::new(0)), Some(0));
        assert_eq!(wm.select_tag(0, 1, TagId::new(0)), None);
        assert_eq!(wm.select_tag(1, 0, TagId::new(0)), None);
        assert_eq!(wm.select_tag(0, 0, TagId::new(1)), Some(1));
        assert_eq!(wm.select_tag(0, 1, TagId::new(1)), Some(2));
        assert_eq!(wm.select_tag(0, 0, TagId::new(3)), Some(3));
        assert_eq!(wm.select_tag(0, 1, TagId::new(3)), Some(5));
        assert_eq!(wm.select_tag(0, 2, TagId::new(3)), None);
        assert_eq!(wm.select_tag(10, 0, TagId::new(3)), None);
    }
}
