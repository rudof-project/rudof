use crate::{tap_config::TapConfig, tap_error::TapError, ShapeId, TapReaderBuilder, TapShape};
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use std::{io, path::Path};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
struct TapShapeId(String);

#[derive(Debug, Serialize, PartialEq)]
pub struct DCTap {
    version: String,
    shapes: IndexMap<Option<ShapeId>, TapShape>,
}

impl Default for DCTap {
    fn default() -> Self {
        Self::new()
    }
}

impl DCTap {
    pub fn new() -> DCTap {
        DCTap {
            version: "0.1".to_string(),
            shapes: IndexMap::new(),
        }
    }

    pub fn add_shape(&mut self, shape: &TapShape) {
        self.shapes.insert(shape.shape_id(), shape.clone());
    }

    pub fn from_path(path: &Path, _config: TapConfig) -> Result<DCTap, TapError> {
        let mut dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        let mut tap_reader = TapReaderBuilder::new().flexible(true).from_path(path)?;
        for maybe_shape in tap_reader.shapes() {
            let shape = maybe_shape?;
            println!("Shape read: {shape:?}");
            dctap.add_shape(&shape)
        }
        Ok(dctap)
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<DCTap, TapError> {
        let mut dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        let mut tap_reader = TapReaderBuilder::new().flexible(true).from_reader(reader)?;
        for maybe_shape in tap_reader.shapes() {
            let shape = maybe_shape?;
            println!("Shape read: {shape:?}");
            dctap.add_shape(&shape)
        }
        Ok(dctap)
    }
}

#[cfg(test)]
mod tests {
    use crate::{PropertyId, TapShape, TapStatement};

    use super::*;

    #[test]
    fn test_simple() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,KnowsLabel
";
        let dctap = DCTap::from_reader(data.as_bytes()).unwrap();
        let mut expected_shape = TapShape::new();
        expected_shape.set_shape_id(&ShapeId::new("Person"));
        expected_shape.add_statement(TapStatement::new(PropertyId::new("knows")));

        let mut expected_dctap = DCTap::new();
        expected_dctap.add_shape(&expected_shape);
        assert_eq!(dctap, expected_dctap);
    }
}
