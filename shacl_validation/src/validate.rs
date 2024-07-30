use shacl_ast::shape::Shape;
use srdf::{RDFFormat, SRDFGraph, SRDFSparql};

use crate::runner::oxigraph::OxigraphRunner;
use crate::runner::srdf::SRDFRunner;
use crate::runner::ValidatorRunner;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub enum Backend {
    InMemory,
    SPARQL,
    InDisk,
}

pub struct Validator;

impl Validator {
    pub fn validate<'a>(
        data: &str,
        data_format: RDFFormat,
        shapes: &str,
        shapes_format: RDFFormat,
        backend: Backend,
        base: Option<&str>,
    ) -> Result<ValidationReport<'a>, ValidateError> {
        let runner: Box<dyn ValidatorRunner> = match backend {
            Backend::InMemory => Box::new(SRDFRunner::<SRDFGraph>::new(data, data_format, base)?),
            Backend::SPARQL => Box::new(SRDFRunner::<SRDFSparql>::new(data, data_format, base)?),
            Backend::InDisk => Box::new(OxigraphRunner::new(data, data_format, base)?),
        };

        let schema = runner.load_shapes_graph(shapes, shapes_format, base)?;

        let mut ans = ValidationReport::default(); // conformant by default...
        for (_, shape) in schema.iter() {
            match shape {
                Shape::NodeShape(shape) => shape.validate(&runner, &mut ans),
                Shape::PropertyShape(shape) => shape.validate(&runner, &mut ans),
            };
        }
        Ok(ans)
    }
}
