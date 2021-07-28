use super::{DResult, DrawError, Drawer};
use crate::config::{Config, Contents, Decorations, Rectangle};
use crate::parser::{Content, Slide};
use crate::util::pdf_util::*;
use printpdf::{
    IndirectFontRef, Line, PdfDocument, PdfDocumentReference, PdfLayerReference, PdfPageReference,
};
use rusttype::{Font, Scale};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct PdfMaker {
    doc: PdfDocumentReference,
    font: Font<'static>,
    font_pdf: IndirectFontRef,
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
        let doc = PdfDocument::empty(config.doc_name);
        let (font_pdf, font) = Self::init_font(get_font_path(&config.font)?, &doc).unwrap();
        let drawer = Self {
            font,
            font_pdf,
            doc,
            slide_idx: 0,
        };

        Ok(drawer)
    }

    /// loads the font at the given path into the pdf and
    /// with rusttype
    pub fn init_font(
        path: PathBuf,
        doc: &PdfDocumentReference,
    ) -> DResult<(IndirectFontRef, Font<'static>)> {
        std::fs::read(&path)
            .ok()
            .and_then(|data| {
                Some((
                    doc.add_external_font(&data[..]).ok()?,
                    Font::try_from_vec(data)?,
                ))
            })
            .ok_or_else(|| {
                DrawError::FontNotLoaded(
                    // maybe find a better way to convert to string
                    path.to_string_lossy().into_owned(),
                )
            })
    }

    // TODO: maybe input the config to fix ownership problems
    /// draws a slide with the information from the config
    fn draw_slide(&mut self, slide: Slide, config: &Config<'_>) -> DResult<()> {
        // get info of how the slide should be drawn
        let kind = config
            .slide_styles
            .get(&slide.kind)
            .ok_or_else(|| DrawError::KindNotFound(slide.kind.clone()))?;

        // create the new pdf page for the slide
        let (layer, page) = self.create_pdf_page(self.slide_idx.to_string());

        self.draw_decorations(&kind.decorations, layer, config)?;
        self.draw_content(&kind.content, slide, &page)
    }

    /// draws the given decoration a slide to the pdf layer
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

    /// draws the content of a slide to the pdf page
    fn draw_content(
        &mut self,
        contents: &Contents,
        slide: Slide,
        page: &PdfPageReference,
    ) -> DResult<()> {
        let mut layer = page.add_layer("");
        for ((area, font_size), content) in contents.iter().zip(slide.contents.into_iter()) {
            let args = DrawingArgs {
                area: pdfify_rect(area),
                font_size: *font_size,
                layer: layer.clone(),
            };

            layer.set_font(&self.font_pdf, args.font_size as f64);
            match content {
                Content::Text(s) => self.text(s, args),
                Content::Config(_) => panic!("Config calls should be handled before drawing"),
                Content::Image(_, _) => {
                    // needs a new layer
                    layer = page.add_layer("");
                }
                Content::List(_) => todo!(),
            }
        }

        Ok(())
    }

    /// draws some text to a pdf layer with the given parameters
    /// (font size, area)
    fn text(
        &self,
        content: String,
        DrawingArgs {
            layer,
            font_size,
            area,
        }: DrawingArgs,
    ) {
        let width = area.size.0 as i32;
        // all the beginnings of the line
        let mut beginnings = self.determine_line_beginnings(&content, font_size, width);
        // TODO: maybe check for None
        // get the first start (0)
        let mut start = beginnings.next().unwrap();
        // add the total length of the text => all endings
        let endings = beginnings.chain(std::iter::once(content.len()));

        layer.begin_text_section();

        // settings for text
        layer.set_font(&self.font_pdf, font_size as f64);
        layer.set_line_height(font_size as f64);
        // TODO: add position

        // actual drawing
        for end in endings {
            layer.write_text(&content[start..end], &self.font_pdf);
            layer.add_line_break();
            start = end;
        }

        layer.end_text_section();
    }

    /// determines when a line of glyphs
    /// in the selected font is wider than the arranged area
    fn determine_line_beginnings<'a>(
        &'a self,
        text: &'a str,
        font_size: f32,
        width: i32,
    ) -> impl Iterator<Item = usize> + 'a {
        self.font
            .layout(text, Scale::uniform(font_size), Default::default())
            // TODO: add better error handling
            .map(|g| g.pixel_bounding_box().unwrap().width())
            .scan(0, |state, w| {
                *state += w;
                Some(*state)
            })
            .enumerate()
            .filter_map(Self::process_glyph_width(width))
    }

    /// gives back a closure that
    /// looks if with this glyph a new line should start
    /// if that's the case it will return Some(index)
    /// else it will return None
    fn process_glyph_width(max_width: i32) -> impl FnMut((usize, i32)) -> Option<usize> {
        let mut times = 0;
        move |(i, sum)| {
            if sum > max_width * times {
                times += 1;
                Some(i)
            } else {
                None
            }
        }
    }

    /// creates a new pdf page with constant size and a new layer
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
