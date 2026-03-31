use crate::{
    Result, Rudof,
    errors::{ComparisonError, IriError, ShExError},
    formats::{ComparisonFormat, ComparisonMode, DataReaderMode, InputSpec, ResultComparisonFormat, ShExFormat},
    utils::get_base_iri,
};
use iri_s::{IriS, MimeType};
use shapes_comparator::{CoShaMo, CoShaMoConverter};
use shex_ast::{Schema, ShExParser};
use std::{env, io};
use url::Url;

pub fn show_schema_comparison<W: io::Write>(
    rudof: &mut Rudof,
    schema1: &InputSpec,
    schema2: &InputSpec,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    format1: &ComparisonFormat,
    format2: &ComparisonFormat,
    mode1: &ComparisonMode,
    mode2: &ComparisonMode,
    shape1: Option<&str>,
    shape2: Option<&str>,
    _show_time: Option<bool>,
    result_format: Option<&ResultComparisonFormat>,
    writer: &mut W,
) -> Result<()> {
    let (base1, base2, reader_mode, result_format) = init_defaults(rudof, base1, base2, reader_mode, result_format)?;

    let schema1_reader = schema1
        .open_read(Some(format1.mime_type()), "ShEx Schema")
        .map_err(|error| ComparisonError::DataSourceSpec {
            message: format!("Failed to open schema source '{}': {error}", schema1.source_name()),
        })?;
    let coshamo1 = get_coshamo(
        rudof,
        schema1_reader,
        mode1,
        format1,
        base1,
        &reader_mode,
        shape1,
        &schema1.source_name(),
    )?;

    let schema2_reader = schema2
        .open_read(Some(format2.mime_type()), "ShEx Schema")
        .map_err(|error| ComparisonError::DataSourceSpec {
            message: format!("Failed to open schema source '{}': {error}", schema2.source_name()),
        })?;
    let coshamo2 = get_coshamo(
        rudof,
        schema2_reader,
        mode2,
        format2,
        base2,
        &reader_mode,
        shape2,
        &schema2.source_name(),
    )?;

    let shaco = coshamo1.compare(&coshamo2);

    match result_format {
        ResultComparisonFormat::Internal => {
            writeln!(writer, "{shaco}").map_err(|error| ComparisonError::ComparisonError {
                error: error.to_string(),
                format1: format1.to_string(),
                format2: format2.to_string(),
                mode1: mode1.to_string(),
                mode2: mode2.to_string(),
            })?;
        },
        ResultComparisonFormat::Json => {
            let json = serde_json::to_string_pretty(&shaco).map_err(|error| ComparisonError::ComparisonError {
                error: error.to_string(),
                format1: format1.to_string(),
                format2: format2.to_string(),
                mode1: mode1.to_string(),
                mode2: mode2.to_string(),
            })?;
            writeln!(writer, "{json}").map_err(|error| ComparisonError::ComparisonError {
                error: error.to_string(),
                format1: format1.to_string(),
                format2: format2.to_string(),
                mode1: mode1.to_string(),
                mode2: mode2.to_string(),
            })?;
        },
    }

    Ok(())
}

fn init_defaults(
    rudof: &mut Rudof,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    result_format: Option<&ResultComparisonFormat>,
) -> Result<(IriS, IriS, DataReaderMode, ResultComparisonFormat)> {
    let base1 = get_base_iri(rudof, base1)?;
    let base2 = get_base_iri(rudof, base2)?;
    Ok((
        base1,
        base2,
        reader_mode.copied().unwrap_or_default(),
        result_format.copied().unwrap_or_default(),
    ))
}

fn get_coshamo<R: io::Read>(
    rudof: &Rudof,
    reader: R,
    mode: &ComparisonMode,
    format: &ComparisonFormat,
    base: IriS,
    _reader_mode: &DataReaderMode,
    label: Option<&str>,
    source_name: &str,
) -> Result<CoShaMo> {
    match mode {
        ComparisonMode::Shacl => todo!("Not yet implemented comparison between SHACL files"),
        ComparisonMode::ShEx => {
            let shex = read_shex_only(reader, &format.clone().into(), base, source_name)?;
            let mut converter = CoShaMoConverter::new(&rudof.config.comparator_config());
            let coshamo =
                converter
                    .populate_from_shex(&shex, label)
                    .map_err(|e| ComparisonError::CoShaMoFromShExError {
                        source_name: source_name.to_string(),
                        error: e.to_string(),
                    })?;
            Ok(coshamo)
        },
        ComparisonMode::Service => todo!("Not yet implemented comparison between SPARQL endpoints"),
        ComparisonMode::Dctap => todo!("Not yet implemented comparison between DCTAP files"),
    }
}

pub fn read_shex_only<R: io::Read>(
    reader: R,
    format: &ShExFormat,
    base: IriS,
    source_name: &str,
) -> Result<Schema> {
    match format {
        ShExFormat::ShExC => {
            let cwd = env::current_dir().map_err(|e| ComparisonError::CurrentDirError { error: format!("{e}") })?;
            // Note: we use from_directory_path to convert a directory to a file URL that ends with a trailing slash
            // from_url_path would not add the trailing slash and would fail when resolving relative IRIs
            let url = Url::from_directory_path(&cwd).map_err(|_| ComparisonError::CurrentDirError { error: "Failed to convert current directory to URL".into() })?;
            let source_iri = IriS::from_str_base(source_name, Some(url.as_str())).map_err(|e| {
                IriError::ParseError { iri: source_name.to_string(), error: e.to_string() }
            })?;
            let schema_json = ShExParser::from_reader(reader, Some(base), &source_iri).map_err(|e| ShExError::FailedParsingShExSchema {
                error: e.to_string(),
                source_name: source_name.to_string(),
                format: format.to_string(),
            })?;
            Ok(schema_json)
        },
        ShExFormat::ShExJ => {
            let schema_json = Schema::from_reader(reader).map_err(|e| ShExError::FailedParsingShExSchema {
                error: e.to_string(),
                source_name: source_name.to_string(),
                format: format.to_string(),
            })?;
            Ok(schema_json)
        },
        _ => todo!("Not yet implemented reading ShEx from RDF files"),
    }
}
