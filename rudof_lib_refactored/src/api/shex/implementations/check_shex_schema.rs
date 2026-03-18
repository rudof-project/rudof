use crate::{Rudof, Result, formats::{InputSpec, ShExFormat}};

pub fn check_shex_schema<W: std::io::Write>(
    rudof: &Rudof,
    schema: &InputSpec,
    schema_format: Option<&ShExFormat>,
    base_schema: Option<&str>,
    writer: &mut W
) -> Result<()> {
    todo!()
}