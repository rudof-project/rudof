use crate::{
    ShaclOperations, Result,
    formats::{InputSpec, ShaclFormat, DataReaderMode, ShaclValidationMode, ShaclValidationSortByMode}
};
use std::io;

impl ShaclOperations for crate::Rudof {
    fn load_shacl_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: &Option<ShaclFormat>,
        base: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_shacl_schema<W: io::Write>(
        &self, 
        format: Option<&ShaclFormat>, 
        writer: &mut W
    ) -> Result<()> {
        todo!()
    }

    fn reset_shacl_schema(&mut self) {
        todo!()
    }

    fn load_shapes(
        &mut self,
        shapes: &InputSpec,
        format: Option<&ShaclFormat>,
        base: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_shapes<W: io::Write>(
        &self,
        format: Option<&ShaclFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_shapes(&mut self) {
        todo!()
    }

    fn validate_shacl(
        &mut self, 
        mode: Option<&ShaclValidationMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShaclValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_shacl_validation(&mut self) {
        todo!()
    }
}
