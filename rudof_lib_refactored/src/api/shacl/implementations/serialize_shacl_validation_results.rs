use crate::{Rudof, Result, formats::{ShaclValidationSortByMode, ResultShaclValidationFormat}};
use std::io;

pub fn serialize_shacl_validation_results<W: io::Write>(
    rudof: &Rudof,
    shacl_validation_sort_order_mode: Option<&ShaclValidationSortByMode>,
    result_shacl_validation_format: Option<&ResultShaclValidationFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
