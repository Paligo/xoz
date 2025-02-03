use fm_index::{
    converter::IdConverter, suffix_array::SuffixOrderSampledArray, FMIndex, SearchIndexBuilder,
};

pub(crate) struct TextSearch {
    text: String,
    pub(crate) index: FMIndex<u8, IdConverter, SuffixOrderSampledArray>,
    is_tiny: bool,
}

impl TextSearch {
    pub(crate) fn new(text: String) -> TextSearch {
        let is_tiny = text.len() < 5;
        // for now level > 0 leads to bugs
        // a higher level allows for more compressed storage but less
        // efficient search
        // https://github.com/ajalab/fm-index/issues/24
        let level = 0;

        // If the text length is tiny, FMIndex starts to break down, so
        // use a workaround
        // https://github.com/ajalab/fm-index/issues/22
        // https://github.com/ajalab/fm-index/issues/23
        // just search in the text itself directly
        let index = if is_tiny {
            // construct a dummy text index where we know construction will succeed
            // we don't use it
            SearchIndexBuilder::new()
                .sampling_level(level)
                .build("dummy text".as_bytes().to_vec())
        } else {
            SearchIndexBuilder::new()
                .sampling_level(level)
                .build(text.as_bytes().to_vec())
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
        // a bit of duplication so we don't have to turn stuff into bytes and then
        // back into a str in locate_by_bytes
        if self.is_tiny {
            // if it's tiny we can't use the text index, so just search directly
            return self.text.match_indices(pattern).map(|(i, _)| i).collect();
        }
        self.locate_by_bytes(pattern.as_bytes())
    }

    pub(crate) fn locate_by_bytes(&self, pattern: &[u8]) -> Vec<usize> {
        if self.is_tiny {
            let pattern = std::str::from_utf8(pattern).unwrap();
            // if it's tiny we can't use the text index, so just search directly
            return self.text.match_indices(pattern).map(|(i, _)| i).collect();
        }
        self.index
            .search(pattern)
            .locate()
            .iter()
            .map(|i| {
                let i: usize = (*i).try_into().unwrap();
                i
            })
            .collect()
    }

    // TODO: to implement efficient count we really need to be able to use
    // an FM Index that starts with \0. This would allow efficient count for
    // everything except contains

    pub(crate) fn starts_with(&self, pattern: &str) -> Vec<usize> {
        // find those text indices that start with the pattern
        // this means i - 1 is \0, or alternative i == 0
        // TODO: an alternative implementation would look for \0pattern, but
        // would need special handling for the first section, plus allocation,
        // so not sure it's faster
        // if we *could* have the text start with 0 (but fm index right now crashes on it),
        // it would actually make code a lot simpler.
        self.locate(pattern)
            .into_iter()
            .filter(|&i| i == 0 || self.text.as_bytes()[i - 1] == 0)
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
        self.locate(pattern)
            .into_iter()
            .filter(|&i| {
                let bytes = self.text.as_bytes();
                bytes[i + pattern.len()] == 0 && (i == 0 || bytes[i - 1] == 0)
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
        let text = "hello something\0world hello\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.starts_with("hello");
        located.sort();
        assert_eq!(located, vec![0]);
    }

    #[test]
    fn test_starts_with_middle() {
        let text = "world\0hello world\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.starts_with("hello");
        located.sort();
        assert_eq!(located, vec![6]);
    }

    #[test]
    fn test_ends_with_simple() {
        let text = "hello world\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.ends_with("world");
        located.sort();
        assert_eq!(located, vec![6]);
    }

    #[test]
    fn test_doesnt_end_with() {
        let text = "hello world\0";
        let search = TextSearch::new(text.to_string());
        let located = search.ends_with("hello");
        assert_eq!(located, vec![]);
    }

    #[test]
    fn test_ends_with_middle() {
        let text = "world\0hello world\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.ends_with("world");
        located.sort();
        assert_eq!(located, vec![0, 12]);
    }

    #[test]
    fn test_equals() {
        let text = "hello\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.equals("hello");
        located.sort();
        assert_eq!(located, vec![0]);
        let mut located = search.equals("hel");
        located.sort();
        assert_eq!(located, vec![]);
    }

    #[test]
    fn test_equals_middle() {
        let text = "hello\0world\0";
        let search = TextSearch::new(text.to_string());
        let mut located = search.equals("world");
        located.sort();
        assert_eq!(located, vec![6]);
        let mut located = search.equals("wor");
        located.sort();
        assert_eq!(located, vec![]);
    }
}
