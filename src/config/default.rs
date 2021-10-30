use super::*;

impl<'a> Default for super::Config<'a> {
    fn default() -> Self {
        Self {
            style: Default::default(),
            slide_templates: default_slide_templates(),
            doc_name: "presentation",
        }
    }
}

pub fn default_slide_templates() -> StyleMap {
    let header_orientation = Orientation {
        vertical: VertOrientation::Bottom,
        horizontal: HorOrientation::Middle,
    };
    crate::map! {
        "Title" => SlideTemplate {
            decorations: vec![],
            content: vec![
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.0},
                        size: Point{x: 1.0,y: 0.8} },
                    font_size: 36.0,
                    orientation: header_orientation.clone(),
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.8},
                        size: Point{x: 1.0,y: 0.2} },
                    font_size: 18.0,
                    orientation: Orientation::default(),
                },
            ],
        },

        "Head_Cont" => SlideTemplate {
            decorations: vec![],
            content: vec![
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.0},
                        size: Point{x: 1.0,y: 0.3},
                    },
                    font_size: 24.0,
                    orientation: header_orientation.clone(),
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.3},
                        size: Point{x: 1.0,y: 0.7},
                    },
                    font_size: 18.0,
                    orientation: Orientation::default(),
                },
            ],
        },

        "Vert_Split" => SlideTemplate {
            decorations: vec![],
            content: vec![
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.0},
                        size: Point{x: 0.5,y: 0.3},
                    },
                    font_size: 24.0,
                    orientation: header_orientation.clone(),
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.3},
                        size: Point{x: 0.5,y: 0.7},
                    },
                    font_size: 18.0,
                    orientation: Orientation::default(),
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.5,y: 0.0},
                        size: Point{x: 0.5,y: 0.3},
                    },
                    font_size: 24.0,
                    orientation: header_orientation,
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.5,y: 0.3},
                        size: Point{x: 0.5,y: 0.7},
                    },
                    font_size: 18.0,
                    orientation: Orientation::default(),
                },
            ],
        },
        "Two_Hor" => SlideTemplate {
            decorations: vec![],
            content: vec![
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.0},
                        size: Point{x: 1.0,y: 0.5},
                    },
                    font_size: 20.0,
                    orientation: Orientation::default(),
                },
                ContentTemplate {
                    area: Rectangle {
                        orig: Point{x: 0.0,y: 0.5},
                        size: Point{x: 1.0,y: 0.5},
                    },
                    font_size: 20.0,
                    orientation: Orientation::default(),
                },
            ],
        },
    }
}
