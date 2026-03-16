use crate::{Rudof, Result, formats::ShaclValidationSortByMode};
use std::io;

pub fn serialize_shacl_validation_results<W: io::Write>(
    rudof: &Rudof,
    sort_order: Option<&ShaclValidationSortByMode>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
