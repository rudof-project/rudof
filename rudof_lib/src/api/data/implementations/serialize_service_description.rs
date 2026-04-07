use crate::{Result, Rudof, errors::DataError, formats::ResultServiceFormat};
use std::io;

pub fn serialize_service_description<W: io::Write>(
    rudof: &Rudof,
    result_service_format: Option<&ResultServiceFormat>,
    writer: &mut W,
) -> Result<()> {
    let result_service_format = result_service_format.copied().unwrap_or_default();

    if let Some(service_description) = &rudof.service_description {
        service_description
            .serialize(Some(&result_service_format.into()), writer)
            .map_err(|error| {
                Box::new(DataError::FailedSerializingServiceDescription {
                    result_service_format: result_service_format.to_string(),
                    error: error.to_string(),
                })
            })?;
    } else {
        Err(Box::new(DataError::NoServiceDescription))?
    }

    Ok(())
}
