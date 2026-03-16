use crate::{Rudof, Result, formats::{InputSpec, ShExFormat, DataReaderMode}};

pub fn load_shex_schema(
    rudof: &mut Rudof,
    schema: &InputSpec,
    schema_format: Option<&ShExFormat>,
    base_schema: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    todo!()
}
