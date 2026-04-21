use crate::{Result, Rudof, errors::DCTapError, formats::ResultDCTapFormat};
use std::io;

pub fn serialize_dctap<W: io::Write>(
    rudof: &Rudof,
    result_dctap_format: Option<&ResultDCTapFormat>,
    writer: &mut W,
) -> Result<()> {
    let dctap = rudof.dctap.as_ref().ok_or(DCTapError::NoDCTapLoaded)?;
    let result_dctap_format = result_dctap_format.copied().unwrap_or_default();

    match result_dctap_format {
        ResultDCTapFormat::Internal => {
            write!(writer, "{dctap}").map_err(|e| DCTapError::FailedSerializingDCTap {
                format: result_dctap_format.to_string(),
                error: e.to_string(),
            })?;
        },
        ResultDCTapFormat::Json => {
            let str = serde_json::to_string_pretty(&dctap).map_err(|e| DCTapError::FailedSerializingDCTap {
                format: result_dctap_format.to_string(),
                error: e.to_string(),
            })?;
            write!(writer, "{str}").map_err(|e| DCTapError::FailedSerializingDCTap {
                format: result_dctap_format.to_string(),
                error: e.to_string(),
            })?;
        },
    }

    Ok(())
}
