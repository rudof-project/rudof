use crate::{
    Result, Rudof,
    errors::{IriError, ShExError},
    formats::{DataReaderMode, InputSpec, ShExFormat},
    utils::get_base_iri,
};
use iri_s::{IriS, MimeType};
use shex_ast::{ResolveMethod, Schema as ShExSchema, compact::ShExParser, ir::schema_ir::SchemaIR};
use shex_validation::Validator as ShExValidator;
use std::{env, io};
#[cfg(not(target_family = "wasm"))]
use url::Url;

pub fn load_shex_schema(
    rudof: &mut Rudof,
    schema: &InputSpec,
    schema_format: Option<&ShExFormat>,
    base_schema: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    let (schema_format, base_schema, reader_mode) = init_defaults(rudof, schema_format, base_schema, reader_mode)?;

    let schema_reader = schema
        .open_read(Some(schema_format.mime_type()), "ShEx Schema")
        .map_err(|error| ShExError::DataSourceSpec {
            message: format!("Failed to open schema source '{}': {error}", schema.source_name()),
        })?;

    match schema_format {
        ShExFormat::ShExC => {
            load_shex_schema_shexc(rudof, schema_reader, &schema.source_name(), base_schema, &reader_mode)?;
        },
        ShExFormat::ShExJ => {
            load_shex_schema_shexj(rudof, schema_reader, &schema.source_name(), base_schema, &reader_mode)?;
        },
        _ => {
            todo!("Implement loading for ShEx format '{}'", schema_format);
        },
    }

    Ok(())
}

fn init_defaults(
    rudof: &mut Rudof,
    schema_format: Option<&ShExFormat>,
    base_schema: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
) -> Result<(ShExFormat, IriS, DataReaderMode)> {
    let base_schema = get_base_iri(rudof, base_schema)?;
    Ok((
        schema_format.copied().unwrap_or_default(),
        base_schema,
        reader_mode.copied().unwrap_or_default(),
    ))
}

fn load_shex_schema_shexc<R: io::Read>(
    rudof: &mut Rudof,
    schema_reader: R,
    source_name: &str,
    base_schema: IriS,
    reader_mode: &DataReaderMode,
) -> Result<()> {
    #[cfg(target_family = "wasm")]
    let source_iri = {
        IriS::from_str(source_name).map_err(|error| IriError::ParseError {
            iri: source_name.to_string(),
            error: error.to_string(),
        })?
    };

    #[cfg(not(target_family = "wasm"))]
    let source_iri = {
        let cwd = env::current_dir().map_err(|e| IriError::PathConversionError {
            path: ".".to_string(),
            error: format!("Error resolving source IRI. Failed to get current directory: {e}"),
        })?;

        let url = Url::from_directory_path(&cwd).map_err(|_| IriError::PathConversionError {
            path: cwd.to_string_lossy().to_string(),
            error: "Error resolving source IRI. Cannot convert current directory to a file URL".to_string(),
        })?;

        IriS::from_str_base(source_name, Some(url.as_str())).map_err(|e| IriError::ParseError {
            iri: source_name.to_string(),
            error: format!("Failed to parse source name as IRI: {e}"),
        })?
    };

    let schema = ShExParser::from_reader(schema_reader, Some(base_schema.clone()), &source_iri).map_err(|error| {
        ShExError::FailedParsingShExSchema {
            error: error.to_string(),
            source_name: source_name.to_string(),
            format: "ShExC".to_string(),
        }
    })?;

    compile_shex_schema(rudof, base_schema, schema, reader_mode)?;

    Ok(())
}

fn load_shex_schema_shexj<R: io::Read>(
    rudof: &mut Rudof,
    schema_reader: R,
    source_name: &str,
    base_schema: IriS,
    reader_mode: &DataReaderMode,
) -> Result<()> {
    let schema = ShExSchema::from_reader(schema_reader).map_err(|error| ShExError::FailedParsingShExSchema {
        error: error.to_string(),
        source_name: source_name.to_string(),
        format: "ShExJ".to_string(),
    })?;

    compile_shex_schema(rudof, base_schema, schema, reader_mode)?;

    Ok(())
}

fn compile_shex_schema(rudof: &mut Rudof, base: IriS, schema: ShExSchema, reader_mode: &DataReaderMode) -> Result<()> {
    let mut schema_ir = SchemaIR::new();

    schema_ir
        .populate_from_schema_json(&schema, &ResolveMethod::default(), &Some(base))
        .map_err(|error| ShExError::FailedCompilingShExSchema {
            error: error.to_string(),
        })?;

    match reader_mode {
        DataReaderMode::Strict => {
            let neg_cycles = schema_ir.neg_cycles();
            let has_neg_cycles = !neg_cycles.is_empty();

            if has_neg_cycles {
                Err(ShExError::FailedCompilingShExSchema {
                    error: format!(
                        "Schema contains negative cycles in its dependency graph. Found {} negative cycle(s).",
                        neg_cycles.len()
                    ),
                })?
            }
        },
        DataReaderMode::Lax => {},
    }

    let validator = ShExValidator::new(schema_ir.clone(), &rudof.config.validator_config()).map_err(|_| {
        ShExError::FailedCompilingShExSchema {
            error: "Failed to create ShEx validator.".to_string(),
        }
    })?;

    rudof.shex_schema = Some(schema);
    rudof.shex_schema_ir = Some(schema_ir);
    rudof.shex_validator = Some(validator);
    Ok(())
}
