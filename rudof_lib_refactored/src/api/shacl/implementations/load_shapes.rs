use crate::{Rudof, Result, formats::{InputSpec, ShaclFormat, DataReaderMode}};

pub fn load_shapes(
    rudof: &mut Rudof,
    shapes: &InputSpec,
    format: Option<&ShaclFormat>,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    todo!()
}
