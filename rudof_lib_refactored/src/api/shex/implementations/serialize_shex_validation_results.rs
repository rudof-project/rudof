use crate::{Rudof, Result, formats::{ShExValidationSortByMode, ResultShExValidationFormat}};
use std::io;

pub fn serialize_shex_validation_results<W: io::Write>(
    rudof: &Rudof,
    sort_order: Option<&ShExValidationSortByMode>,
    result_format: Option<&ResultShExValidationFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
