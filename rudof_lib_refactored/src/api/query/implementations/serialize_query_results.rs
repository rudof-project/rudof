use crate::{Rudof, Result, formats::ResultQueryFormat};
use std::io;

pub fn serialize_query_results<W: io::Write>(
    rudof: &Rudof,
    result_query_format: Option<&ResultQueryFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
