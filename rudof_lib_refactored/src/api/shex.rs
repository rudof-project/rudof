use crate::{
    ShExOperations, Result,
    formats::{InputSpec, ShExFormat, DataReaderMode, ShapeMapFormat, ShExValidationSortByMode}
};
use std::io;

impl ShExOperations for crate::Rudof {
    fn load_shex_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: &Option<ShExFormat>,
        base_schema: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_shex_schema<W: io::Write>(
        &self,
        shape_label: Option<&str>,
        show_schema: Option<bool>,
        show_statistics: Option<bool>,
        show_dependencies: Option<bool>,
        show_time: Option<bool>,
        writer: &mut W
    ) -> Result<()> {
        todo!()
    }

    fn reset_shex_schema(&mut self) {
        todo!()
    }

    fn load_shapemap(
        &mut self,
        shapemap: &InputSpec,
        shapemap_format: Option<&ShapeMapFormat>,
        base_nodes: &Option<&str>,
        base_shapes: &Option<&str>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_shapemap<W: io::Write>(
        &self,
        shapemap_format: Option<&ShapeMapFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_shapemap(&mut self) {
        todo!()
    }

    fn validate_shex(&mut self) -> Result<()> {
        todo!()
    }

    fn serialize_shex_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShExValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_shex(&mut self) {
        todo!()
    }
}
