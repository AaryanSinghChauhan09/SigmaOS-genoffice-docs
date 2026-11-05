#[derive(Debug, Clone)]
pub struct Style {
    pub name: String,
    pub font_family: String,
    pub font_size_pt: u16,
    pub bold: bool,
    pub italic: bool,
}

impl Style {
    pub fn new(name: impl Into<String>, font_family: impl Into<String>, font_size_pt: u16) -> Self {
        Self { name: name.into(), font_family: font_family.into(), font_size_pt, bold: false, italic: false }
    }
    pub fn bold(mut self) -> Self { self.bold = true; self }
    pub fn italic(mut self) -> Self { self.italic = true; self }
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub text: String,
    pub style: String,
}

#[derive(Debug, Default)]
pub struct Document {
    pub title: String,
    paragraphs: Vec<Paragraph>,
    styles: Vec<Style>,
}

impl Document {
    pub fn new(title: impl Into<String>) -> Self {
        let mut doc = Self { title: title.into(), ..Default::default() };
        doc.styles.push(Style::new("Normal", "Liberation Serif", 12));
        doc.styles.push(Style::new("Heading 1", "Liberation Sans", 18).bold());
        doc
    }
    pub fn append_paragraph(&mut self, text: impl Into<String>, style: impl Into<String>) {
        self.paragraphs.push(Paragraph { text: text.into(), style: style.into() });
    }
    pub fn paragraph_count(&self) -> usize { self.paragraphs.len() }
    pub fn style_count(&self) -> usize { self.styles.len() }
    pub fn paragraphs(&self) -> &[Paragraph] { &self.paragraphs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_document_has_default_styles() {
        let doc = Document::new("Test");
        assert_eq!(doc.style_count(), 2);
    }
}
