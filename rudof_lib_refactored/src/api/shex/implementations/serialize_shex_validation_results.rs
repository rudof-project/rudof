use crate::{Rudof, Result, formats::ShExValidationSortByMode};
use std::io;

pub fn serialize_shex_validation_results<W: io::Write>(
    rudof: &Rudof,
    sort_order: Option<&ShExValidationSortByMode>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
