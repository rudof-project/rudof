use crate::validate_error::ValidateError;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use shacl_ir::compiled::schema_ir::SchemaIR;
use shacl_ir::compiled_shacl_error::CompiledShaclError;
use shacl_rdf::ShaclParser;
use std::io::BufRead;

#[cfg(feature = "sparql")]
mod graph_sparql;
#[cfg(not(feature = "sparql"))]
mod graph_native;

#[cfg(feature = "sparql")]
pub use graph_sparql::Graph;
#[cfg(not(feature = "sparql"))]
pub use graph_native::Graph;

#[cfg(feature = "sparql")]
pub mod sparql;

pub trait Store<S> {
    fn store(&self) -> &S;
}

pub struct ShaclDataManager;

impl ShaclDataManager {
    pub fn load<R: BufRead>(
        reader: &mut R,
        source_name: &str,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<SchemaIR, Box<ValidateError>> {
        let rdf = InMemoryGraph::from_reader(reader, source_name, &rdf_format, base, &ReaderMode::default())
            .map_err(|e| Box::new(ValidateError::Graph(e)))?;
        match ShaclParser::new(rdf).parse() {
            Ok(schema) => {
                let schema_compiled = schema
                    .try_into()
                    .map_err(|e: Box<CompiledShaclError>| ValidateError::CompiledShacl { error: Box::new(*e) })?;
                Ok(schema_compiled)
            },
            Err(error) => Err(Box::new(ValidateError::ShaclParser(error))),
        }
    }
}
