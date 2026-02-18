use crate::error::ShaclWriterError;
use iri_s::IriS;
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use shacl_ast::{ShaclSchema, ShaclVocab};
use std::io::Write;
use std::str::FromStr;

pub struct ShaclWriter<RDF: BuildRDF> {
    rdf: RDF,
    shape_count: isize,
}

impl<RDF: BuildRDF> ShaclWriter<RDF> {
    pub fn new() -> Self {
        Self {
            rdf: RDF::empty(),
            shape_count: 0,
        }
    }

    pub fn write(&mut self, schema: &ShaclSchema<RDF>) -> Result<(), ShaclWriterError> {
        let mut prefix_map = schema.prefix_map();
        prefix_map.add_prefix("rdf", IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#")?)?;
        prefix_map.add_prefix("xsd", IriS::from_str("http://www.w3.org/2001/XMLSchema#")?)?;
        prefix_map.add_prefix("sh", ShaclVocab::sh().clone())?;

        self.rdf
            .add_prefix_map(prefix_map)
            .map_err(|e| ShaclWriterError::AddPrefixMap { msg: e.to_string() })?;
        self.rdf
            .add_base(&schema.base())
            .map_err(|e| ShaclWriterError::AddBase { msg: e.to_string() })?;

        schema
            .iter()
            .try_for_each(|(_, shape)| {
                self.shape_count += 1;
                shape.write(&mut self.rdf)
            })
            .map_err(|e| ShaclWriterError::Write { msg: e.to_string() })?;

        Ok(())
    }

    pub fn shapes_count(&self) -> isize {
        self.shape_count
    }

    pub fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), RDF::Err> {
        self.rdf.serialize(format, writer)
    }
}

impl<RDF: BuildRDF> Default for ShaclWriter<RDF> {
    fn default() -> Self {
        Self::new()
    }
}
