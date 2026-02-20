use std::io::Write;

use iri_s::{IriS, MimeType};
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use shacl_ast::ShaclFormat;
use shacl_validation::validation_report::report::ValidationReport;

use crate::{
    InputSpec, Rudof, RudofConfig, RudofError,
    data::get_base,
    result_shacl_validation_format::{
        ResultShaclValidationFormat, SortByShaclValidationReport, cnv_sort_mode_report, result_format_to_rdf_format,
    },
    shacl_format::ShaclFormat as libShaclFormat,
    terminal_width::terminal_width,
};

pub fn add_shacl_schema_rudof(
    rudof: &mut Rudof,
    schema: &InputSpec,
    shapes_format: &libShaclFormat,
    base_shapes: &Option<IriS>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<(), RudofError> {
    let mime_type = shapes_format.mime_type();
    let mut reader = schema
        .open_read(Some(mime_type), "SHACL shapes")
        .map_err(|e| RudofError::ReadingPathContext {
            path: schema.source_name().to_string(),
            error: e.to_string(),
            context: "SHACL Schema".to_string(),
        })?;
    let reader_name = schema.to_string();
    let shapes_format = shacl_format_convert(*shapes_format)?;
    let base = get_base(schema, config, base_shapes)?;
    rudof.read_shacl(&mut reader, &reader_name, &shapes_format, base.as_deref(), reader_mode)?;
    Ok(())
}

pub fn shacl_format_convert(shacl_format: libShaclFormat) -> Result<ShaclFormat, RudofError> {
    match shacl_format {
        libShaclFormat::Turtle => Ok(ShaclFormat::Turtle),
        libShaclFormat::RdfXml => Ok(ShaclFormat::RdfXml),
        libShaclFormat::NTriples => Ok(ShaclFormat::NTriples),
        libShaclFormat::TriG => Ok(ShaclFormat::TriG),
        libShaclFormat::N3 => Ok(ShaclFormat::N3),
        libShaclFormat::NQuads => Ok(ShaclFormat::NQuads),
        libShaclFormat::Internal => Ok(ShaclFormat::Internal),
        libShaclFormat::JsonLd => Ok(ShaclFormat::JsonLd),
    }
}

pub fn write_validation_report<W: Write>(
    mut writer: W,
    format: &ResultShaclValidationFormat,
    report: ValidationReport,
    sort_by: &SortByShaclValidationReport,
) -> Result<(), RudofError> {
    let terminal_width = terminal_width();
    let sort_mode = cnv_sort_mode_report(sort_by);
    match format {
        ResultShaclValidationFormat::Minimal => {
            if report.conforms() {
                writeln!(writer, "Conforms")?;
            } else {
                writeln!(
                    writer,
                    "Does not conform, {} violations, {} warnings",
                    report.count_violations(),
                    report.count_warnings()
                )?;
            }
            Ok(())
        },
        ResultShaclValidationFormat::Compact => {
            report.show_as_table(writer, sort_mode, false, terminal_width)?;
            Ok(())
        },
        ResultShaclValidationFormat::Details => {
            report.show_as_table(writer, sort_mode, true, terminal_width)?;
            Ok(())
        },
        ResultShaclValidationFormat::Json => Err(RudofError::NotImplemented {
            msg: "Generation of JSON for SHACL validation report is not implemented yet".to_string(),
        }),
        _ => {
            let mut rdf_writer = InMemoryGraph::new();
            report.to_rdf(&mut rdf_writer).map_err(|e| RudofError::Generic {
                error: format!("Error converting SHACL validation report to RDF: {e}"),
            })?;
            let rdf_format = result_format_to_rdf_format(format)?;
            rdf_writer
                .serialize(&rdf_format, &mut writer)
                .map_err(|e| RudofError::RdfError { error: e.to_string() })?;
            Ok(())
        },
    }
}
