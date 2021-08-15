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
        let drawer = Self { doc };

        Ok(drawer)
    }

    /// draws a slide with the information from the config
    fn draw_slide(&mut self, slide: Slide, config: &Config<'_>) -> DResult<()> {
        // get info of how the slide should be drawn
        let kind = config
            .slide_styles
            .get(&slide.kind)
            .ok_or_else(|| DrawError::KindNotFound(slide.kind.clone()))?;

        // create the new pdf page for the slide
        let mut page = self.doc.new_page("");

        Self::draw_decorations(&mut page, &kind.decorations, config)?;
        Self::draw_content(&mut page, &kind.content, slide, &config.font)
    }

    /// draws the given decoration a slide to the pdf layer
    fn draw_decorations(
        page: &mut pdf::Page,
        decos: &[Decoration],
        config: &Config<'_>,
    ) -> DResult<()> {
        for d in decos.iter() {
            let area = page.doc.scale_pdf_rect(d.area.clone());
            let color = config.get_color(d.color_idx)?;

            page.draw_rect(&area, Some(color.into()), None)
        }

        Ok(())
    }

    /// draws the content of a slide to the pdf page
    fn draw_content(
        page: &mut pdf::Page,
        contents: &[SlideTemplate],
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
                    page.draw_text(&args, &s)?;
                }
                Content::Config(_) => panic!("Config calls should be handled before drawing"),
                Content::Image(_, _p) => {
                    // TODO: add description
                    page.new_layer("imaage");
                }
                Content::List(i) => Self::list(page, i, args)?,
            }
        }

        Ok(())
    }

    fn list(
        page: &mut pdf::Page,
        items: Vec<(u8, String)>,
        mut args: pdf::TextArgs,
    ) -> DResult<()> {
        use printpdf::Pt;
        let ident_width = page.doc.get_width("-", args.font_size, args.font)?;
        let orig = *args.area.origin();

        // TODO: make lists work with different orientations
        for (ident, text) in items {
            page.new_layer("please end my suffering");
            dbg!(ident);
            // the ident of the list item and drawing the symbol
            let mut ident_pos = orig
                + config::Point {
                    x: dbg!(ident_width * ident as f64),
                    y: Pt(0.0),
                };
            page.doc.set_lower_left(&mut args.area, dbg!(ident_pos));
            page.draw_text(&args, "-")?;
            ident_pos.x += ident_width;
            //dbg!(ident_pos.x);
            page.doc.set_lower_left(&mut args.area, dbg!(ident_pos));

            // TODO: move the area to the right according to the ident

            // writing the item and move down to the next item
            let pt_written = page.draw_text(&args, &text)?;
            // decrease the height of the area
            page.doc
                .move_upper_right(&mut args.area, (Pt(0.0), Pt(0.0) - pt_written).into());
        }

        Ok(())
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
