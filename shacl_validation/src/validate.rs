use std::path::Path;

use clap::ValueEnum;
use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::{RDFFormat, SRDFBasic, SRDFGraph, SRDFSparql};

use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::helper::srdf::load_shapes_graph;
use crate::shape::Validate;
use crate::store::graph::Graph;
use crate::store::sparql::Sparql;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Mode {
    Default,
    SPARQL,
}

pub trait Validator<'a, S: SRDFBasic> {
    fn base(&self) -> Option<&'a str>;
    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<S> + '_>;

    fn validate(
        &self,
        shapes: &Path,
        shapes_format: RDFFormat,
    ) -> Result<ValidationReport<S>, ValidateError> {
        let schema = load_shapes_graph(shapes, shapes_format, self.base())?;
        let mut ans: ValidationReport<S> = ValidationReport::default(); // conformant by default...
        for (_, shape) in schema.iter() {
            match shape {
                Shape::NodeShape(s) => s.validate(self.executor(&schema).as_ref(), &mut ans)?,
                Shape::PropertyShape(s) => s.validate(self.executor(&schema).as_ref(), &mut ans)?,
            };
        }
        Ok(ans)
    }
}

pub struct GraphValidator<'a> {
    store: Graph,
    mode: Mode,
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
            mode,
            base,
        })
    }
}

impl<'a> Validator<'a, SRDFGraph> for GraphValidator<'a> {
    fn base(&self) -> Option<&'a str> {
        self.base
    }

    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<SRDFGraph> + '_> {
        match self.mode {
            Mode::Default => Box::new(DefaultExecutor::new(self.store.store(), schema.to_owned())),
            Mode::SPARQL => todo!(),
        }
    }
}

pub struct SparqlValidator<'a> {
    store: Sparql,
    mode: Mode,
    base: Option<&'a str>,
}

impl<'a> SparqlValidator<'a> {
    pub fn new(data: &str, mode: Mode) -> Result<Self, ValidateError> {
        Ok(SparqlValidator {
            store: Sparql::new(data)?,
            mode,
            base: None,
        })
    }
}

impl<'a> Validator<'a, SRDFSparql> for SparqlValidator<'a> {
    fn base(&self) -> Option<&'a str> {
        self.base
    }

    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<SRDFSparql> + '_> {
        match self.mode {
            Mode::Default => Box::new(DefaultExecutor::new(self.store.store(), schema.to_owned())),
            Mode::SPARQL => Box::new(QueryExecutor::new(self.store.store(), schema.to_owned())),
        }
    }
}
