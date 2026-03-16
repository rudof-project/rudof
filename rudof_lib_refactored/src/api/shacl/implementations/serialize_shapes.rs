use crate::{Rudof, Result, formats::ShaclFormat};
use std::io;

pub fn serialize_shapes<W: io::Write>(
    rudof: &Rudof,
    format: Option<&ShaclFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
