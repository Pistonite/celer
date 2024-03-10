//! Iterators for CompDoc

use crate::lang::DocRichTextBlock;

use super::{CompDoc, CompLine, DocNote};

impl<'p> CompDoc<'p> {
    /// Iterate over each CompLine of the CompDoc
    pub fn lines(&self) -> impl Iterator<Item = &CompLine> {
        self.route.iter().flat_map(|section| section.lines.iter())
    }

    /// Iterate over each CompLine of the CompDoc
    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut CompLine> {
        self.route.iter_mut().flat_map(|section| section.lines.iter_mut())
    }

    /// Iterate over each DocRichText of the CompDoc.
    ///
    /// Preface and counter are excluded by default. Use the builder options
    /// in the returned object to configure them
    pub fn rich_texts(&self) -> RichTextIter<'_, 'p> {
        RichTextIter::new(self)
    }

    /// Iterate over each DocRichText of the CompDoc, except for the counter
    ///
    /// Preface and counter are excluded by default. Use the builder options
    /// in the returned object to configure them
    pub fn rich_texts_mut(&mut self) -> RichTextIterMut<'_, 'p> {
        RichTextIterMut::new(self)
    }
}

impl CompLine {
    /// Iterate over each DocRichText of the CompLine
    ///
    /// Counter is excluded by default. Use the builder options
    /// in the returned object to configure them
    pub fn rich_texts(&self) -> LineRichTextIter {
        LineRichTextIter::new(self)
    }

    /// Iterate over each DocRichText of the CompLine, except for the counter
    ///
    /// Counter is excluded by default. Use the builder options
    /// in the returned object to configure them
    pub fn rich_texts_mut(&mut self) -> LineRichTextIterMut {
        LineRichTextIterMut::new(self)
    }
}

/// Implementation of the rich text iterators
struct RichTextIter<'c, 'p> {
    doc: &'c CompDoc<'p>,
    with_preface: bool,
    with_counter: bool,
}
struct RichTextIterMut<'c, 'p> {
    doc: &'c mut CompDoc<'p>,
    with_preface: bool,
    with_counter: bool,
}
struct LineRichTextIter<'c> {
    line: &'c CompLine,
    with_counter: bool,
}
struct LineRichTextIterMut<'c> {
    line: &'c mut CompLine,
    with_counter: bool,
}

impl<'c, 'p> RichTextIter<'c, 'p> {
    pub fn new(doc: &'c CompDoc<'p>) -> Self {
        Self { 
            doc, 
            with_preface: false,
            with_counter: false 
        }
    }
    pub fn with_preface(mut self) -> Self {
        self.with_preface = true;
        self
    }
    pub fn with_counter(mut self) -> Self {
        self.with_counter = true;
        self
    }
}
impl<'c, 'p> RichTextIterMut<'c, 'p> {
    pub fn new(doc: &'c mut CompDoc<'p>) -> Self {
        Self { 
            doc, 
            with_preface: false,
            with_counter: false,
        }
    }
    pub fn with_preface(mut self) -> Self {
        self.with_preface = true;
        self
    }
    pub fn with_counter(mut self) -> Self {
        self.with_counter = true;
        self
    }
}
impl<'c> LineRichTextIter<'c> {
    pub fn new(line: &'c CompLine) -> Self {
        Self { 
            line, 
            with_counter: false 
        }
    }
    pub fn with_counter(mut self) -> Self {
        self.with_counter = true;
        self
    }
}
impl<'c> LineRichTextIterMut<'c> {
    pub fn new(line: &'c mut CompLine) -> Self {
        Self { 
            line, 
            with_counter: false 
        }
    }
    pub fn with_counter(mut self) -> Self {
        self.with_counter = true;
        self
    }
}

impl<'c, 'p> IntoIterator for RichTextIter<'c, 'p> 
{
    type Item = &'c DocRichTextBlock;
    type IntoIter = DynIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = self.doc.route.iter().flat_map(|section| {
            section.lines.iter().flat_map(|line| {
                rich_text_iter(line, self.with_counter)
            })
        });
        if self.with_preface {
            let preface_iter = self.doc.preface.iter().flat_map(|text| text.iter());
            return DynIter::new(preface_iter.chain(iter));
        }
        DynIter::new(iter)
    }
}
impl<'c, 'p> IntoIterator for RichTextIterMut<'c, 'p> 
{
    type Item = &'c mut DocRichTextBlock;
    type IntoIter = DynIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = self.doc.route.iter_mut().flat_map(|section| {
            section.lines.iter_mut().flat_map(|line| {
                rich_text_iter_mut(line, self.with_counter)
            })
        });
        if self.with_preface {
            let preface_iter = self.doc.preface.iter_mut().flat_map(|text| text.iter_mut());
            return DynIter::new(preface_iter.chain(iter));
        }
        DynIter::new(iter)
    }
}
impl<'c> IntoIterator for LineRichTextIter<'c> 
{
    type Item = &'c DocRichTextBlock;
    type IntoIter = DynIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = rich_text_iter(self.line, self.with_counter);
        DynIter::new(iter)
    }
}
impl<'c> IntoIterator for LineRichTextIterMut<'c> 
{
    type Item = &'c mut DocRichTextBlock;
    type IntoIter = DynIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = rich_text_iter_mut(self.line, self.with_counter);
        DynIter::new(iter)
    }
}

fn rich_text_iter<'c>(line: &'c CompLine, with_counter: bool) -> impl Iterator<Item = &'c DocRichTextBlock> {
    let iter = line.text.iter()
        .chain(line.secondary_text.iter())
        .chain(line.notes.iter().flat_map(|note| {
            match note {
                DocNote::Text { content } => DynIter::new(content.iter()),
                _ => DynIter::new(std::iter::empty())
            }
        }))
        .chain({
            if !with_counter {
                DynIter::new(std::iter::empty())
            } else if let Some(counter) = &line.counter_text {
                DynIter::new(std::iter::once(counter))
            } else {
                DynIter::new(std::iter::empty())
            }
        })
        .chain({
            if let Some(split_name) = &line.split_name {
                DynIter::new(split_name.iter())
            } else {
                DynIter::new(std::iter::empty())
            }
        });

    iter
}

fn rich_text_iter_mut<'c>(line: &'c mut CompLine, with_counter: bool) -> impl Iterator<Item = &'c mut DocRichTextBlock> {
    let iter = line.text.iter_mut()
        .chain(line.secondary_text.iter_mut())
        .chain(line.notes.iter_mut().flat_map(|note| {
            match note {
                DocNote::Text { content } => DynIter::new(content.iter_mut()),
                _ => DynIter::new(std::iter::empty())
            }
        }))
        .chain({
            if !with_counter {
                DynIter::new(std::iter::empty())
            } else if let Some(counter) = &mut line.counter_text {
                DynIter::new(std::iter::once(counter))
            } else {
                DynIter::new(std::iter::empty())
            }
        })
        .chain({
            if let Some(split_name) = &mut line.split_name {
                DynIter::new(split_name.iter_mut())
            } else {
                DynIter::new(std::iter::empty())
            }
        });

    iter
}

struct DynIter<T>(Box<dyn Iterator<Item = T>>);

impl<T> DynIter<T> {
    fn new<I>(iter: I) -> Self
        where I: Iterator<Item = T>{
        Self(Box::new(iter))
    }
}

impl<T> Iterator for DynIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
