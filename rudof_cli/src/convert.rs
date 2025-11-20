use crate::run_shacl_convert;
use crate::{
    InputConvertFormat, InputConvertMode, OutputConvertFormat, OutputConvertMode,
    dctap_format::DCTapFormat as CliDCTapFormat, parse_dctap, run_shex,
    writer::get_writer,
};
use anyhow::{Result, anyhow, bail};
use iri_s::IriS;
use prefixmap::IriRef;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, ShExFormatter, ShapeMapParser, UmlGenerationMode,
    shacl::add_shacl_schema_rudof, shacl_format::CliShaclFormat,
};
use shapes_converter::{ShEx2Html, ShEx2Sparql, ShEx2Uml, Shacl2ShEx, Tap2ShEx};
use srdf::UmlConverter;
use srdf::{ImageFormat, ReaderMode};
use std::{
    io::Write,
    path::{Path, PathBuf},
};
use tracing::trace;

#[allow(clippy::too_many_arguments)]
pub fn run_convert(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    input_mode: &InputConvertMode,
    maybe_shape_str: &Option<String>,
    result_format: &OutputConvertFormat,
    output: &Option<PathBuf>,
    output_mode: &OutputConvertMode,
    target_folder: &Option<PathBuf>,
    template_folder: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    show_time: bool,
) -> Result<()> {
    match (input_mode, output_mode) {
        (InputConvertMode::ShEx, OutputConvertMode::ShEx) => {
            let shex_format = format.to_shex_format()?;
            let output_format = result_format.to_shex_format()?;
            let shape = None;
            // config.shex_without_showing_stats();
            run_shex(
                input,
                &shex_format,
                &shape,
                base,
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
                base,
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
                base,
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
            base,
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
            base,
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
            Some(output_path) => run_shex2html(
                input,
                format,
                base,
                output_path,
                template_folder,
                config,
                reader_mode,
            ),
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
                "Conversion from DCTAP to HTML requires parameter `target-folder` to indicate where to write the generated HTML files"
            )),
            Some(output_path) => run_tap2html(input, format, output_path, template_folder, config),
        },
        _ => Err(anyhow!(
            "Conversion from {input_mode} to {output_mode} is not supported yet"
        )),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_shacl2shex(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
        _ => Err(anyhow!("Can't obtain SHACL format from {format}")),
    }?;
    let mut rudof = Rudof::new(config)?;
    add_shacl_schema_rudof(&mut rudof, input, &schema_format, base, reader_mode, config)?;
    let shacl_schema = rudof.get_shacl().unwrap();
    let mut converter = Shacl2ShEx::new(&config.shacl2shex_config());

    converter.convert(shacl_schema)?;
    let (mut writer, color) = get_writer(output, force_overwrite)?;
    let result_schema_format = result_format.to_shex_format()?;
    let formatter = match color {
        crate::ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        crate::ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof_lib::shex::serialize_shex_rudof(
        &rudof,
        converter.current_shex(),
        &result_schema_format,
        &formatter,
        &mut writer,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_shex2uml(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    maybe_shape: &Option<String>,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
) -> Result<()> {
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config)?;
    rudof_lib::shex::parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config)
        .map_err(|e| anyhow!("{}", e))?;
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
    base: &Option<IriS>,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    template_folder: &Option<PathBuf>,
    config: &RudofConfig,
    reader_mode: &ReaderMode,
) -> Result<()> {
    trace!("Starting shex2html");
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config)?;

    rudof_lib::shex::parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config)
        .map_err(|e| anyhow!("{}", e))?;
    if let Some(schema) = rudof.get_shex() {
        let shex2html_config = config.shex2html_config();
        let config = shex2html_config
            .clone()
            .with_target_folder(output_folder.as_ref());
        let landing_page = config.landing_page().to_string_lossy().to_string();
        trace!("Landing page will be generated at {landing_page}\nStarted converter...");
        let mut converter = ShEx2Html::new(config);
        converter.convert(schema)?;
        let template_folder = match template_folder {
            None => match shex2html_config.template_folder {
                None => bail!("No template folder specified neither in config nor in command line"),
                Some(tf) => PathBuf::from(tf),
            },
            Some(tf) => tf.to_path_buf(),
        };
        converter.export_schema(template_folder)?;
        trace!("HTML pages generated at {}", landing_page);
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
    template_folder: &Option<PathBuf>,
    config: &RudofConfig,
) -> Result<()> {
    trace!("Starting tap2html");
    let mut rudof = Rudof::new(config)?;
    let dctap_format = format.to_dctap_format()?;
    parse_dctap(&mut rudof, input, &dctap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter_tap = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter_tap.convert(dctap)?;
        trace!(
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
        trace!("Landing page {landing_page}\nConverter...");
        let mut converter = ShEx2Html::new(shex2html_config);
        converter.convert(&shex)?;
        // debug!("Converted HTMLSchema: {:?}", converter.current_html());
        let template_folder = match template_folder {
            None => match config.shex2html_config().template_folder {
                None => bail!("No template folder specified neither in config nor in command line"),
                Some(tf) => PathBuf::from(tf),
            },
            Some(tf) => tf.to_path_buf(),
        };
        converter.export_schema(template_folder)?;
        trace!("HTML pages generated at {}", landing_page);
        Ok(())
    } else {
        bail!("Internal error: no DCTAP")
    }
}

#[allow(clippy::too_many_arguments)]
fn run_shex2sparql(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    shape: Option<IriRef>,
    output: &Option<PathBuf>,
    _result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
) -> Result<()> {
    let schema_format = format.to_shex_format()?;
    let mut rudof = Rudof::new(config)?;
    rudof_lib::shex::parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config)
        .map_err(|e| anyhow!("{}", e))?;
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
    let mut rudof = Rudof::new(config)?;
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
        let (mut writer, color) = get_writer(output, force_overwrite)?;
        let formatter = match color {
            crate::ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            crate::ColorSupport::WithColor => ShExFormatter::default(),
        };
        rudof_lib::shex::serialize_shex_rudof(
            &rudof,
            &shex,
            &result_schema_format,
            &formatter,
            &mut writer,
        )?;
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
    let mut rudof = Rudof::new(config)?;
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
