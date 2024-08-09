use std::path::Path;

use clap::ValueEnum;
use shacl_ast::shape::Shape;
use srdf::{RDFFormat, SRDFBasic, SRDFGraph, SRDFSparql};

use crate::helper::srdf::load_shapes_graph;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::shape::Validate;
use crate::store::graph::Graph;
use crate::store::sparql::Sparql;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Mode {
    Default,
    SPARQL,
}

pub trait Validator<'a, S: SRDFBasic> {
    fn store(&self) -> &S;
    fn runner(&self) -> &dyn ValidatorRunner<S>;
    fn base(&self) -> Option<&'a str>;

    fn validate(
        &self,
        shapes: &Path,
        shapes_format: RDFFormat,
    ) -> Result<ValidationReport<S>, ValidateError> {
        let schema = load_shapes_graph(shapes, shapes_format, self.base())?;

        let mut ans: ValidationReport<S> = ValidationReport::default(); // conformant by default...
        for (_, shape) in schema.iter() {
            match shape {
                Shape::NodeShape(s) => s.validate(self.store(), self.runner(), &mut ans)?,
                Shape::PropertyShape(s) => s.validate(self.store(), self.runner(), &mut ans)?,
            };
        }
        Ok(ans)
    }
}

pub struct GraphValidator<'a> {
    store: Graph,
    runner: &'a dyn ValidatorRunner<SRDFGraph>,
    base: Option<&'a str>,
}

impl<'a> GraphValidator<'a> {
    pub fn new(
        data: &Path,
        data_format: RDFFormat,
        base: Option<&'a str>,
        mode: Mode,
    ) -> Result<Self, ValidateError> {
        Ok(GraphValidator {
            store: Graph::new(data, data_format, base)?,
            runner: match mode {
                Mode::Default => &DefaultValidatorRunner,
                Mode::SPARQL => return Err(ValidateError::UnsupportedMode),
            },
            base,
        })
    }
}

impl<'a> Validator<'a, SRDFGraph> for GraphValidator<'a> {
    fn store(&self) -> &SRDFGraph {
        self.store.store()
    }

    fn runner(&self) -> &dyn ValidatorRunner<SRDFGraph> {
        self.runner
    }

    fn base(&self) -> Option<&'a str> {
        self.base
    }
}

pub struct SparqlValidator<'a> {
    store: Sparql,
    runner: &'a dyn ValidatorRunner<SRDFSparql>,
    base: Option<&'a str>,
}

impl<'a> SparqlValidator<'a> {
    pub fn new(data: &str, mode: Mode) -> Result<Self, ValidateError> {
        Ok(SparqlValidator {
            store: Sparql::new(data)?,
            runner: match mode {
                Mode::Default => &DefaultValidatorRunner,
                Mode::SPARQL => &SparqlValidatorRunner,
            },
            base: None,
        })
    }
}

impl<'a> Validator<'a, SRDFSparql> for SparqlValidator<'a> {
    fn store(&self) -> &SRDFSparql {
        self.store.store()
    }

    fn runner(&self) -> &dyn ValidatorRunner<SRDFSparql> {
        self.runner
    }

    fn base(&self) -> Option<&'a str> {
        self.base
    }
}
