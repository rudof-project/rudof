use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Default)]

pub struct LandingHtmlTemplate {
    pub title: String,
    pub rudof_version: String,
    pub created_time: String,
    pub shapes: Vec<ShapeRef>,
    pub svg_schema: String,
}

#[derive(Serialize, Debug, PartialEq, Default)]

pub struct ShapeRef {
    name: String,
    href: String,
    label: String,
}

impl ShapeRef {
    pub fn new(name: &str, href: &str, label: &str) -> ShapeRef {
        ShapeRef {
            name: name.to_string(),
            href: href.to_string(),
            label: label.to_string(),
        }
    }
}
