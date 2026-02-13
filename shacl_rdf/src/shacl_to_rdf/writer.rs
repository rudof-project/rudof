use iri_s::IriS;
use shacl_ast::{Schema, ShaclVocab};
use srdf::{BuildRDF, RDF, RDFFormat, XSD};
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

    pub fn write(&mut self, schema: &Schema<RDF>) -> Result<(), RDF::Err> {
        let mut prefix_map = schema.prefix_map();
        let _ = prefix_map.add_prefix("rdf", IriS::from_str(RDF).unwrap());
        let _ = prefix_map.add_prefix("xsd", IriS::from_str(XSD).unwrap());
        let _ = prefix_map.add_prefix("sh", ShaclVocab::sh().clone());

        self.rdf.add_prefix_map(prefix_map)?;
        self.rdf.add_base(&schema.base())?;

        schema.iter().try_for_each(|(_, shape)| {
            self.shape_count += 1;
            shape.write(&mut self.rdf)
        })?;

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
