use std::ops::Range;

use sucds::bit_vectors::{Rank, SArray, Select};
use vers_vecs::{BitVec, RsVec};

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
        self.s.push('\0');
        // we have a single true we append now
        self.bitmap.append(true);
    }

    pub(crate) fn build(self) -> TextUsage {
        TextUsage {
            text: self.s,
            sarray: SArray::from_bits(self.bitmap.iter().map(|b| b != 0)).enable_rank(),
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct TextId(usize);

impl TextId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub(crate) fn id(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub(crate) struct TextUsage {
    text: String,
    sarray: SArray,
}

impl TextUsage {
    #[allow(dead_code)]
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
        &self.text[range]
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
}
