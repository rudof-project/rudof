use crate::{
    RdfConfigOperations, Result,
    formats::{InputSpec, RdfConfigFormat, ResultRdfConfigFormat}
};
use std::io;

impl RdfConfigOperations for crate::Rudof {
    fn load_rdf_config(
        &mut self,
        rdf_config: &InputSpec,
        format: Option<&RdfConfigFormat>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_rdf_config<W: io::Write>(
        &self,
        format: Option<&ResultRdfConfigFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_rdf_config(&mut self) {
        todo!()
    }
}
