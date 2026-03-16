use crate::{
    Rudof, Result,
    formats::ResultRdfConfigFormat
};
use std::io;

pub fn serialize_rdf_config<W: io::Write>(
    rudof: &Rudof,
    format: Option<&ResultRdfConfigFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
