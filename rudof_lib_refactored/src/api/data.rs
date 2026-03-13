use crate::{
    DataOperations, Result,
    formats::{InputSpec, DataFormat, DataReaderMode, NodeInspectionMode}
};
use std::io;

impl DataOperations for crate::Rudof {
    fn load_data(
        &mut self,
        data: &[InputSpec],
        data_format: Option<&DataFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_data<W: io::Write>(
        &self, 
        format: Option<&DataFormat>, 
        writer: &mut W
    ) -> Result<()> {
        todo!()
    }

    fn reset_data(&mut self) {
        todo!()
    }

    fn load_service_description(
        &mut self,
        service: &InputSpec,
        format: Option<&DataFormat>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_service_description<W: io::Write>(
        &self,
        format: Option<&crate::formats::ResultServiceFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_service_description(&mut self) {
        todo!()
    }

    fn show_node_info<W: io::Write>(
        &self,
        node: &str,
        predicates: Option<&[&str]>,
        show_node_mode: Option<&NodeInspectionMode>,
        depth: Option<usize>,
        show_hyperlinks: Option<bool>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }
}
