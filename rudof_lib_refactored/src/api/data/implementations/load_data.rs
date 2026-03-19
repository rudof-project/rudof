use crate::{Rudof, Result, formats::{InputSpec, DataFormat, DataReaderMode}};

pub fn load_data(
    rudof: &mut Rudof,
    data: &[InputSpec],
    data_format: Option<&DataFormat>,
    base: Option<&str>,
    endpoint: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    merge: Option<bool>,
) -> Result<()> {
    todo!()
}
