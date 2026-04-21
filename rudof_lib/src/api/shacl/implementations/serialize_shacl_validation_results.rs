use crate::{
    Result, Rudof,
    errors::ShaclError,
    formats::{ResultShaclValidationFormat, ShaclValidationSortByMode},
    utils::terminal_width,
};
use rudof_rdf::{rdf_core::BuildRDF, rdf_impl::InMemoryGraph};
use shacl::validator::report::ValidationReport;
use std::io;
use shacl::types::Severity;
use crate::display::Table;

pub fn serialize_shacl_validation_results<W: io::Write>(
    rudof: &Rudof,
    shacl_validation_sort_order_mode: Option<&ShaclValidationSortByMode>,
    result_shacl_validation_format: Option<&ResultShaclValidationFormat>,
    writer: &mut W,
) -> Result<()> {
    let (_shacl_validation_sort_order_mode, result_shacl_validation_format) =
        init_defaults(shacl_validation_sort_order_mode, result_shacl_validation_format);

    let serialize_shacl_validation_results = rudof
        .shacl_validation_results
        .as_ref()
        .ok_or(ShaclError::NoShaclValidationResultsAvailable)?;

    match result_shacl_validation_format {
        ResultShaclValidationFormat::Minimal => {
            serialize_shacl_validation_results_minimal(serialize_shacl_validation_results, writer)?;
        },
        ResultShaclValidationFormat::Compact => {
            serialize_shacl_validation_results
                .table(
                    writer,
                    Some(false),
                    Some(true),
                    Some(terminal_width()),
                )
                .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
        },
        ResultShaclValidationFormat::Details => {
            serialize_shacl_validation_results
                .table(
                    writer,
                    Some(true),
                    Some(true),
                    Some(terminal_width()),
                )
                .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
        },
        ResultShaclValidationFormat::Json => {
            todo!("Generation of JSON for SHACL validation report is not implemented yet")
        },
        _ => {
            serialize_shacl_validation_results_rdf(
                serialize_shacl_validation_results,
                result_shacl_validation_format,
                writer,
            )?;
        },
    }

    Ok(())
}

fn init_defaults(
    shacl_validation_sort_order_mode: Option<&ShaclValidationSortByMode>,
    result_shacl_validation_format: Option<&ResultShaclValidationFormat>,
) -> (ShaclValidationSortByMode, ResultShaclValidationFormat) {
    (
        shacl_validation_sort_order_mode.copied().unwrap_or_default(),
        result_shacl_validation_format.copied().unwrap_or_default(),
    )
}

fn serialize_shacl_validation_results_minimal<W: io::Write>(
    shacl_validation_results: &ValidationReport,
    writer: &mut W,
) -> Result<()> {
    if shacl_validation_results.conforms() {
        writeln!(writer, "Conforms").map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
    } else {
        writeln!(
            writer,
            "Does not conform, {} violations, {} warnings",
            shacl_validation_results.get_count_of(&Severity::Violation),
            shacl_validation_results.get_count_of(&Severity::Warning)
        )
        .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;
    }

    Ok(())
}

fn serialize_shacl_validation_results_rdf<W: io::Write>(
    shacl_validation_results: &ValidationReport,
    result_shacl_validation_format: ResultShaclValidationFormat,
    writer: &mut W,
) -> Result<()> {
    let mut rdf_writer = InMemoryGraph::new();

    shacl_validation_results
        .to_rdf(&mut rdf_writer)
        .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;

    let rdf_format = result_shacl_validation_format.try_into()?;

    rdf_writer
        .serialize(&rdf_format, writer)
        .map_err(|e| ShaclError::FailedIoOperation { error: e.to_string() })?;

    Ok(())
}
