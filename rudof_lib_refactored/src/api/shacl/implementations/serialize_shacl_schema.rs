use crate::{Rudof, Result, formats::ShaclFormat};
use std::io;

pub fn serialize_shacl_schema<W: io::Write>(
    rudof: &Rudof,
    shacl_format: Option<&ShaclFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
