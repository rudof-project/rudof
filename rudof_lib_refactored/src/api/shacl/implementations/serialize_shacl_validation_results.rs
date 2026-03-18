use crate::{Rudof, Result, formats::{ShaclValidationSortByMode, ResultShaclValidationFormat}};
use std::io;

pub fn serialize_shacl_validation_results<W: io::Write>(
    rudof: &Rudof,
    sort_order: Option<&ShaclValidationSortByMode>,
    result_format: Option<&ResultShaclValidationFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
