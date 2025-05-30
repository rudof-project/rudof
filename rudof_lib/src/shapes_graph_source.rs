#[derive(Debug, PartialEq, Clone, Default)]
pub enum ShapesGraphSource {
    #[default]
    CurrentData,
    CurrentSchema,
}

impl ShapesGraphSource {
    pub fn new() -> ShapesGraphSource {
        Self::default()
    }

    pub fn current_schema() -> ShapesGraphSource {
        ShapesGraphSource::CurrentSchema
    }

    pub fn current_data() -> ShapesGraphSource {
        ShapesGraphSource::CurrentData
    }
}
