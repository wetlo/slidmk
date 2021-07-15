use super::{DResult, DrawError, Drawer};
use crate::config::{Config, Decorations, Rectange};
use crate::parser::Slide;
use printpdf::{
    Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference, PdfPageReference, Point,
};
use std::io::{Write, BufWriter};

/*
cant make an const function with floating point
arithmetic, yet
*/
macro_rules! px_to_mm {
    ($px:expr) => {
        Mm($px as f64 * (25.4 / 300.0))
    };
}

const X_SIZE: Mm = px_to_mm!(1920);
const Y_SIZE: Mm = px_to_mm!(1080);

pub struct PdfMaker {
    doc: PdfDocumentReference,
    slide_idx: usize,
}

impl Drawer for PdfMaker {
    /// creates a pdf maker with information from the
    /// config
    fn with_config(config: &Config<'_>) -> Self {
        Self {
            doc: PdfDocument::empty(config.doc_name),
            slide_idx: 0,
        }
    }

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


    // TODO: maybe input the config to fix ownership problems
    fn draw_slide(&mut self, slide: Slide, config: &Config<'_>) -> DResult<()> {
        // get info of how the slide should be drawn
        let kind = config
            .slide_styles
            .get(&slide.kind)
            .ok_or_else(|| DrawError::KindNotFound(slide.kind))?;

        // create the new pdf page for the slide
        let (layer, _page) = self.create_pdf_page(self.slide_idx.to_string());

        self.draw_decorations(&kind.decorations, layer, config)?;
        Ok(())
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

///
fn to_pdf_rect(_rect: &Rectange<f64>) -> Vec<(Point, bool)> {
    vec![]
}

fn _to_pdf_coords((x, y): (f64, f64)) -> (f64, f64) {
    (x, y)
}
