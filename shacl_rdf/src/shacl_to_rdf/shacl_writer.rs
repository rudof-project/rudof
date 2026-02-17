// TODO - MERGE - Move to Write.rs

use iri_s::IriS;
use rdf::rdf_core::{BuildRDF, RDFFormat};
use shacl_ast::Schema;
use shacl_ast::shacl_vocab::sh;
use std::io::Write;
use std::str::FromStr;

pub struct ShaclWriter<RDF>
where
    RDF: BuildRDF,
{
    rdf: RDF,
    shapes: isize,
}

impl<RDF> ShaclWriter<RDF>
where
    RDF: BuildRDF,
{
    pub fn new() -> Self {
        Self {
            rdf: RDF::empty(),
            shapes: 0,
        }
    }

    pub fn write(&mut self, schema: &Schema<RDF>) -> Result<(), RDF::Err> {
        let mut prefix_map = schema.prefix_map();
        let _ = prefix_map.add_prefix(
            "rdf",
            IriS::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap(),
        );
        let _ = prefix_map.add_prefix("xsd", IriS::from_str("http://www.w3.org/2001/XMLSchema#").unwrap());
        let _ = prefix_map.add_prefix("sh", sh().clone());

        self.rdf.add_prefix_map(prefix_map)?;
        self.rdf.add_base(&schema.base())?;

        schema.iter().try_for_each(|(_, shape)| {
            self.shapes += 1;
            shape.write(&mut self.rdf)
        })?;

        Ok(())
    }

    pub fn shapes_count(&self) -> isize {
        self.shapes
    }

    pub fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), RDF::Err> {
        self.rdf.serialize(format, writer)
    }
}

impl<RDF> Default for ShaclWriter<RDF>
where
    RDF: BuildRDF,
{
    fn default() -> Self {
        Self::new()
    }
}
