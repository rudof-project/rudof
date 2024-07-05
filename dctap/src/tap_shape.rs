use crate::tap_statement::TapStatement;
use crate::ShapeId;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct TapShape {
    #[serde(rename = "shapeID")]
    shape_id: Option<ShapeId>,

    #[serde(rename = "shapeLabel")]
    shape_label: Option<String>,

    statements: Vec<TapStatement>,
}

impl TapShape {
    pub fn new() -> TapShape {
        TapShape {
            shape_id: Option::None,
            shape_label: Option::None,
            statements: Vec::new(),
        }
    }

    pub fn shape_id(&self) -> Option<ShapeId> {
        self.shape_id.clone()
    }

    pub fn set_shape_id(&mut self, shape_id: &ShapeId) {
        self.shape_id = Some(shape_id.clone());
        // Reset the statements because we have a new shape
        self.statements = Vec::new();
    }

    pub fn set_shape_label(&mut self, shape_label: &str) {
        self.shape_label = Some(shape_label.to_string());
        // Reset the statements because we have a new shape
        self.statements = Vec::new();
    }

    pub fn add_statement(&mut self, statement: TapStatement) {
        self.statements.push(statement.clone());
    }

    pub fn statements(&self) -> impl Iterator<Item = &TapStatement> {
        self.statements.iter()
    }
}
