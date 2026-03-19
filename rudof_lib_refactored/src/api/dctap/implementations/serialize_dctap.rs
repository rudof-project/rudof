use crate::{Result, Rudof, formats::ResultDCTapFormat};
use std::io;

/// Implementation stub for `serialize_dctap` operation.
pub fn serialize_dctap<W: io::Write>(
    rudof: &Rudof,
    result_dctap_format: Option<&ResultDCTapFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
