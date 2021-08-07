use super::{DResult, DrawError, Drawer};
use crate::config::{Config, Content as SlideTemplate, Decoration, Point, Rectangle};
use crate::parser::{Content, Slide};
use crate::util::pdf_util::*;
use printpdf::{
    image, IndirectFontRef, Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference,
    PdfPageReference, Pt,
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
        decos: &Vec<Decoration>,
        layer: PdfLayerReference,
        config: &Config<'_>,
    ) -> DResult<()> {
        for d in decos.into_iter() {
            // creates the decoration/shape to draw
            let line = Line {
                points: to_pdf_rect(&d.area),
                is_closed: true,
                has_fill: true,
                has_stroke: false,
                is_clipping_path: false,
            };

            // set the color
            layer.set_fill_color(config.get_color(d.color_idx)?.into());
            // and draw it
            layer.add_shape(line)
        }

        Ok(())
    }

    /// draws the content of a slide to the pdf page
    fn draw_content(
        &mut self,
        contents: &Vec<SlideTemplate>,
        slide: Slide,
        page: &PdfPageReference,
    ) -> DResult<()> {
        let mut layer = page.add_layer("");
        for (template, content) in contents.iter().zip(slide.contents.into_iter()) {
            let args = DrawingArgs {
                area: make_inner_pt(&to_bottom_left(&template.area)),
                font_size: template.font_size,
                layer: layer.clone(),
            };

            layer.set_font(&self.font_pdf, args.font_size as f64);
            match content {
                Content::Text(s) => {
                    self.text(s, &args);
                }
                Content::Config(_) => panic!("Config calls should be handled before drawing"),
                Content::Image(_, p) => {
                    // TODO: add description
                    self.image(p, args)?;
                    layer = page.add_layer("");
                }
                Content::List(i) => self.list(i, args),
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
            area:
                Rectangle {
                    orig: Point(x_orig, y_orig),
                    size,
                },
        }: &DrawingArgs,
    ) -> Pt {
        dbg!(&content);
        let width = size.0 .0 as f32;
        let font_size = *font_size;

        // all the beginnings of the line
        let mut beginnings = self
            .determine_line_beginnings(&content, font_size, width)
            .inspect(|b| {
                eprintln!("line beginning at {}", b);
            });
        // TODO: maybe check for None
        // get the first start (0)
        let mut start = beginnings.next().unwrap();
        // add the total length of the text => all endings
        let endings = beginnings.chain(std::iter::once(content.len()));
        dbg!(content.len());

        layer.begin_text_section();

        // settings for text
        layer.set_font(&self.font_pdf, font_size as f64);
        layer.set_line_height(font_size as f64);
        // TODO: add position
        layer.set_text_cursor(Mm::from(*x_orig), Mm::from(*y_orig));

        let mut lines_written = 0;
        // actual drawing
        for end in endings {
            dbg!(start, end, &content[start..end]);
            layer.write_text(&content[start..end], &self.font_pdf);
            layer.add_line_break();
            start = end;
            lines_written += 1;
        }

        //layer.write_text(&content, &self.font_pdf);
        layer.end_text_section();

        Pt((lines_written + 1) as f64 * font_size as f64)
    }

    fn list(&self, items: Vec<(u8, String)>, mut args: DrawingArgs) {
        let orig_x = args.area.orig.0;
        let space = Pt(args.font_size as f64);

        let mut pt_written = Pt(0.0);

        for (ident, text) in items {
            // TODO: draw something to indicate list
            args.area.orig.0 = orig_x + space * ident as f64;
            self.text(String::from("-"), &args);
            args.area.orig.0 = orig_x + space * (ident + 1) as f64;
            args.area.orig.1 += pt_written;
            args.area.size.1 -= pt_written;
            pt_written = self.text(text, &args);
        }
    }

    fn image(
        &self,
        image_path: PathBuf,
        DrawingArgs {
            layer,
            area: Rectangle { orig: pos, .. },
            ..
        }: DrawingArgs,
    ) -> DResult<()> {
        let image = image::io::Reader::open(image_path)?.decode()?;
        let pdf_image = printpdf::Image::from_dynamic_image(&image);

        // TODO: get the scaling right
        pdf_image.add_to_layer(
            layer,
            Some(pos.0.into()),
            Some(pos.1.into()),
            None,
            None,
            None,
            Some(DPI as f64),
        );
        Ok(())
    }

    /// determines when a line of glyphs
    /// in the selected font is wider than the arranged area
    fn determine_line_beginnings<'a>(
        &'a self,
        text: &'a str,
        font_size: f32,
        width: f32,
    ) -> impl Iterator<Item = usize> + 'a {
        /*let v_metrics = self.font.v_metrics_unscaled();
        let scale = (v_metrics.ascent - v_metrics.descent) / self.font.units_per_em() as f32;
        let scale = Scale::uniform(scale);*/
        let width = pt_to_px(width, DPI);
        //dbg!(width);
        self.font
            .layout(text, Scale::uniform(font_size), Default::default())
            // TODO: add better error handling
            .map(move |g| g.into_unpositioned().h_metrics().advance_width)
            //.inspect(|w| eprintln!("width of glyph: {}", w))
            .scan(0.0, |state, w| {
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
    fn process_glyph_width(max_width: f32) -> impl FnMut((usize, f32)) -> Option<usize> {
        //let mut times: u8 = 0;
        let mut last_line: f32 = 0.0;
        move |(i, sum)| {
            //dbg!(sum);
            if sum - last_line > max_width || i == 0 {
                //times += 1;
                last_line = sum;
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
