use crate::{Schema, SH_STR};
use iri_s::IriS;
use srdf::model::mutable_rdf::MutableRdf;
use srdf::model::rdf_format::RdfFormat;
use srdf::{RDF, XSD};
use std::io::Write;
use std::str::FromStr;

pub struct ShaclWriter<RDF: MutableRdf> {
    rdf: RDF,
}

impl<RDF: MutableRdf + Default> ShaclWriter<RDF> {
    pub fn new() -> Self {
        Self {
            rdf: RDF::default(),
        }
    }

    pub fn write(&mut self, schema: &Schema<RDF>) -> Result<(), RDF::Error> {
        let mut prefix_map = schema.prefix_map();
        let _ = prefix_map.insert("rdf", &IriS::from_str(RDF).unwrap());
        let _ = prefix_map.insert("xsd", &IriS::from_str(XSD).unwrap());
        let _ = prefix_map.insert("sh", &IriS::from_str(SH_STR).unwrap());

        self.rdf.add_prefix_map(prefix_map)?;

        if let Some(base) = schema.base() {
            self.rdf.add_base(base.clone())?;
        }

        schema
            .iter()
            .try_for_each(|(_, shape)| shape.write(&mut self.rdf))?;

        Ok(())
    }

    pub fn serialize<W: Write>(&self, format: RdfFormat, writer: &mut W) -> Result<(), RDF::Error> {
        self.rdf.serialize(format, writer)
    }
}
