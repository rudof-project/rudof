use crate::{Schema, SH_STR};
use iri_s::IriS;
use srdf::{RDFFormat, SRDFBuilder, RDF, XSD};
use std::io::Write;
use std::str::FromStr;

pub struct ShaclWriter<RDF>
where
    RDF: SRDFBuilder,
{
    rdf: RDF,
}

impl<RDF> ShaclWriter<RDF>
where
    RDF: SRDFBuilder,
{
    pub fn new() -> Self {
        Self { rdf: RDF::empty() }
    }

    pub fn write(&mut self, schema: &Schema) -> Result<(), RDF::Err> {
        let mut prefix_map = schema.prefix_map();
        prefix_map.insert("rdf", &IriS::from_str(RDF).unwrap());
        prefix_map.insert("xsd", &IriS::from_str(XSD).unwrap());
        prefix_map.insert("sh", &IriS::from_str(SH_STR).unwrap());

        self.rdf.add_prefix_map(prefix_map)?;
        self.rdf.add_base(&schema.base())?;

        schema
            .iter()
            .try_for_each(|(_, shape)| shape.write(&mut self.rdf))?;

        Ok(())
    }

    pub fn serialize<W: Write>(&self, format: RDFFormat, writer: W) -> Result<(), RDF::Err> {
        self.rdf.serialize(format, writer)
    }
}

impl<RDF> Default for ShaclWriter<RDF>
where
    RDF: SRDFBuilder,
{
    fn default() -> Self {
        Self::new()
    }
}
