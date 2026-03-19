use crate::{
    Result, Rudof,
    formats::ResultServiceFormat,
};
use std::io;

pub fn serialize_service_description<W: io::Write>(
    rudof: &Rudof,
    result_service_format: Option<&ResultServiceFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
