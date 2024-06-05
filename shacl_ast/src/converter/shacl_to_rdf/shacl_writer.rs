use srdf::{RDFFormat, SRDFBasic, SRDFBuilder};
use std::io::Write;

use crate::{shape::Shape, Schema, SH_NODE_SHAPE, SH_PROPERTY_SHAPE};

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
        self.rdf.add_prefix_map(schema.prefix_map())?;
        self.rdf.add_base(&schema.base())?;
        for (node, shape) in schema.iter() {
            match shape {
                Shape::NodeShape(_) => self.rdf.add_type(node, node_shape::<RDF>())?,
                Shape::PropertyShape(_) => self.rdf.add_type(node, property_shape::<RDF>())?,
            }
        }
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
fn node_shape<RDF>() -> RDF::Term
where
    RDF: SRDFBasic,
{
    RDF::iri_s2term(&SH_NODE_SHAPE)
}

fn property_shape<RDF>() -> RDF::Term
where
    RDF: SRDFBasic,
{
    RDF::iri_s2term(&SH_PROPERTY_SHAPE)
}
