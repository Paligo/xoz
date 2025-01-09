use fm_index::{
    converter::IdConverter,
    suffix_array::{SuffixOrderSampledArray, SuffixOrderSampler},
    BackwardSearchIndex, FMIndex,
};

pub(crate) struct TextSearch {
    pub(crate) index: FMIndex<u8, IdConverter, SuffixOrderSampledArray>,
}

impl TextSearch {
    pub(crate) fn new(text: &str) -> TextSearch {
        let converter = IdConverter::new(256);
        let sampler = SuffixOrderSampler::new().level(2);
        dbg!(text);
        Self {
            index: FMIndex::new(text.as_bytes().to_vec(), converter, sampler),
        }
    }
}
