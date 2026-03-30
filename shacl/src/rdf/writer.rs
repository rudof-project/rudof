use crate::ir::IRSchema;
use crate::rdf::error::ShaclWriterError;
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use std::io::Write;

pub struct ShaclWriter<RDF: BuildRDF> {
    rdf: RDF,
}

impl<RDF: BuildRDF> ShaclWriter<RDF> {
    pub fn new() -> Self {
        Self { rdf: RDF::empty(), }
    }

    pub fn register(&mut self, ir: &IRSchema) -> Result<(), ShaclWriterError> {
        self.rdf = ir.build_graph()?;
        Ok(())
    }

    pub fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), ShaclWriterError> {
        self.rdf.serialize(format, writer).map_err(|_| todo!())
    }
}

impl<RDF: BuildRDF> Default for ShaclWriter<RDF> {
    fn default() -> Self {
        Self::new()
    }
}
