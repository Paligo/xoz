use crate::{data::Structure, tagvec::SArrayMatrix, text::TextUsage};

pub struct Document {
    pub(crate) structure: Structure<SArrayMatrix>,
    pub(crate) text_usage: TextUsage,
}
