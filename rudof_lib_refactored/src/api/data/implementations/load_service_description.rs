use crate::{
    Result, Rudof,
    formats::{DataFormat, DataReaderMode, InputSpec},
};

pub fn load_service_description(
    rudof: &mut Rudof,
    service: &InputSpec,
    format: Option<&DataFormat>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    todo!()
}
