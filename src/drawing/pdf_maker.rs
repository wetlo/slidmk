use super::{DResult, DrawError, Drawer};
use crate::config::{self, Config, Content as SlideTemplate, Decoration};
use crate::parser::{Content, Slide};
use crate::util::pdf;
use std::io::Write;

const DPI: u16 = 300;
const SIZE: pdf::Size = pdf::Size::Px(1920, 1080);
const DRAW_AREA: config::Rectangle<f64> = config::Rectangle {
    orig: config::Point { x: 0.05, y: 0.05 },
    size: config::Point { x: 0.9, y: 0.9 },
};

pub struct PdfMaker {
    doc: pdf::Document,
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
        self.doc.save(to).map_err(|e| e.into())
    }
}

impl PdfMaker {
    /// creates a pdf maker with information from the
    /// config
    pub fn with_config(config: &Config<'_>) -> DResult<Self> {
        let doc = pdf::Document::new(config.doc_name, SIZE, DRAW_AREA, DPI)?;
        let drawer = Self { doc, slide_idx: 0 };

        Ok(drawer)
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
        let mut page = self.doc.new_page(self.slide_idx.to_string());

        Self::draw_decorations(&mut page, &kind.decorations, config)?;
        Self::draw_content(&mut page, &kind.content, slide, &config.font)
    }

    /// draws the given decoration a slide to the pdf layer
    fn draw_decorations(
        page: &mut pdf::Page,
        decos: &Vec<Decoration>,
        config: &Config<'_>,
    ) -> DResult<()> {
        for d in decos.into_iter() {
            let area = page.doc.scale_pdf_rect(d.area.clone());
            let color = config.get_color(d.color_idx)?;
            page.draw_rect(&area, Some(color.into()), None)
        }

        Ok(())
    }

    /// draws the content of a slide to the pdf page
    fn draw_content(
        page: &mut pdf::Page,
        contents: &Vec<SlideTemplate>,
        slide: Slide,
        font: &str,
    ) -> DResult<()> {
        for (template, content) in contents.iter().zip(slide.contents.into_iter()) {
            let area = page.doc.scale_pdf_rect(template.area.clone());
            let args = pdf::TextArgs {
                area,
                font_size: template.font_size as f64,
                font,
                orientation: &template.orientation,
            };

            match content {
                Content::Text(s) => {
                    page.draw_text(&args, s)?;
                }
                Content::Config(_) => panic!("Config calls should be handled before drawing"),
                Content::Image(_, _p) => {
                    // TODO: add description
                    page.new_layer("imaage");
                }
                Content::List(_i) => (), //self.list(i, args),
            }
        }

        Ok(())
    }

    fn list(page: &mut pdf::Page, items: Vec<(u8, String)>, mut args: pdf::TextArgs) {
        /*let orig_x = args.area.orig.0;
        let space = Pt(args.font_size);

        let mut pt_written = Pt(0.0);

        for (ident, text) in items {
            // TODO: draw something to indicate list
            args.area.orig.0 = orig_x + space * ident as f64;
            self.text(String::from("-"), &args);
            args.area.orig.0 = orig_x + space * (ident + 1) as f64;
            args.area.orig.1 += pt_written;
            args.area.size.1 -= pt_written;
            pt_written = self.text(text, &args);
        }*/
    }

    /*fn image(
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
    }*/
}
