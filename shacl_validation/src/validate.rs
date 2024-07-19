use anyhow::*;
use shacl_ast::{node_shape::NodeShape, property_shape::PropertyShape, shape::Shape, Schema};
use srdf::SRDFGraph;

use crate::{
    constraints::{ConstraintFactory, Evaluate},
    validation_report::{ValidationReport, ValidationResult},
};

trait Validate {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>>;
}

impl Validate for Shape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        match self {
            Shape::NodeShape(node_shape) => node_shape.validate(data_graph),
            Shape::PropertyShape(property_shape) => property_shape.validate(data_graph),
        }
    }
}

impl Validate for NodeShape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return None;
        }

        let mut results = Vec::new();

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            if let Some(result) = constraint.evaluate() {
                results.push(result)
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}

impl Validate for PropertyShape {
    fn validate(&self, data_graph: &SRDFGraph) -> Option<Vec<ValidationResult>> {
        if *self.is_deactivated() {
            return None;
        }

        let mut results = Vec::new();

        for component in self.components() {
            let constraint = ConstraintFactory::new_constraint(component);

            if let Some(result) = constraint.evaluate() {
                results.push(result)
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}

pub fn validate(data_graph: &SRDFGraph, shapes_graph: Schema) -> Result<ValidationReport> {
    let mut validation_report = ValidationReport::default(); // conforming by default...
    for (_, shape) in shapes_graph.iter() {
        let result = shape.validate(data_graph); // TODO: Extend traits
        if let Some(result) = result {
            validation_report.set_non_conformant();
            validation_report.extend_result(result);
        }
    }
    Ok(validation_report)
}
