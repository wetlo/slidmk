use super::{DResult, DrawError, Drawer};
use crate::config::{Config, Contents, Decorations};
use crate::parser::Slide;
use crate::util::pdf_util::*;
use printpdf::{Line, PdfDocument, PdfDocumentReference, PdfLayerReference, PdfPageReference};
use rusttype::Font;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct PdfMaker {
    doc: PdfDocumentReference,
    font: Font<'static>,
    slide_idx: usize,
}

impl Drawer for PdfMaker {
    /// draws all the slides of the iterator into the document
    fn create_slides<I>(&mut self, mut slides: I, config: &Config<'_>) -> DResult<()>
    where
        I: Iterator<Item = Slide>,
    {
        slides.try_for_each(|slide| self.draw_slide(slide, config))
    }

    /// writes the document to the file system
    fn write<W: Write>(self, to: W) -> Result<(), DrawError> {
        let mut buf = BufWriter::new(to);
        self.doc.save(&mut buf).map_err(|e| e.into())
    }
}

impl PdfMaker {
    /// creates a pdf maker with information from the
    /// config
    pub fn with_config(config: &Config<'_>) -> DResult<Self> {
        // TODO: add a better system to not have 2 fs interactions for rusttype and printpdf
        let doc = PdfDocument::empty(config.doc_name);

        let drawer = Self {
            font: Self::init_font(get_font_path(&config.font)?, &doc)?,
            doc,
            slide_idx: 0,
        };

        Ok(drawer)
    }

    pub fn init_font(path: PathBuf, doc: &PdfDocumentReference) -> DResult<Font<'static>> {
        std::fs::read(&path)
            .ok()
            .and_then(|data| {
                doc.add_external_font(&data[..]).ok()?;
                Font::try_from_vec(data)
            })
            .map_or_else(
                || Err(DrawError::FontNotLoaded(path.to_string_lossy().into_owned())),
                |f| Ok(f),
            )
    }

    // TODO: maybe input the config to fix ownership problems
    fn draw_slide(&mut self, slide: Slide, config: &Config<'_>) -> DResult<()> {
        // get info of how the slide should be drawn
        let kind = config
            .slide_styles
            .get(&slide.kind)
            .ok_or_else(|| DrawError::KindNotFound(slide.kind.clone()))?;

        // create the new pdf page for the slide
        let (layer, _page) = self.create_pdf_page(self.slide_idx.to_string());

        self.draw_decorations(&kind.decorations, layer, config)?;
        self.draw_content(&kind.content, slide)
    }

    /// draws the given decoration on a given pdf-layer
    fn draw_decorations(
        &mut self,
        decos: &Decorations,
        layer: PdfLayerReference,
        config: &Config<'_>,
    ) -> DResult<()> {
        for (pos, color) in decos {
            // creates the decoration/shape to draw
            let line = Line {
                points: to_pdf_rect(pos),
                is_closed: true,
                has_fill: true,
                has_stroke: false,
                is_clipping_path: false,
            };

            // set the color
            layer.set_fill_color(config.get_color(*color)?.into());
            // and draw it
            layer.add_shape(line)
        }

        Ok(())
    }

    fn draw_content(&mut self, contents: &Contents, slide: Slide) -> DResult<()> {
        Ok(())
    }

    fn create_pdf_page<S>(&mut self, name: S) -> (PdfLayerReference, PdfPageReference)
    where
        S: Into<String>,
    {
        // TODO: add way to determain the size, maybe hard code for now
        let (page, layer) = self.doc.add_page(X_SIZE, Y_SIZE, name);
        let page = self.doc.get_page(page);

        (page.get_layer(layer), page)
    }
}
