use super::{Point, Rectangle};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct StyleJson {
    colors: Vec<String>,
    font: String,
    #[serde(rename = "lineSpace")]
    line_spacing: f64,
}

#[derive(Deserialize)]
pub struct TemplateJson {
    margin: Rectangle<f64>,
    slides: BTreeMap<String, SlideTemplate>,
}

#[derive(Deserialize)]
pub struct SlideTemplate {
    decoration: Vec<DecorationJson>,
    template: Vec<ContentTemplate>,
}

#[derive(Deserialize)]
pub struct ContentTemplate {
    // TODO: change to upper left and lower right
    orig: Point<f64>,
    size: Point<f64>,
    orientation: String,
    #[serde(rename = "fontSize")]
    font_size: f64,
}

#[derive(Deserialize)]
pub struct DecorationJson {
    // TODO: change to upper left and lower right
    orig: Point<f64>,
    size: Point<f64>,
    color: usize,
}
