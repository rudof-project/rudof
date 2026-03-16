use crate::{Rudof, Result, formats::{InputSpec, ShaclFormat, DataReaderMode}};

pub fn load_shacl_schema(
    rudof: &mut Rudof,
    schema: &InputSpec,
    schema_format: Option<&ShaclFormat>,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    todo!()
}
