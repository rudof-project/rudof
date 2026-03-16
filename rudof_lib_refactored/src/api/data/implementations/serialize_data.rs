use crate::{Result, Rudof, formats::ResultDataFormat};
use std::io;

pub fn serialize_data<W: io::Write>(rudof: &Rudof, format: Option<&ResultDataFormat>, writer: &mut W) -> Result<()> {
    todo!()
}
