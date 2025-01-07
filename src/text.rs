use std::ops::Range;

use sucds::bit_vectors::{Rank, SArray, Select};
use vers_vecs::{BitVec, RsVec};

pub(crate) struct TextBuilder {
    s: String,
    bitmap: BitVec,
}

impl TextBuilder {
    pub(crate) fn new() -> Self {
        let mut bitmap = BitVec::new();
        // add terminator in the beginning, so if select of 1 happens, we get
        // the start of the text
        bitmap.append(true);
        let mut s = String::new();
        s.push(0.into());
        Self { s, bitmap }
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
        TextUsage {
            text: self.s,
            sarray: SArray::from_bits(self.bitmap.iter().map(|b| b != 0)).enable_rank(),
        }
    }
}

pub(crate) struct TextUsage {
    text: String,
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
        // we subtract 1 here, as we have a terminator in the beginning
        // and we want to start from 0
        TextId(
            self.sarray
                .rank1(index)
                .expect("Text index should be in bounds")
                - 1,
        )
    }

    pub(crate) fn text_index(&self, text_id: TextId) -> usize {
        // we add 1 here as we want the index of the actual start of the
        // text rather than the terminator
        // unwrap is okay as we know we have a text id already
        self.sarray.select1(text_id.0).unwrap() + 1
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
        let text_id = usage.text_id(1);
        assert_eq!(usage.text_index(text_id), 1);
    }

    #[test]
    fn test_one_text_middle() {
        let mut builder = TextBuilder::new();
        builder.text_node("hello");
        let usage = builder.build();
        let text_id = usage.text_id(2);
        assert_eq!(usage.text_index(text_id), 1);
    }

    #[test]
    fn test_two_texts() {
        let mut builder = TextBuilder::new();
        // 1..6
        builder.text_node("hello");
        // 7..12
        builder.text_node("world");
        let usage = builder.build();

        // in 'hello' text
        let text_id = usage.text_id(1);
        assert_eq!(usage.text_index(text_id), 1);
        let text_id = usage.text_id(2);
        assert_eq!(usage.text_index(text_id), 1);

        // in 'world' text
        let text_id = usage.text_id(7);
        assert_eq!(usage.text_index(text_id), 7);
        let text_id = usage.text_id(8);
        assert_eq!(usage.text_index(text_id), 7);
    }

    #[test]
    fn test_two_texts_range() {
        let mut builder = TextBuilder::new();
        // 1..6
        builder.text_node("hello");
        // 7..12
        builder.text_node("world");
        let usage = builder.build();

        assert_eq!(usage.text_range(TextId(0)), 1..6);
        assert_eq!(usage.text_range(TextId(1)), 7..12);
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
