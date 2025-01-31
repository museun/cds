use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use chorts::{data::Text, Filename, Highlight, Visit, Visitor};

use crate::args::ClassifyKind;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Spanned<T> {
    pub(crate) item: T,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl<T> Spanned<T> {
    const fn new(item: T, row: usize, col: usize) -> Self {
        Self { item, row, col }
    }
}

#[derive(Debug)]
pub struct Missing {
    pub message: Spanned<String>,
    pub text: Vec<Highlight<'static>>,
}

#[derive(Default)]
pub struct MissingDocs<'a> {
    pub map: BTreeMap<PathBuf, Vec<Missing>>,
    pub last: Option<String>,
    set: HashSet<&'a PathBuf>,
    include: Vec<&'static str>, // this is exclusive
    exclude: Vec<&'static str>, // this is inclusive
}

impl<'a> MissingDocs<'a> {
    pub fn new(
        set: HashSet<&'a PathBuf>,
        include: impl IntoIterator<Item = ClassifyKind>,
        exclude: impl IntoIterator<Item = ClassifyKind>,
    ) -> Self {
        Self {
            map: BTreeMap::new(),
            last: None,
            set,
            include: include.into_iter().map(|s| s.as_str()).collect(),
            exclude: exclude.into_iter().map(|s| s.as_str()).collect(),
        }
    }

    fn filter_message(&self, msg: &chorts::data::Message) -> bool {
        let Some(code) = &msg.code else { return false };
        match &*code.code {
            "missing_docs"
            | "clippy::empty_docs"
            | "clippy::suspicious_doc_comments"
            | "clippy::missing-errors-doc"
            | "clippy::missing-panics-doc"
            | "clippy::missing-safety-doc"
            | "clippy::unnecessary_safety_doc"
            | "clippy::undocumented_unsafe_blocks" => {}
            _ => return false,
        };

        // the empty check because any([]) == true
        if !self.include.is_empty() && !self.include.iter().any(|c| msg.message.ends_with(c)) {
            return false;
        }

        if !self.exclude.is_empty() && self.exclude.iter().any(|c| msg.message.ends_with(c)) {
            return false;
        }

        true
    }
}

impl Visitor for MissingDocs<'_> {
    fn visit_message(&mut self, message: &chorts::data::Message) {
        if self.filter_message(message) {
            self.last = Some(message.message.clone());
            message.spans.accept(self);
        }
    }

    fn visit_span(&mut self, file: Filename<'_>, text: &[Text]) {
        if !self.set.is_empty() && !self.set.contains(&PathBuf::from(&*file.name)) {
            let _ = self.last.take();
            return;
        }

        let last = self.last.take();
        let last = last.map(|value| Spanned::new(value, file.row, file.col));

        #[derive(Default)]
        struct TextCollector {
            inner: Vec<Highlight<'static>>,
        }

        impl Visitor for TextCollector {
            fn visit_text(&mut self, text: Highlight<'_>) {
                self.inner.push(text.to_owned());
            }
        }

        let mut tv = TextCollector::default();
        for text in text {
            text.accept(&mut tv);
        }

        let missing = Missing {
            message: last.expect("valid tree"),
            text: tv.inner,
        };

        self.map
            .entry(PathBuf::from(file.name.to_string()))
            .or_default()
            .push(missing);
    }
}
