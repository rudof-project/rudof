use srdf::{SRDFBuilder, RDFFormat};

use crate::Schema;


struct SHACLWriter<RDF>
where RDF: SRDFBuilder {
    rdf: RDF
}

impl<RDF> SHACLWriter<RDF> where RDF: SRDFBuilder {
    pub fn new() -> SHACLWriter<RDF> {
        SHACLWriter {
            rdf: RDF::empty()
        }
    }

    pub fn write(&mut self, shacl: Schema) -> Result<(), RDF::Err> {
        self.rdf.add_prefix_map(shacl.prefix_map())?;
        self.rdf.add_base(&shacl.base())?;
        Ok(())
    }

    pub fn serialize(&self, format: RDFFormat) -> Result<(), RDF::Err> {
        todo!()
    }
}