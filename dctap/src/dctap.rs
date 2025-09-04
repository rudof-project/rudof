use crate::{
    // TapReader,
    TapReaderBuilder,
    TapShape,
    tap_config::TapConfig,
    tap_error::TapError,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, io, path::Path};
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
struct TapShapeId(String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DCTap {
    version: String,

    shapes: Vec<TapShape>,
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
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: &TapShape) {
        self.shapes.push(shape.clone());
    }

    pub fn from_path<P: AsRef<Path>>(path: P, config: &TapConfig) -> Result<DCTap, TapError> {
        let mut dctap = DCTap::new();
        let mut tap_reader = TapReaderBuilder::from_path(path, config)?;
        for maybe_shape in tap_reader.shapes() {
            let shape = maybe_shape?;
            dctap.add_shape(&shape)
        }
        if tap_reader.has_warnings() {
            for warning in tap_reader.warnings() {
                info!("Warning: {warning}");
            }
        }
        Ok(dctap)
    }

    pub fn from_reader<R: io::Read>(reader: R, config: &TapConfig) -> Result<DCTap, TapError> {
        let mut dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        let mut tap_reader = TapReaderBuilder::from_reader(reader, config)?;
        for maybe_shape in tap_reader.shapes() {
            let shape = maybe_shape?;
            dctap.add_shape(&shape)
        }
        Ok(dctap)
    }

    /*pub fn from_excel<P: AsRef<Path>>(
        path: P,
        sheet_name: Option<&str>,
        config: &TapConfig,
    ) -> Result<DCTap, TapError> {
        let mut dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        let mut tap_reader: TapReader<io::Empty> =
            TapReaderBuilder::from_excel(path, sheet_name, config)?;
        for maybe_shape in tap_reader.shapes() {
            let shape = maybe_shape?;
            dctap.add_shape(&shape)
        }
        Ok(dctap)
    }*/

    pub fn shapes(&self) -> impl Iterator<Item = &TapShape> {
        self.shapes.iter()
    }
}

impl Display for DCTap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for shape in self.shapes() {
            write!(f, "{shape}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{PropertyId, ShapeId, TapShape, TapStatement};

    use super::*;

    #[test]
    fn test_simple() {
        let data = "\
shapeId,shapeLabel,propertyId,propertyLabel
Person,PersonLabel,knows,knowsLabel
";
        let dctap = DCTap::from_reader(data.as_bytes(), &TapConfig::default()).unwrap();
        let mut expected_shape = TapShape::new(2);
        expected_shape.set_shape_id(&ShapeId::new("Person", 2));
        expected_shape.set_shape_label("PersonLabel");
        let mut statement =
            TapStatement::new(PropertyId::new("knows", 2)).with_source_line_number(2);
        statement.set_property_label("knowsLabel");
        expected_shape.add_statement(statement);
        let mut expected_dctap = DCTap::new();
        expected_dctap.add_shape(&expected_shape);
        assert_eq!(dctap, expected_dctap);
    }
}
