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
        // for now level > 0 leads to bugs
        // a higher level allows for more compressed storage but less
        // efficient search
        // https://github.com/ajalab/fm-index/issues/24
        let sampler = SuffixOrderSampler::new().level(0);

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

    pub(crate) fn starts_with(&self, pattern: &str) -> Vec<usize> {
        // find those text indices that start with the pattern
        // this means i - 1 is \0, or alternative i == 0
        // TODO: an alternative implementation would look for \0pattern, but
        // would need special handling for the first section, plus allocation,
        // so not sure it's faster
        self.locate(pattern)
            .into_iter()
            .filter(|&i| i == 0 || dbg!(self.text.as_bytes()[i - 1]) == 0)
            .collect()
    }

    pub(crate) fn ends_with(&self, pattern: &str) -> Vec<usize> {
        // find those text indices that are followed by \0
        self.locate(pattern)
            .into_iter()
            .filter(|&i| self.text.as_bytes()[i + pattern.len()] == 0)
            .collect()
    }

    pub(crate) fn equals(&self, pattern: &str) -> Vec<usize> {
        // find those text indices that are surrounded by \0
        self.locate(pattern)
            .into_iter()
            .filter(|&i| {
                let bytes = self.text.as_bytes();
                (i == 0 || bytes[i - 1] == 0) && bytes[i + pattern.len()] == 0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate() {
        let text = "hello world\0";
        let search = TextSearch::new(text.to_string());
        assert_eq!(search.locate("world"), vec![6]);
    }

    #[test]
    fn test_locate_multi() {
        let text = "hello world hello\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.locate("hello");
        located.sort();
        assert_eq!(located, vec![0, 12]);
    }

    #[test]
    fn test_locate_multi_null() {
        let text = "hello\0world hello\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.locate("hello");
        located.sort();
        assert_eq!(located, vec![0, 12]);
    }

    #[test]
    fn test_locate_middle() {
        let text = "world\0hello world\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.locate("hello");
        located.sort();
        assert_eq!(located, vec![6]);
    }

    #[test]
    fn test_starts_with_beginning() {
        let text = "hello\0world hello\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.starts_with("hello");
        located.sort();
        assert_eq!(located, vec![0]);
    }

    #[test]
    fn test_starts_with_middle() {
        let text = "world\0hello world\0";
        let search = TextSearch::new(text.to_string());
        dbg!(search.locate("hello"));
        let mut located = search.starts_with("hello");
        located.sort();
        assert_eq!(located, vec![6]);
    }
}
