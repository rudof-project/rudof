use crate::{
    Result, Rudof,
    api::{
        dctap::implementations::load_dctap,
        shacl::implementations::{load_shacl_schema, serialize_shacl_schema},
        shex::implementations::{load_shex_schema, serialize_shex_schema},
    },
    errors::ConversionError,
    formats::{
        ConversionFormat, ConversionMode, DataReaderMode, InputSpec, ResultConversionFormat, ResultConversionMode,
        ShExFormat,
    },
};
use rudof_rdf::rdf_core::visualizer::uml_converter::{ImageFormat, UmlConverter, UmlGenerationMode};
use shapes_converter::{ShEx2Html, ShEx2Sparql, ShEx2Uml, Shacl2ShEx, Tap2ShEx};
use shex_ast::ShapeMapParser;
use std::{
    io,
    path::{Path, PathBuf},
};

#[allow(clippy::too_many_arguments)]
pub fn show_schema_conversion<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_mode: &ConversionMode,
    output_mode: &ResultConversionMode,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    show_time: Option<bool>,
    templates_folder: Option<&Path>,
    output_folder: Option<&Path>,
    writer: &mut W,
) -> Result<()> {
    match (input_mode, output_mode) {
        (ConversionMode::ShEx, ResultConversionMode::ShEx) => show_schema_conversion_shex_to_shex(
            rudof,
            schema,
            base,
            reader_mode,
            input_format,
            output_format,
            shape,
            show_time,
            writer,
        ),
        (ConversionMode::ShEx, ResultConversionMode::Sparql) => show_schema_conversion_shex_to_sparql(
            rudof,
            schema,
            base,
            reader_mode,
            input_format,
            output_format,
            shape,
            writer,
        ),
        (ConversionMode::ShEx, ResultConversionMode::Uml) => show_schema_conversion_shex_to_uml(
            rudof,
            schema,
            base,
            reader_mode,
            input_format,
            output_format,
            shape,
            writer,
        ),
        (ConversionMode::ShEx, ResultConversionMode::Html) => match output_folder {
            Some(of) => show_schema_conversion_shex_to_html(
                rudof,
                schema,
                base,
                reader_mode,
                input_format,
                output_format,
                templates_folder,
                of,
            ),
            None => Err(ConversionError::FailedConversion {
                input_mode: "shex".to_string(),
                output_mode: "html".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: "Output folder must be specified for HTML conversion".to_string(),
            })?,
        },
        (ConversionMode::Shacl, ResultConversionMode::Shacl) => {
            show_schema_conversion_shacl_to_shacl(rudof, schema, base, reader_mode, input_format, output_format, writer)
        },
        (ConversionMode::Shacl, ResultConversionMode::ShEx) => show_schema_conversion_shacl_to_shex(
            rudof,
            schema,
            base,
            reader_mode,
            input_format,
            output_format,
            shape,
            show_time,
            writer,
        ),
        (ConversionMode::Dctap, ResultConversionMode::ShEx) => show_schema_conversion_dctap_to_shex(
            rudof,
            schema,
            base,
            reader_mode,
            input_format,
            output_format,
            shape,
            show_time,
            writer,
        ),
        (ConversionMode::Dctap, ResultConversionMode::Uml) => {
            show_schema_conversion_dctap_to_uml(rudof, schema, input_format, output_format, shape, writer)
        },
        (ConversionMode::Dctap, ResultConversionMode::Html) => match output_folder {
            Some(of) => {
                show_schema_conversion_dctap_to_html(rudof, schema, input_format, output_format, templates_folder, of)
            },
            None => Err(ConversionError::FailedConversion {
                input_mode: "dctap".to_string(),
                output_mode: "html".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: "Output folder must be specified for HTML conversion".to_string(),
            })?,
        },
        _ => Err(ConversionError::UnsupportedConversion {
            input_mode: input_mode.to_string(),
            output_mode: output_mode.to_string(),
        })?,
    }
}

fn show_schema_conversion_shex_to_shex<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    show_time: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    load_shex_schema(rudof, schema, Some(&(*input_format).try_into()?), base, reader_mode)?;

    serialize_shex_schema(
        rudof,
        shape,
        None,
        None,
        None,
        show_time,
        None,
        Some(&(*output_format).try_into()?),
        writer,
    )?;

    Ok(())
}

fn show_schema_conversion_shex_to_sparql<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    writer: &mut W,
) -> Result<()> {
    let shape = match &shape {
        None => None,
        Some(shape_str) => {
            let iri_shape =
                ShapeMapParser::parse_iri_ref(shape_str).map_err(|error| ConversionError::FailedConversion {
                    input_mode: "shex".to_string(),
                    output_mode: "sparql".to_string(),
                    input_format: input_format.to_string(),
                    output_format: output_format.to_string(),
                    error: error.to_string(),
                })?;
            Some(iri_shape)
        },
    };

    load_shex_schema(rudof, schema, Some(&(*input_format).try_into()?), base, reader_mode)?;

    let converter = ShEx2Sparql::new(&rudof.config.shex2sparql_config());
    let sparql = converter
        .convert(rudof.shex_schema.as_ref().unwrap(), shape)
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "sparql".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    write!(writer, "{sparql}").map_err(|error| ConversionError::FailedConversion {
        input_mode: "shex".to_string(),
        output_mode: "sparql".to_string(),
        input_format: input_format.to_string(),
        output_format: output_format.to_string(),
        error: error.to_string(),
    })?;

    Ok(())
}

fn show_schema_conversion_shex_to_uml<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    writer: &mut W,
) -> Result<()> {
    load_shex_schema(rudof, schema, Some(&(*input_format).try_into()?), base, reader_mode)?;

    let mut converter = ShEx2Uml::new(&rudof.config.shex2uml_config());
    converter
        .convert(rudof.shex_schema.as_ref().unwrap())
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "uml".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    generate_uml_output(
        converter,
        shape,
        writer,
        input_format,
        output_format,
        rudof.config.shex2uml_config().plantuml_path(),
    )
}

fn generate_uml_output<P: AsRef<Path>, W: io::Write>(
    uml_converter: ShEx2Uml,
    maybe_shape: Option<&str>,
    writer: &mut W,
    input_format: &ConversionFormat,
    result_format: &ResultConversionFormat,
    plantuml_path: P,
) -> Result<()> {
    let mode = if let Some(str) = maybe_shape {
        UmlGenerationMode::neighs(str)
    } else {
        UmlGenerationMode::all()
    };
    match result_format {
        ResultConversionFormat::PlantUML | ResultConversionFormat::Default => {
            uml_converter
                .as_plantuml(writer, &mode)
                .map_err(|error| ConversionError::FailedConversion {
                    input_mode: "shex".to_string(),
                    output_mode: "uml".to_string(),
                    input_format: input_format.to_string(),
                    output_format: result_format.to_string(),
                    error: error.to_string(),
                })?;
            Ok(())
        },
        ResultConversionFormat::Svg => {
            uml_converter
                .as_image(writer, ImageFormat::SVG, &mode, plantuml_path)
                .map_err(|error| ConversionError::FailedConversion {
                    input_mode: "shex".to_string(),
                    output_mode: "uml".to_string(),
                    input_format: input_format.to_string(),
                    output_format: result_format.to_string(),
                    error: error.to_string(),
                })?;
            Ok(())
        },
        ResultConversionFormat::Png => {
            uml_converter
                .as_image(writer, ImageFormat::PNG, &mode, plantuml_path)
                .map_err(|error| ConversionError::FailedConversion {
                    input_mode: "shex".to_string(),
                    output_mode: "uml".to_string(),
                    input_format: input_format.to_string(),
                    output_format: result_format.to_string(),
                    error: error.to_string(),
                })?;
            Ok(())
        },
        _ => Err(ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "uml".to_string(),
            input_format: input_format.to_string(),
            output_format: result_format.to_string(),
            error: "Unsupported UML output format".to_string(),
        })?,
    }
}

fn show_schema_conversion_shex_to_html<P: AsRef<Path>>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    templates_folder: Option<&Path>,
    output_folder: P,
) -> Result<()> {
    load_shex_schema(rudof, schema, Some(&(*input_format).try_into()?), base, reader_mode)?;

    let mut shex2html_config = rudof.config.shex2html_config();
    shex2html_config = shex2html_config.with_target_folder(output_folder);

    let mut converter = ShEx2Html::new(shex2html_config.clone());
    converter
        .convert(rudof.shex_schema.as_ref().unwrap())
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "html".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    let resolved_template = match templates_folder {
        Some(tf) => tf.to_path_buf(),
        None => shex2html_config
            .template_folder
            .map(PathBuf::from)
            .ok_or(ConversionError::FailedConversion {
                input_mode: "shex".to_string(),
                output_mode: "html".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: "Templates folder must be specified for HTML conversion if not configured in RudofConfig"
                    .to_string(),
            })?,
    };
    converter
        .export_schema(resolved_template)
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "html".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    Ok(())
}

fn show_schema_conversion_shacl_to_shacl<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    writer: &mut W,
) -> Result<()> {
    load_shacl_schema(
        rudof,
        Some(schema),
        Some(&(*input_format).try_into()?),
        base,
        reader_mode,
    )?;

    serialize_shacl_schema(rudof, Some(&(*output_format).try_into()?), writer)?;

    Ok(())
}

fn show_schema_conversion_shacl_to_shex<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    show_time: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    load_shacl_schema(
        rudof,
        Some(schema),
        Some(&(*input_format).try_into()?),
        base,
        reader_mode,
    )?;

    let mut converter = Shacl2ShEx::new(&rudof.config.shacl2shex_config());
    converter
        .convert(rudof.shacl_shapes.as_ref().unwrap())
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shacl".to_string(),
            output_mode: "shex".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    let shex_schema = InputSpec::Str(converter.current_shex().to_string());
    println!("Generated ShEx schema:\n{}", converter.current_shex());
    load_shex_schema(
        rudof,
        &shex_schema,
        Some(&crate::formats::ShExFormat::ShExJ),
        base,
        reader_mode,
    )?;

    serialize_shex_schema(
        rudof,
        shape,
        None,
        None,
        None,
        show_time,
        None,
        Some(&(*output_format).try_into()?),
        writer,
    )?;

    Ok(())
}

fn show_schema_conversion_dctap_to_shex<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    base: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    show_time: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    load_dctap(rudof, schema, Some(&(*input_format).try_into()?))?;

    let converter = Tap2ShEx::new(&rudof.config.tap2shex_config());
    let shex_schema =
        converter
            .convert(rudof.dctap.as_ref().unwrap())
            .map_err(|error| ConversionError::FailedConversion {
                input_mode: "dctap".to_string(),
                output_mode: "shex".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: error.to_string(),
            })?;

    let shex_schema = InputSpec::Str(shex_schema.to_string());
    load_shex_schema(rudof, &shex_schema, Some(&ShExFormat::ShExJ), base, reader_mode)?;

    serialize_shex_schema(
        rudof,
        shape,
        None,
        None,
        None,
        show_time,
        Some(false),
        Some(&(*output_format).try_into()?),
        writer,
    )?;

    Ok(())
}

fn show_schema_conversion_dctap_to_uml<W: io::Write>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    shape: Option<&str>,
    writer: &mut W,
) -> Result<()> {
    load_dctap(rudof, schema, Some(&(*input_format).try_into()?))?;

    let converter = Tap2ShEx::new(&rudof.config.tap2shex_config());
    let shex_schema =
        converter
            .convert(rudof.dctap.as_ref().unwrap())
            .map_err(|error| ConversionError::FailedConversion {
                input_mode: "dctap".to_string(),
                output_mode: "shex".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: error.to_string(),
            })?;

    let mut converter = ShEx2Uml::new(&rudof.config.shex2uml_config());
    converter
        .convert(&shex_schema)
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "dctap".to_string(),
            output_mode: "uml".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    generate_uml_output(
        converter,
        shape,
        writer,
        input_format,
        output_format,
        rudof.config.shex2uml_config().plantuml_path(),
    )
}

fn show_schema_conversion_dctap_to_html<P: AsRef<std::path::Path>>(
    rudof: &mut Rudof,
    schema: &InputSpec,
    input_format: &ConversionFormat,
    output_format: &ResultConversionFormat,
    templates_folder: Option<&Path>,
    output_folder: P,
) -> Result<()> {
    load_dctap(rudof, schema, Some(&(*input_format).try_into()?))?;

    let converter = Tap2ShEx::new(&rudof.config.tap2shex_config());
    let shex_schema =
        converter
            .convert(rudof.dctap.as_ref().unwrap())
            .map_err(|error| ConversionError::FailedConversion {
                input_mode: "dctap".to_string(),
                output_mode: "shex".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: error.to_string(),
            })?;

    let mut shex2html_config = rudof.config.shex2html_config();
    shex2html_config = shex2html_config.with_target_folder(output_folder);

    let mut converter = ShEx2Html::new(shex2html_config.clone());
    converter
        .convert(&shex_schema)
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "html".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    let resolved_template = match templates_folder {
        Some(tf) => tf.to_path_buf(),
        None => shex2html_config
            .template_folder
            .map(PathBuf::from)
            .ok_or(ConversionError::FailedConversion {
                input_mode: "shex".to_string(),
                output_mode: "html".to_string(),
                input_format: input_format.to_string(),
                output_format: output_format.to_string(),
                error: "Templates folder must be specified for HTML conversion if not configured in RudofConfig"
                    .to_string(),
            })?,
    };
    converter
        .export_schema(resolved_template)
        .map_err(|error| ConversionError::FailedConversion {
            input_mode: "shex".to_string(),
            output_mode: "html".to_string(),
            input_format: input_format.to_string(),
            output_format: output_format.to_string(),
            error: error.to_string(),
        })?;

    Ok(())
}
