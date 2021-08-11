use crate::config;
use arrayvec::ArrayVec;
use printpdf::{Mm, Pt};
use std::collections::HashMap;
use std::io;

mod error;
mod util;

pub use error::PdfError;

#[allow(dead_code)]
pub enum Size {
    Mm(f64, f64),
    Px(f64, f64),
    Pt(f64, f64),
}

impl Size {
    fn to_mm(self, dpi: u16) -> (Mm, Mm) {
        let px_to_mm = |x| Mm::from(Pt(util::px_to_pt(x, dpi)));

        match self {
            Size::Mm(x, y) => (Mm(x), Mm(y)),
            Size::Pt(x, y) => (Pt(x).into(), Pt(y).into()),
            Size::Px(x, y) => (px_to_mm(x), px_to_mm(y)),
        }
    }
}

/// a rectangle inside the pdf document
/// (bottom-left)
#[derive(Debug, PartialEq)]
pub struct PdfRect(config::Rectangle<Pt>);

impl PdfRect {
    /// creates an pdf rectangle from a "scalor" rectangle
    fn from(r: config::Rectangle<f64>, size: (Pt, Pt)) -> Self {
        let config::Rectangle {
            mut orig,
            size: r_size,
        } = r;

        // make it left bottom
        orig.y = 1.0 - (orig.y + r_size.y);

        Self(config::Rectangle {
            orig: Self::scale_to_pt(orig, &size),
            size: Self::scale_to_pt(r_size, &size),
        })
    }

    /// scales the point from the given size
    fn scale_to_pt(
        config::Point { x, y }: config::Point<f64>,
        size: &(Pt, Pt),
    ) -> config::Point<Pt> {
        config::Point {
            x: size.0 * x,
            y: size.1 * y,
        }
    }

    /// constructs all the points for drawing inside printpdf
    fn to_points(&self) -> Vec<(printpdf::Point, bool)> {
        let point = |x, y| (printpdf::Point { x, y }, false);
        let config::Rectangle { orig: o, size: s } = self.0;
        vec![
            point(o.x, o.y),
            point(o.x + s.x, o.y),
            point(o.x + s.x, o.y + s.y),
            point(o.x, o.y + s.y),
        ]
    }
}

pub struct TextArgs<'a> {
    pub area: PdfRect,
    pub font_size: f64,
    pub font: &'a str,
    pub orientation: &'a config::Orientation,
}

pub struct Document {
    /// a map to the index of a font
    /// fontname -> index
    font_map: HashMap<String, usize>,
    /// all fonts loaded as the printpdf format
    pdf_fonts: Vec<printpdf::IndirectFontRef>,
    /// all fonts loaded as the rusttype format
    rt_fonts: Vec<rusttype::Font<'static>>,
    /// fontconfig for finding the font paths
    font_config: fontconfig::Fontconfig,

    /// the printpdf document
    inner_doc: printpdf::PdfDocumentReference,
    size: (Mm, Mm),
    drawing_area: PdfRect,
    dpi: u16,
}

// redefine for easier use in this module
type Result<T, E = PdfError> = std::result::Result<T, E>;

impl Document {
    pub fn new<S: Into<String>>(
        name: S,
        size: Size,
        drawing_area: config::Rectangle<f64>,
        dpi: u16,
    ) -> Result<Self> {
        let size = size.to_mm(dpi);
        let pt_size = (size.0.into(), size.1.into());
        Ok(Self {
            size,
            drawing_area: dbg!(PdfRect::from(drawing_area, pt_size)),
            dpi,
            font_map: Default::default(),
            pdf_fonts: vec![],
            rt_fonts: vec![],
            font_config: fontconfig::Fontconfig::new().ok_or(PdfError::NoFontConfig)?,
            inner_doc: printpdf::PdfDocument::empty(name),
        })
    }

    pub fn save<W: io::Write>(self, to: W) -> Result<(), printpdf::Error> {
        let mut buf_writer = io::BufWriter::new(to);
        self.inner_doc.save(&mut buf_writer)
    }

    /// add a new page to the document all future operation will be done
    /// on that new page
    pub fn new_page<S: Into<String>>(&'_ mut self, name: S) -> Page<'_> {
        let (page, layer) = self.inner_doc.add_page(self.size.0, self.size.1, name);
        let page = self.inner_doc.get_page(page);
        let layer = page.get_layer(layer);

        let page = Page {
            doc: self,
            page,
            layer,
        };

        #[cfg(debug_assertions)]
        page.draw_rect(&page.doc.drawing_area, None, Some(Page::DBG_COLOR));

        page
    }

    pub fn scale_pdf_rect(&self, area: config::Rectangle<f64>) -> PdfRect {
        let draw_area_size = self.drawing_area.0.size.into();
        let mut tmp = PdfRect::from(area, draw_area_size);
        tmp.0.orig += self.drawing_area.0.orig;
        tmp
    }

    fn fonts(&self, name: &str) -> (&printpdf::IndirectFontRef, &rusttype::Font<'static>) {
        let index = *self.font_map.get(name).unwrap_or(&0);
        (&self.pdf_fonts[index], &self.rt_fonts[index])
    }

    fn maybe_load_font(&mut self, name: &str) -> Result<()> {
        // it is already loaded
        if self.font_map.contains_key(name) {
            return Ok(());
        }

        // find the font path
        let path = self
            .font_config
            .find(name, None)
            .ok_or_else(|| PdfError::FontNotFound(String::from(name)))?
            .path;

        // read the font with printpdf and rusttype
        let (pdf_font, rt_font) = std::fs::read(&path)
            .ok()
            .and_then(|data| {
                Some((
                    self.inner_doc.add_external_font(&data[..]).ok()?,
                    rusttype::Font::try_from_vec(data)?,
                ))
            })
            // give the path to the font if it couldn't be loaded
            .ok_or_else(|| PdfError::FontNotLoaded(path.to_string_lossy().into()))?;

        // add the fonts to the map and lists
        self.rt_fonts.push(rt_font);
        self.pdf_fonts.push(pdf_font);
        let index = self.rt_fonts.len() - 1;
        self.font_map.insert(String::from(name), index);

        Ok(())
    }
}

struct LineData {
    end_index: usize,
    width: f32,
}

struct PositionArgs<'a> {
    text_args: &'a TextArgs<'a>,
    font_height: f64,
    lines: &'a ArrayVec<LineData, 64>,
}

impl<'a> PositionArgs<'a> {
    fn new(args: &'a TextArgs<'a>, lines: &'a ArrayVec<LineData, 64>, dpi: u16) -> Self {
        Self {
            lines,
            // TODO: needs to be changed
            font_height: dbg!(util::px_to_pt(args.font_size, dpi)) * 2.0,
            text_args: args,
        }
    }
}

pub struct Page<'a> {
    pub doc: &'a mut Document,
    page: printpdf::PdfPageReference,
    layer: printpdf::PdfLayerReference,
}

impl<'a> Page<'a> {
    pub fn new_layer<S: Into<String>>(&mut self, name: S) {
        self.layer = self.page.add_layer(name);
    }

    const DBG_COLOR: printpdf::Color = printpdf::Color::Rgb(printpdf::Rgb {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        icc_profile: None,
    });

    pub fn draw_rect(
        &self,
        rect: &PdfRect,
        fill_color: Option<printpdf::Color>,
        stroke_color: Option<printpdf::Color>,
    ) {
        let layer = &self.layer;
        let line = printpdf::Line {
            points: rect.to_points(),
            is_closed: true,
            has_fill: fill_color.is_some(),
            has_stroke: stroke_color.is_some(),
            is_clipping_path: false,
        };

        // set the color
        fill_color.map(|c| layer.set_fill_color(c));
        stroke_color.map(|c| layer.set_outline_color(c));

        // and draw it
        layer.add_shape(line)
    }

    pub fn draw_text(&mut self, args: &TextArgs<'_>, text: String) -> Result<Pt> {
        // draw the box outlines in debug mode
        #[cfg(debug_assertions)]
        self.draw_rect(&args.area, None, Some(Self::DBG_COLOR));

        // get the fonts
        self.doc.maybe_load_font(args.font)?;
        let (pdf_font, rt_font) = self.doc.fonts(args.font);

        // reassign for readability
        let width = args.area.0.size.x.0;
        let font_size = args.font_size;
        let dpi = self.doc.dpi;
        let layer = &self.layer;

        // PANICS: content with more than 64 lines should be a sin
        // TODO: maybe use Vec for better memory usage
        let beginnings: ArrayVec<_, 64> =
            Self::get_lines(rt_font, &text, font_size as f32, dpi, width).collect();
        let mut pos_args = PositionArgs::new(args, &beginnings, dpi);

        layer.begin_text_section();

        layer.set_font(&pdf_font, font_size);
        // TODO: maybe needs to be changed
        layer.set_line_height(font_size);

        let mut i = 0;
        let mut start = 0; // start at index 0, duh

        for line in beginnings.iter() {
            // prepare line
            let end = line.end_index;
            let pos = self.get_position(i, &mut pos_args);
            layer.set_text_cursor(pos.x, pos.y);

            dbg!(line.width, start, end, &text[start..end]);
            layer.write_text(&text[start..end], &pdf_font);
            layer.add_line_break();
            start = end;
            i += 1;
        }

        layer.end_text_section();

        Ok(Pt((i + 1) as f64 * font_size))
    }

    fn get_lines<'b>(
        font: &'b rusttype::Font<'static>,
        text: &'b str,
        font_size: f32,
        dpi: u16,
        width: f64,
    ) -> impl Iterator<Item = LineData> + 'b {
        let width = util::pt_to_px(width, dpi);
        // TODO: do something about font
        let font_size = util::pt_to_px(font_size as f64, dpi) as f32;

        font.layout(
            text,
            rusttype::Scale::uniform(font_size),
            Default::default(),
        )
        // get the width of every glyph
        .map(move |g| g.into_unpositioned().h_metrics().advance_width)
        // build partial sums
        .scan(0.0, |state, w| {
            *state += w;
            Some(*state)
        })
        .enumerate()
        .filter_map(is_line_end(width as f32, text.len(), dpi))
    }

    fn get_position(&self, line_idx: usize, args: &mut PositionArgs<'_>) -> config::Point<Mm> {
        let orientation = dbg!(args.text_args.orientation);
        let area = &args.text_args.area.0;
        let size = dbg!(area.size);

        use config::HorOrientation as Hor;
        use config::VertOrientation as Vert;
        // TODO: don't forget your paper
        let y = match orientation.vertical {
            Vert::Top => size.y.0 - (line_idx + 1) as f64 * args.font_height,
            Vert::Middle => size.y.0 / 2.0 - line_idx as f64 * args.font_height,
            Vert::Bottom => (line_idx + 1) as f64 * args.font_height,
        };

        let width = args.lines[line_idx].width;
        let x = match orientation.horizontal {
            Hor::Left => 0.0,
            Hor::Middle => (size.x.0 - width as f64) / 2.0,
            Hor::Right => size.x.0 - width as f64,
        };

        let pos = config::Point { x: Pt(x), y: Pt(y) } + dbg!(area.orig);
        dbg!(pos);
        pos.map(|pt| pt.into())
    }
}

fn is_line_end(
    line_width: f32,
    str_len: usize,
    dpi: u16,
) -> impl FnMut((usize, f32)) -> Option<LineData> {
    let mut last_line: f32 = 0.0;
    move |(i, sum)| {
        let this_line = sum - last_line;

        let index = if this_line > line_width {
            last_line = sum;
            i
        // the end
        } else if i == str_len - 1 {
            str_len
        } else {
            return None;
        };

        Some(LineData {
            end_index: index,
            // TODO: something something font
            width: util::px_to_pt(this_line as f64, dpi) as f32,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Point, Rectangle};
    use printpdf::{Mm, Pt};

    fn equal_within_error(left: f64, right: f64) {
        assert!((left - right).abs() < 0.001)
    }

    const DPI: u16 = 96;

    #[test]
    fn size_mm_to_mm() {
        let left = 24.3;
        let right = 12.2;

        assert_eq!(
            super::Size::Mm(left, right).to_mm(DPI),
            (Mm(left), Mm(right))
        );
    }

    #[test]
    fn size_pt_to_mm() {
        let left = 24.3;
        let right = 12.2;
        let (result_x, result_y) = super::Size::Pt(left, right).to_mm(DPI);
        let expected_x = 8.5725;
        let expected_y = 4.30388;

        equal_within_error(result_x.0, expected_x);
        equal_within_error(result_y.0, expected_y);
    }

    #[test]
    fn size_px_to_mm() {
        let left = 1920.0;
        let right = 1080.0;
        let (result_x, result_y) = super::Size::Px(left, right).to_mm(DPI);
        let expected_x = 508.0;
        let expected_y = 285.75;

        equal_within_error(result_x.0, expected_x);
        equal_within_error(result_y.0, expected_y);
    }

    const RECT_SIZE: (Pt, Pt) = (Pt(100.0), Pt(100.0));
    #[test]
    fn rect_upperleft_origin() {
        let rect = Rectangle {
            orig: Point { x: 0.0, y: 0.0 },
            size: Point { x: 1.0, y: 1.0 },
        };

        let expected = Rectangle {
            orig: Point {
                x: Pt(0.0),
                y: Pt(0.0),
            },
            size: Point {
                x: RECT_SIZE.0,
                y: RECT_SIZE.1,
            },
        };

        assert_eq!(
            super::PdfRect::from(rect, RECT_SIZE),
            super::PdfRect(expected)
        );
    }
}
