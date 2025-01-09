use fm_index::{
    converter::IdConverter,
    suffix_array::{SuffixOrderSampledArray, SuffixOrderSampler},
    BackwardSearchIndex, FMIndex,
};

pub(crate) struct TextSearch {
    text: String,
    pub(crate) index: FMIndex<u8, IdConverter, SuffixOrderSampledArray>,
    is_tiny: bool,
}

impl TextSearch {
    pub(crate) fn new(text: String) -> TextSearch {
        let converter = IdConverter::new(256);

        let is_tiny = text.len() < 5;
        let sampler = SuffixOrderSampler::new().level(2);

        // If the text length is tiny, FMIndex starts to break down, so
        // use a workaround
        // https://github.com/ajalab/fm-index/issues/22
        // https://github.com/ajalab/fm-index/issues/23
        // just search in the text itself directly
        let index = if is_tiny {
            // construct a dummy text index where we know construction will succeed
            // we don't use it
            FMIndex::new("dummy text".as_bytes().to_vec(), converter, sampler)
        } else {
            FMIndex::new(text.as_bytes().to_vec(), converter, sampler)
        };
        Self {
            text,
            index,
            is_tiny,
        }
    }

    pub(crate) fn text_in_range(&self, range: std::ops::Range<usize>) -> &str {
        &self.text[range]
    }

    pub(crate) fn locate(&self, pattern: &str) -> Vec<usize> {
        if self.is_tiny {
            // if it's tiny we can't use the text index, so just search directly
            return self.text.match_indices(pattern).map(|(i, _)| i).collect();
        }
        self.index
            .search_backward(pattern.as_bytes())
            .locate()
            .iter()
            .map(|i| {
                let i: usize = (*i).try_into().unwrap();
                i
            })
            .collect()
    }
}
