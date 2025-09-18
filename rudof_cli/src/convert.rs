use crate::run_shacl_convert;
use crate::{
    CliShaclFormat, InputConvertFormat, InputConvertMode, OutputConvertFormat, OutputConvertMode,
    RDFReaderMode, add_shacl_schema_rudof, dctap_format::DCTapFormat as CliDCTapFormat,
    parse_dctap, parse_shex_schema_rudof, run_shex, show_shex_schema, writer::get_writer,
};
use anyhow::{Result, anyhow, bail};
use prefixmap::IriRef;
use rudof_lib::{InputSpec, Rudof, RudofConfig, ShExFormatter, ShapeMapParser, UmlGenerationMode};
use shapes_converter::{ShEx2Html, ShEx2Sparql, ShEx2Uml, Shacl2ShEx, Tap2ShEx};
use srdf::ImageFormat;
use srdf::UmlConverter;
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use tracing::debug;

#[allow(clippy::too_many_arguments)]
pub fn run_convert(
    input: &InputSpec,
    format: &InputConvertFormat,
    input_mode: &InputConvertMode,
    maybe_shape_str: &Option<String>,
    result_format: &OutputConvertFormat,
    output: &Option<PathBuf>,
    output_mode: &OutputConvertMode,
    target_folder: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    show_time: bool,
) -> Result<()> {
    match (input_mode, output_mode) {
        (InputConvertMode::ShEx, OutputConvertMode::ShEx) => {
            let shex_format = format.to_shex_format()?;
            let output_format = result_format.to_shex_format()?;
            // config.shex_without_showing_stats();
            run_shex(
                input,
                &shex_format,
                &output_format,
                output,
                show_time,
                true,
                false,
                force_overwrite,
                reader_mode,
                config,
            )
        }
        (InputConvertMode::SHACL, OutputConvertMode::SHACL) => {
            let shacl_format = format.to_shacl_format()?;
            let output_format = result_format.to_shacl_format()?;
            run_shacl_convert(
                input,
                &shacl_format,
                output,
                &output_format,
                force_overwrite,
                reader_mode,
                config,
            )
        }
        (InputConvertMode::DCTAP, OutputConvertMode::ShEx) => run_tap2shex(
            input,
            format,
            output,
            result_format,
            config,
            force_overwrite,
        ),
        (InputConvertMode::ShEx, OutputConvertMode::SPARQL) => {
            let maybe_shape = match maybe_shape_str {
                None => None,
                Some(shape_str) => {
                    let iri_shape = ShapeMapParser::parse_iri_ref(shape_str)?;
                    Some(iri_shape)
                }
            };
            run_shex2sparql(
                input,
                format,
                maybe_shape,
                output,
                result_format,
                config,
                force_overwrite,
                reader_mode,
            )
        }
        (InputConvertMode::ShEx, OutputConvertMode::UML) => run_shex2uml(
            input,
            format,
            output,
            result_format,
            maybe_shape_str,
            config,
            force_overwrite,
            reader_mode,
        ),
        (InputConvertMode::SHACL, OutputConvertMode::ShEx) => run_shacl2shex(
            input,
            format,
            output,
            result_format,
            config,
            force_overwrite,
            reader_mode,
        ),
        (InputConvertMode::ShEx, OutputConvertMode::HTML) => match target_folder {
            None => Err(anyhow!(
                "Conversion from ShEx to HTML requires an output parameter to indicate where to write the generated HTML files"
            )),
            Some(output_path) => run_shex2html(input, format, output_path, config, reader_mode),
        },
        (InputConvertMode::DCTAP, OutputConvertMode::UML) => run_tap2uml(
            input,
            format,
            output,
            maybe_shape_str,
            result_format,
            config,
            force_overwrite,
        ),
        (InputConvertMode::DCTAP, OutputConvertMode::HTML) => match target_folder {
            None => Err(anyhow!(
                "Conversion from DCTAP to HTML requires an output parameter to indicate where to write the generated HTML files"
            )),
            Some(output_path) => run_tap2html(input, format, output_path, config),
        },
        _ => Err(anyhow!(
            "Conversion from {input_mode} to {output_mode} is not supported yet"
        )),
    }
}

fn run_shacl2shex(
    input: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
        _ => Err(anyhow!("Can't obtain SHACL format from {format}")),
    }?;
    let mut rudof = Rudof::new(config);
    let reader_mode = (*reader_mode).into();
    add_shacl_schema_rudof(&mut rudof, input, &schema_format, &reader_mode, config)?;
    let shacl_schema = rudof.get_shacl().unwrap();
    let mut converter = Shacl2ShEx::new(&config.shacl2shex_config());

    converter.convert(shacl_schema)?;
    let (writer, color) = get_writer(output, force_overwrite)?;
    let result_schema_format = result_format.to_shex_format()?;
    show_shex_schema(
        &rudof,
        converter.current_shex(),
        &result_schema_format,
        writer,
        color,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_shex2uml(
    input: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    maybe_shape: &Option<String>,
    config: &RudofConfig,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config);
    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    let mut converter = ShEx2Uml::new(&config.shex2uml_config());
    if let Some(schema) = rudof.get_shex() {
        converter.convert(schema)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        generate_uml_output(
            converter,
            maybe_shape,
            &mut writer,
            result_format,
            config.shex2uml_config().plantuml_path(),
        )?;
    } else {
        bail!("No ShEx schema")
    }
    Ok(())
}

fn generate_uml_output<P: AsRef<Path>>(
    uml_converter: ShEx2Uml,
    maybe_shape: &Option<String>,
    writer: &mut Box<dyn Write>,
    result_format: &OutputConvertFormat,
    plantuml_path: P,
) -> Result<()> {
    let mode = if let Some(str) = maybe_shape {
        UmlGenerationMode::neighs(str)
    } else {
        UmlGenerationMode::all()
    };
    match result_format {
        OutputConvertFormat::PlantUML => {
            uml_converter.as_plantuml(writer, &mode)?;
            Ok(())
        }
        OutputConvertFormat::SVG => {
            uml_converter.as_image(writer, ImageFormat::SVG, &mode, plantuml_path)?;
            Ok(())
        }
        OutputConvertFormat::PNG => {
            uml_converter.as_image(writer, ImageFormat::PNG, &mode, plantuml_path)?;
            Ok(())
        }
        OutputConvertFormat::Default => {
            uml_converter.as_plantuml(writer, &mode)?;
            Ok(())
        }
        _ => Err(anyhow!(
            "Conversion to UML does not support output format {result_format}"
        )),
    }
}

fn run_shex2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &RudofConfig,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    debug!("Starting shex2html");
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config);

    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    if let Some(schema) = rudof.get_shex() {
        let shex2html_config = config.shex2html_config();
        let config = shex2html_config
            .clone()
            .with_target_folder(output_folder.as_ref());
        let landing_page = config.landing_page().to_string_lossy().to_string();
        debug!("Landing page will be generated at {landing_page}\nStarted converter...");
        let mut converter = ShEx2Html::new(config);
        converter.convert(schema)?;
        converter.export_schema()?;
        debug!("HTML pages generated at {}", landing_page);
    } else {
        bail!("No ShEx schema")
    }
    Ok(())
}

fn run_tap2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &RudofConfig,
) -> Result<()> {
    debug!("Starting tap2html");
    let mut rudof = Rudof::new(config);
    let dctap_format = format.to_dctap_format()?;
    parse_dctap(&mut rudof, input, &dctap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter_tap = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter_tap.convert(dctap)?;
        debug!(
            "Converted ShEx: {}",
            ShExFormatter::default().format_schema(&shex)
        );
        let shex2html_config = config
            .shex2html_config()
            .clone()
            .with_target_folder(output_folder.as_ref());
        let landing_page = shex2html_config
            .landing_page()
            .to_string_lossy()
            .to_string();
        debug!("Landing page {landing_page}\nConverter...");
        let mut converter = ShEx2Html::new(shex2html_config);
        converter.convert(&shex)?;
        // debug!("Converted HTMLSchema: {:?}", converter.current_html());
        converter.export_schema()?;
        debug!("HTML pages generated at {}", landing_page);
        Ok(())
    } else {
        bail!("Internal error: no DCTAP")
    }
}

#[allow(clippy::too_many_arguments)]
fn run_shex2sparql(
    input: &InputSpec,
    format: &InputConvertFormat,
    shape: Option<IriRef>,
    output: &Option<PathBuf>,
    _result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config);
    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    if let Some(schema) = rudof.get_shex() {
        let converter = ShEx2Sparql::new(&config.shex2sparql_config());
        let sparql = converter.convert(schema, shape)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        write!(writer, "{sparql}")?;
    }
    Ok(())
}

fn run_tap2shex(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let mut rudof = Rudof::new(config);
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(CliDCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(CliDCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    parse_dctap(&mut rudof, input_path, &tap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter.convert(dctap)?;
        let result_schema_format = result_format.to_shex_format()?;
        let (writer, color) = get_writer(output, force_overwrite)?;
        show_shex_schema(&rudof, &shex, &result_schema_format, writer, color)?;
        Ok(())
    } else {
        bail!("Internal error: No DCTAP")
    }
}

fn run_tap2uml(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    maybe_shape: &Option<String>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let mut rudof = Rudof::new(config);
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(CliDCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(CliDCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    parse_dctap(&mut rudof, input_path, &tap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter_shex = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter_shex.convert(dctap)?;
        let mut converter_uml = ShEx2Uml::new(&config.shex2uml_config());
        converter_uml.convert(&shex)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        generate_uml_output(
            converter_uml,
            maybe_shape,
            &mut writer,
            result_format,
            config.shex2uml_config().plantuml_path(),
        )?;
        Ok(())
    } else {
        bail!("Internal error: No DCTAP")
    }
}
