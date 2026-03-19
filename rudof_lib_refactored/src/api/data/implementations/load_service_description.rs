use crate::{
    Result, Rudof,
    formats::{DataFormat, DataReaderMode, InputSpec},
};

pub fn load_service_description(
    rudof: &mut Rudof,
    service: &InputSpec,
    data_format: Option<&DataFormat>,
    reader_mode: Option<&DataReaderMode>,
    base: Option<&str>,
) -> Result<()> {
    todo!()
}
