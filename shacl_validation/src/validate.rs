use std::path::Path;

use shacl_ast::shape::Shape;
use srdf::{RDFFormat, SRDFBasic, SRDFGraph, SRDFSparql, SRDF};

use crate::helper::srdf::load_shapes_graph;
use crate::runner::{GraphValidatorRunner, SparqlValidatorRunner, ValidatorRunner};
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub trait Validator<'a, S: SRDF + SRDFBasic + 'static> {
    fn runner(&self) -> &impl ValidatorRunner<S>;
    fn base(&self) -> Option<&'a str>;

    fn validate(
        &self,
        shapes: &Path,
        shapes_format: RDFFormat,
    ) -> Result<ValidationReport<S>, ValidateError> {
        let schema = load_shapes_graph(shapes, shapes_format, self.base())?;

        let mut ans = ValidationReport::default(); // conformant by default...
        for (_, shape) in schema.iter() {
            match shape {
                Shape::NodeShape(shape) => shape.validate(self.runner(), &mut ans)?,
                Shape::PropertyShape(shape) => shape.validate(self.runner(), &mut ans)?,
            };
        }
        Ok(ans)
    }
}

pub struct GraphValidator<'a> {
    runner: GraphValidatorRunner,
    base: Option<&'a str>,
}

impl<'a> GraphValidator<'a> {
    pub fn new(
        data: &Path,
        data_format: RDFFormat,
        base: Option<&'a str>,
    ) -> Result<Self, ValidateError> {
        Ok(GraphValidator {
            runner: GraphValidatorRunner::new(data, data_format, base)?,
            base,
        })
    }
}

impl<'a> Validator<'a, SRDFGraph> for GraphValidator<'a> {
    fn runner(&self) -> &impl ValidatorRunner<SRDFGraph> {
        &self.runner
    }

    fn base(&self) -> Option<&'a str> {
        self.base
    }
}

pub struct SparqlValidator<'a> {
    runner: SparqlValidatorRunner,
    base: Option<&'a str>,
}

impl<'a> SparqlValidator<'a> {
    pub fn new(data: &String) -> Result<Self, ValidateError> {
        Ok(SparqlValidator {
            runner: SparqlValidatorRunner::new(data)?,
            base: None,
        })
    }
}

impl<'a> Validator<'a, SRDFSparql> for SparqlValidator<'a> {
    fn runner(&self) -> &impl ValidatorRunner<SRDFSparql> {
        &self.runner
    }

    fn base(&self) -> Option<&'a str> {
        self.base
    }
}
