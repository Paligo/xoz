use std::ops::Range;

use fm_index::BackwardSearchIndex;
use sucds::bit_vectors::{Rank, SArray, Select};
use vers_vecs::{BitVec, RsVec};

use crate::textsearch::TextSearch;

pub(crate) struct TextBuilder {
    s: String,
    bitmap: BitVec,
}

impl TextBuilder {
    pub(crate) fn new() -> Self {
        Self {
            s: String::new(),
            bitmap: BitVec::new(),
        }
    }

    pub(crate) fn text_node(&mut self, text: &str) {
        self.s.push_str(text);
        // add as many false as there are bytes in the text
        for _ in 0..text.len() {
            self.bitmap.append(false);
        }
        // terminator $, the 0 byte
        self.s.push(0.into());
        // we have a single true we append now
        self.bitmap.append(true);
    }

    pub(crate) fn build(self) -> TextUsage {
        let s = if self.s.is_empty() {
            // we always need to end with 0, even if we have no texts
            // TODO: we need to make this long enough, as a single 0 also breaks
            // fm index
            String::from("FOOBAR\0")
        } else {
            self.s
        };
        TextUsage {
            search: TextSearch::new(s),
            sarray: SArray::from_bits(self.bitmap.iter().map(|b| b != 0)).enable_rank(),
        }
    }
}

pub(crate) struct TextUsage {
    search: TextSearch,
    sarray: SArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextId(usize);

impl TextId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

impl TextUsage {
    pub(crate) fn text_id(&self, index: usize) -> TextId {
        TextId(
            self.sarray
                .rank1(index)
                .expect("Text index should be in bounds"),
        )
    }

    pub(crate) fn text_index(&self, text_id: TextId) -> usize {
        let id = text_id.0;
        if id == 0 {
            0
        } else {
            // we add 1 here as we want the index of the actual start of the
            // text rather than the terminator
            // unwrap is okay as we know we have a text id already
            self.sarray.select1(id - 1).unwrap() + 1
        }
    }

    pub(crate) fn text_range(&self, text_id: TextId) -> Range<usize> {
        let start = self.text_index(text_id);
        let end = self.text_index(TextId(text_id.0 + 1));
        start..(end - 1)
    }

    pub(crate) fn text_value(&self, text_id: TextId) -> &str {
        let range = self.text_range(text_id);
        self.search.text_in_range(range)
    }

    pub(crate) fn search_text_ids(&self, query: &str) -> Vec<TextId> {
        self.search
            .locate(query)
            .iter()
            .map(|&i| self.text_id(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_text_beginning() {
        let mut builder = TextBuilder::new();
        builder.text_node("hello");
        let usage = builder.build();
        let text_id = usage.text_id(0);
        assert_eq!(usage.text_index(text_id), 0);
    }

    #[test]
    fn test_one_text_middle() {
        let mut builder = TextBuilder::new();
        builder.text_node("hello");
        let usage = builder.build();
        let text_id = usage.text_id(3);
        assert_eq!(usage.text_index(text_id), 0);
    }

    #[test]
    fn test_two_texts() {
        let mut builder = TextBuilder::new();
        // 0..5
        builder.text_node("hello");
        // 6..11
        builder.text_node("world");
        let usage = builder.build();

        // in 'hello' text
        let text_id = usage.text_id(0);
        assert_eq!(usage.text_index(text_id), 0);
        let text_id = usage.text_id(1);
        assert_eq!(usage.text_index(text_id), 0);

        // in 'world' text
        let text_id = usage.text_id(6);
        assert_eq!(usage.text_index(text_id), 6);
        let text_id = usage.text_id(8);
        assert_eq!(usage.text_index(text_id), 6);
    }

    #[test]
    fn test_two_texts_range() {
        let mut builder = TextBuilder::new();
        // 0..5
        builder.text_node("hello");
        // 6..11
        builder.text_node("world");
        let usage = builder.build();

        assert_eq!(usage.text_range(TextId(0)), 0..5);
        assert_eq!(usage.text_range(TextId(1)), 6..11);
    }

    #[test]
    fn test_two_texts_value() {
        let mut builder = TextBuilder::new();
        builder.text_node("hello");
        builder.text_node("world");
        let usage = builder.build();

        assert_eq!(usage.text_value(TextId(0)), "hello");
        assert_eq!(usage.text_value(TextId(1)), "world");
    }

    #[test]
    fn test_tiny_search() {
        let mut builder = TextBuilder::new();
        builder.text_node("a");
        builder.text_node("b");
        let usage = builder.build();

        assert_eq!(usage.search_text_ids("a"), vec![TextId(0)]);
        assert_eq!(usage.search_text_ids("b"), vec![TextId(1)]);
    }

    #[test]
    fn test_search_bigger_text() {
        let mut builder = TextBuilder::new();
        builder.text_node("hello");
        builder.text_node("world");
        let usage = builder.build();

        assert_eq!(usage.search_text_ids("hello"), vec![TextId(0)]);
        assert_eq!(usage.search_text_ids("world"), vec![TextId(1)]);
        assert_eq!(usage.search_text_ids("wor"), vec![TextId(1)]);
    }
}
