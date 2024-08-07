use std::fmt::Display;

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

    start_line: u64,
}

impl TapShape {
    pub fn new(line: u64) -> TapShape {
        TapShape {
            shape_id: Option::None,
            shape_label: Option::None,
            statements: Vec::new(),
            start_line: line,
        }
    }

    pub fn shape_id(&self) -> Option<ShapeId> {
        self.shape_id.clone()
    }

    pub fn shape_label(&self) -> Option<String> {
        self.shape_label.clone()
    }

    pub fn set_start_line(&mut self, line: u64) {
        self.start_line = line;
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

    pub fn start_line(&self) -> u64 {
        self.start_line
    }
}

impl Display for TapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Shape({}) {}",
            self.shape_id().unwrap_or_else(|| ShapeId::new("", 0)),
            self.shape_label().unwrap_or_default()
        )?;
        for statement in self.statements() {
            writeln!(f, " {statement}")?;
        }
        Ok(())
    }
}
