use crate::config;
use printpdf::{Mm, Pt, Px};

impl From<config::Color> for printpdf::Color {
    fn from(c: config::Color) -> Self {
        printpdf::Color::Rgb(printpdf::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
            icc_profile: None,
        })
    }
}

/// gets the index of a certain sub-string
/// the two references need to reference the same
/// original String so that function works
///
/// # Panics
/// the function panics when origin is
/// after the given reference to
pub fn get_index_of(to: &str, origin: &str) -> usize {
    to.as_ptr() as usize - origin.as_ptr() as usize
}

/*const INCHES_PER_POINT: f64 = 72.0;

pub fn pt_to_px(pt: f64, dpi: u16) -> f64 {
    pt * dpi as f64 / INCHES_PER_POINT
}*/

#[allow(dead_code)]
/// different ways how the size of an pdf document
/// can be described. Here it is always (x, y)
pub enum Size {
    Mm(f64, f64),
    Px(usize, usize),
    Pt(f64, f64),
}

impl Size {
    /// converts the size enum to a tuple
    /// with the size in Mm
    pub fn to_mm(&self, dpi: u16) -> (Mm, Mm) {
        let px_to_mm = |x| Mm::from(Px(x).into_pt(dpi as f64));

        match *self {
            Size::Mm(x, y) => (Mm(x), Mm(y)),
            Size::Pt(x, y) => (Pt(x).into(), Pt(y).into()),
            Size::Px(x, y) => (px_to_mm(x), px_to_mm(y)),
        }
    }
}

/// a simple wrapper around a rusttype font
/// with metadata to scale it and get the right line height for that font
pub struct RtFont<'a> {
    inner: rusttype::Font<'a>,
    scale: rusttype::Scale,
    line_height: Pt,
}

impl<'a> RtFont<'a> {
    /// wraps the given font into a RtFont
    pub fn from_rt(font: rusttype::Font<'a>) -> Self {
        let v_metrics = font.v_metrics_unscaled();
        let line_height = (v_metrics.ascent - v_metrics.descent/*+ v_metrics.line_gap*/)
            / font.units_per_em() as f32;
        Self {
            inner: font,
            scale: rusttype::Scale::uniform(line_height),
            line_height: Pt(line_height as f64),
        }
    }

    /// gets the width of every char (glyph) in the iterator + kerning from the current and
    /// the last glyph
    pub fn text_width<'b, I>(&'b self, font_size: f32, text: I) -> impl Iterator<Item = f32> + 'b
    where
        I: Iterator<Item = char> + 'b,
    {
        let rt_font = &self.inner;
        rt_font
            .glyphs_for(text)
            .scan(None, move |last: &mut Option<rusttype::GlyphId>, g| {
                let kerning = if let Some(last) = last {
                    rt_font.pair_kerning(self.scale, *last, g.id())
                } else {
                    0.0
                };

                *last = Some(g.id());
                // gets the width of the glyph
                let width = self.get_width(font_size, g.id());

                // the total width is the glyph itself and also the space since the last glyph
                Some(kerning + width)
            })
    }

    /// get the width of a glyph at a certain font_size
    pub fn get_width<G: rusttype::IntoGlyphId>(&self, font_size: f32, glyph: G) -> f32 {
        self.inner
            .glyph(glyph)
            .scaled(self.scale)
            .h_metrics()
            .advance_width
            * font_size
    }
}

use super::TextArgs;
use arrayvec::ArrayVec;

/// data for drawing individual lines
pub struct LineData {
    pub end_index: usize,
    pub width: f32,
}

/// data to better calculate the beginning
/// position of the next line
pub struct PositionArgs<'a> {
    text_args: &'a TextArgs<'a>,
    pub line_height: f64,
    lines: &'a ArrayVec<LineData, 64>,
}

impl<'a> PositionArgs<'a> {
    /// bundles the arguments into a PositionArgs struct together
    pub fn new(
        args: &'a TextArgs<'a>,
        lines: &'a ArrayVec<LineData, 64>,
        font: &RtFont<'_>,
    ) -> Self {
        Self {
            lines,
            line_height: font.line_height.0 * args.font_size,
            text_args: args,
        }
    }

    /// calculates the position a certain line should be drawn at
    pub fn get_position(&self, line_idx: usize) -> config::Point<Mm> {
        let orientation = self.text_args.orientation;
        let area = &self.text_args.area.0;
        let size = area.size;

        use config::HorOrientation as Hor;
        use config::VertOrientation as Vert;

        let y = match orientation.vertical {
            Vert::Top => size.y.0 - (line_idx + 1) as f64 * self.line_height,
            Vert::Middle => size.y.0 / 2.0 - line_idx as f64 * self.line_height,
            Vert::Bottom => (self.lines.len() - (line_idx + 1)) as f64 * self.line_height,
        };

        let width = self.lines[line_idx].width;
        let x = match orientation.horizontal {
            Hor::Left => 0.0,
            Hor::Middle => (size.x.0 - width as f64) / 2.0,
            Hor::Right => size.x.0 - width as f64,
        };

        let pos = config::Point { x: Pt(x), y: Pt(y) } + area.orig;
        //dbg!(pos);
        pos.map(|pt| pt.into())
    }
}
