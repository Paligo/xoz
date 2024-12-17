use vers_vecs::{BitVec, WaveletMatrix};

use crate::{
    data::{TagId, TagsUsage},
    error::Error,
};

pub(crate) trait TagVec {
    fn get_tag(&self, i: usize) -> Option<TagId>;

    fn rank_tag(&self, i: usize, tag_id: TagId) -> Option<usize>;

    fn select_tag(&self, rank: usize, tag_id: TagId) -> Option<usize>;
}

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

fn make_wavelet_matrix_usage(tags_usage: &TagsUsage) -> Result<WaveletMatrix, Error> {
    let usage = BitVec::pack_sequence_u64(tags_usage.usage(), tags_usage.bits_per_element());
    let bits_per_element: u16 = tags_usage
        .bits_per_element()
        .try_into()
        .map_err(|_| Error::TooManyBitsPerElement)?;
    Ok(WaveletMatrix::from_bit_vec(&usage, bits_per_element))
}
