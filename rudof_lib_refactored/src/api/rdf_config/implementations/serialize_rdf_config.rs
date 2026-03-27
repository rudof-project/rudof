use crate::{Result, Rudof, errors::RdfConfigError, formats::ResultRdfConfigFormat};
use std::io;

pub fn serialize_rdf_config<W: io::Write>(
    rudof: &Rudof,
    result_rdf_config_format: Option<&ResultRdfConfigFormat>,
    writer: &mut W,
) -> Result<()> {
    let rdf_config = rudof.rdf_config.as_ref().ok_or(RdfConfigError::NoRdfConfigLoaded)?;

    let result_rdf_config_format = result_rdf_config_format.copied().unwrap_or_default();

    rdf_config
        .serialize(&result_rdf_config_format.into(), writer)
        .map_err(|e| RdfConfigError::FailedIoOperation { error: e.to_string() })?;

    Ok(())
}
