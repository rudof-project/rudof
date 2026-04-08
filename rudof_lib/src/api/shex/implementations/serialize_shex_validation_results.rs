use crate::{
    Result, Rudof,
    errors::ShExError,
    formats::{ResultShExValidationFormat, ShExValidationSortByMode},
    utils::terminal_width,
};
use std::io;

pub fn serialize_shex_validation_results<W: io::Write>(
    rudof: &Rudof,
    sort_order: Option<&ShExValidationSortByMode>,
    result_shex_validation_format: Option<&ResultShExValidationFormat>,
    writer: &mut W,
) -> Result<()> {
    let (sort_order, result_shex_validation_format) = init_defaults(sort_order, result_shex_validation_format);
    let shex_validation_results = rudof
        .shex_validation_results
        .as_ref()
        .ok_or(ShExError::NoShexValidationResultsAvailable)?;

    match result_shex_validation_format {
        ResultShExValidationFormat::Compact => {
            writeln!(writer, "Results:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            shex_validation_results
                .as_table(writer, Some(&sort_order.into()), Some(false), Some(terminal_width()))
                .map_err(|e| ShExError::FailedSerializingShExValidationResults {
                    format: "compact".to_string(),
                    error: e.to_string(),
                })?;
        },
        ResultShExValidationFormat::Csv => {
            shex_validation_results
                .as_csv(writer, sort_order.into(), true)
                .map_err(|e| ShExError::FailedSerializingShExValidationResults {
                    format: "csv".to_string(),
                    error: e.to_string(),
                })?;
        },
        ResultShExValidationFormat::Details => {
            writeln!(writer, "Results:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            shex_validation_results
                .as_table(writer, Some(&sort_order.into()), Some(true), Some(terminal_width()))
                .map_err(|e| ShExError::FailedSerializingShExValidationResults {
                    format: "details".to_string(),
                    error: e.to_string(),
                })?;
        },
        ResultShExValidationFormat::Json => {
            writeln!(writer, "Results:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

            let str = serde_json::to_string_pretty(&shex_validation_results).map_err(|e| {
                ShExError::FailedSerializingShExValidationResults {
                    format: "json".to_string(),
                    error: e.to_string(),
                }
            })?;

            writeln!(writer, "{str}").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        },
        _ => {
            todo!("Implement serialization for the specified format: {result_shex_validation_format:?}");
        },
    }

    Ok(())
}

fn init_defaults(
    sort_order: Option<&ShExValidationSortByMode>,
    result_shex_validation_format: Option<&ResultShExValidationFormat>,
) -> (ShExValidationSortByMode, ResultShExValidationFormat) {
    (
        sort_order.copied().unwrap_or_default(),
        result_shex_validation_format.copied().unwrap_or_default(),
    )
}
