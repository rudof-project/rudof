use crate::{Rudof, Result, formats::ShapeMapFormat};
use std::io;

pub fn serialize_shapemap<W: io::Write>(
    rudof: &Rudof,
    shapemap_format: Option<&ShapeMapFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
