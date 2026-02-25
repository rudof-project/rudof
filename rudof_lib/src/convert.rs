use crate::{
    dctap_format::DCTapFormat, shacl_format::ShaclFormat, shex_format::ShExFormat, RudofError, InputSpec, RudofConfig,
    ReaderMode, Rudof, ShExFormatter, shacl::{add_shacl_schema_rudof, shacl_format_convert}, shex::{parse_shex_schema, serialize_shex_rudof},
    UmlGenerationMode, rdf_core::visualizer::uml_converter::ImageFormat, data::get_base,
};
use rudof_rdf::rdf_core::visualizer::uml_converter::UmlConverter;
use shapes_converter::{ShEx2Html, ShEx2Sparql, ShEx2Uml, Shacl2ShEx, Tap2ShEx};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::io::Write;
use iri_s::{IriS, MimeType};
use prefixmap::IriRef;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InputConvertMode {
    Shacl,
    ShEx,
    Dctap,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::Shacl => write!(dest, "shacl"),
            InputConvertMode::ShEx => write!(dest, "shex"),
            InputConvertMode::Dctap => write!(dest, "dctap"),
        }
    }
}

impl FromStr for InputConvertMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shacl" => Ok(InputConvertMode::Shacl),
            "shex" => Ok(InputConvertMode::ShEx),
            "dctap" => Ok(InputConvertMode::Dctap),
            _ => Err(format!(
                "Invalid conversion mode: '{}'. Supported modes: shacl, shex, dctap",
                s
            )),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum InputConvertFormat {
    Csv,
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    Xlsx,
}

impl InputConvertFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, String> {
        match self {
            InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            InputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            InputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => Err(format!("Converting ShEx, format {self} not supported")),
        }
    }
    pub fn to_shacl_format(self) -> Result<ShaclFormat, String> {
        match self {
            InputConvertFormat::Turtle => Ok(ShaclFormat::Turtle),
            _ => Err(format!("Converting to SHACL, format {self} not supported")),
        }
    }

    pub fn to_dctap_format(self) -> Result<DCTapFormat, String> {
        match self {
            InputConvertFormat::Csv => Ok(DCTapFormat::Csv),
            InputConvertFormat::Xlsx => Ok(DCTapFormat::Xlsx),
            _ => Err(format!("Converting to DCTAP, format {self} not supported")),
        }
    }
}

impl FromStr for InputConvertFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(InputConvertFormat::Csv),
            "xlsx" => Ok(InputConvertFormat::Xlsx),
            "shexc" => Ok(InputConvertFormat::ShExC),
            "shexj" => Ok(InputConvertFormat::ShExJ),
            "turtle" => Ok(InputConvertFormat::Turtle),
            _ => Err(format!("Unsupported input convert format {s}")),
        }
    }
}

impl Display for InputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertFormat::Csv => write!(dest, "csv"),
            InputConvertFormat::Xlsx => write!(dest, "xlsx"),
            InputConvertFormat::ShExC => write!(dest, "shexc"),
            InputConvertFormat::ShExJ => write!(dest, "shexj"),
            InputConvertFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OutputConvertFormat {
    Default,
    Internal,
    Json,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    Html,
    Svg,
    Png,
}

impl OutputConvertFormat {
    pub fn to_shex_format(self) -> Result<ShExFormat, String> {
        match self {
            OutputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
            OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
            OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
            _ => Err(format!("Converting ShEx, format {self} not supported")),
        }
    }

    pub fn to_shacl_format(self) -> Result<ShaclFormat, String> {
        match self {
            OutputConvertFormat::Default => Ok(ShaclFormat::Internal),
            OutputConvertFormat::Turtle => Ok(ShaclFormat::Turtle),
            _ => Err(format!("Converting to SHACL, format {self} not supported")),
        }
    }
}

impl Display for OutputConvertFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertFormat::Internal => write!(dest, "internal"),
            OutputConvertFormat::Json => write!(dest, "json"),
            OutputConvertFormat::Default => write!(dest, "default"),
            OutputConvertFormat::ShExC => write!(dest, "shexc"),
            OutputConvertFormat::ShExJ => write!(dest, "shexj"),
            OutputConvertFormat::Turtle => write!(dest, "turtle"),
            OutputConvertFormat::PlantUML => write!(dest, "uml"),
            OutputConvertFormat::Html => write!(dest, "html"),
            OutputConvertFormat::Png => write!(dest, "png"),
            OutputConvertFormat::Svg => write!(dest, "svg"),
        }
    }
}

impl FromStr for OutputConvertFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(OutputConvertFormat::Default),
            "internal" => Ok(OutputConvertFormat::Internal),
            "json" => Ok(OutputConvertFormat::Json),
            "shexc" => Ok(OutputConvertFormat::ShExC),
            "shexj" => Ok(OutputConvertFormat::ShExJ),
            "turtle" => Ok(OutputConvertFormat::Turtle),
            "uml" => Ok(OutputConvertFormat::PlantUML),
            "html" => Ok(OutputConvertFormat::Html),
            "svg" => Ok(OutputConvertFormat::Svg),
            "png" => Ok(OutputConvertFormat::Png),
            _ => Err(format!("Unsupported output conversion format: '{}'", s)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OutputConvertMode {
    Sparql,
    ShEx,
    Uml,
    Html,
    Shacl,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::Sparql => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::Uml => write!(dest, "uml"),
            OutputConvertMode::Html => write!(dest, "html"),
            OutputConvertMode::Shacl => write!(dest, "shacl"),
        }
    }
}

impl FromStr for OutputConvertMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sparql" => Ok(OutputConvertMode::Sparql),
            "shex" => Ok(OutputConvertMode::ShEx),
            "uml" => Ok(OutputConvertMode::Uml),
            "html" => Ok(OutputConvertMode::Html),
            "shacl" => Ok(OutputConvertMode::Shacl),
            _ => Err(format!(
                "Unknown output conversion mode: '{}'. Supported modes: sparql, shex, uml, html, shacl",
                s
            )),
        }
    }
}

pub fn run_shacl2shex(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    reader_mode: &ReaderMode,
    formatter: &ShExFormatter,
    writer: &mut Box<dyn Write>
) -> Result<(), RudofError> {
    let schema_format = match format {
        InputConvertFormat::Turtle => Ok(ShaclFormat::Turtle),
        _ => Err(RudofError::UnsupportedShaclInputFormat {
            format: format.to_string(),
        }),
    }?;
    let mut rudof = Rudof::new(config)?;
    add_shacl_schema_rudof(&mut rudof, input, &schema_format, base, reader_mode, config)?;
    let shacl_schema = rudof.get_shacl().ok_or(RudofError::NoShaclSchemaFound)?;
    let mut converter = Shacl2ShEx::new(&config.shacl2shex_config());
    converter
        .convert(shacl_schema)
        .map_err(|e| RudofError::Shacl2ShExError { error: e.to_string() })?;
    let result_schema_format = result_format
        .to_shex_format()
        .map_err(|e| RudofError::UnsupportedShExInputFormat { format: e })?;
    serialize_shex_rudof(
        &rudof,
        converter.current_shex(),
        &result_schema_format,
        &formatter,
        writer,
    )?;
    Ok(())
}

pub fn run_shex2uml(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    result_format: &OutputConvertFormat,
    maybe_shape: &Option<String>,
    config: &RudofConfig,
    reader_mode: &ReaderMode,
    writer: &mut Box<dyn Write>
) -> Result<(), RudofError> {
    let schema_format = format
        .to_shex_format()
        .map_err(|e| RudofError::UnsupportedShExInputFormat { format: e })?;
    let mut rudof = Rudof::new(config)?;
    parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config).map_err(|e| {
        RudofError::ShExCParserError {
            error: e.to_string(),
            source_name: input.to_string(),
        }
    })?;
    let schema = rudof.get_shex().ok_or(RudofError::NoShExSchema)?;
    let mut converter = ShEx2Uml::new(&config.shex2uml_config());
    converter
        .convert(schema)
        .map_err(|e| RudofError::ShEx2UmlError { error: e.to_string() })?;
    generate_uml_output(
        converter,
        maybe_shape,
        writer,
        result_format,
        config.shex2uml_config().plantuml_path(),
    )
}

pub fn generate_uml_output<P: AsRef<Path>>(
    uml_converter: ShEx2Uml,
    maybe_shape: &Option<String>,
    writer: &mut Box<dyn Write>,
    result_format: &OutputConvertFormat,
    plantuml_path: P,
) -> Result<(), RudofError> {
    let mode = if let Some(str) = maybe_shape {
        UmlGenerationMode::neighs(str)
    } else {
        UmlGenerationMode::all()
    };
    match result_format {
        OutputConvertFormat::PlantUML | OutputConvertFormat::Default => {
            uml_converter
                .as_plantuml(writer, &mode)
                .map_err(|e| RudofError::ShEx2PlantUmlErrorAsPlantUML { error: e.to_string() })?;
            Ok(())
        },
        OutputConvertFormat::Svg => {
            uml_converter
                .as_image(writer, ImageFormat::SVG, &mode, plantuml_path)
                .map_err(|e| RudofError::ShEx2PlantUmlError { error: e.to_string() })?;
            Ok(())
        },
        OutputConvertFormat::Png => {
            uml_converter
                .as_image(writer, ImageFormat::PNG, &mode, plantuml_path)
                .map_err(|e| RudofError::ShEx2PlantUmlError { error: e.to_string() })?;
            Ok(())
        },
        _ => Err(RudofError::UnsupportedUmlOutputFormat {
            format: result_format.to_string(),
        }),
    }
}

pub fn run_shex2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    output_folder: P,
    template_folder: &Option<PathBuf>,
    config: &RudofConfig,
    reader_mode: &ReaderMode,
) -> Result<(), RudofError> {
    let schema_format = format
        .to_shex_format()
        .map_err(|e| RudofError::UnsupportedShExInputFormat { format: e })?;
    let mut rudof = Rudof::new(config)?;
    parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config).map_err(|e| {
        RudofError::ShExCParserError {
            error: e.to_string(),
            source_name: input.to_string(),
        }
    })?;
    let schema = rudof.get_shex().ok_or(RudofError::NoShExSchema)?;
    let shex2html_config = config.shex2html_config();
    let html_config = shex2html_config.clone().with_target_folder(output_folder.as_ref());
    let mut converter = ShEx2Html::new(html_config);
    converter
        .convert(schema)
        .map_err(|e| RudofError::ShEx2HtmlError { error: e.to_string() })?;
    let resolved_template = match template_folder {
        Some(tf) => tf.to_path_buf(),
        None => shex2html_config
            .template_folder
            .map(PathBuf::from)
            .ok_or(RudofError::NoTemplateFolder)?,
    };
    converter
        .export_schema(resolved_template)
        .map_err(|e| RudofError::ShEx2HtmlError { error: e.to_string() })?;
    Ok(())
}

pub fn run_tap2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    output_folder: P,
    template_folder: &Option<PathBuf>,
    config: &RudofConfig,
) -> Result<(), RudofError> {
    let mut rudof = Rudof::new(config)?;
    let dctap_format = format
        .to_dctap_format()
        .map_err(|e| RudofError::UnsupportedDCTAPInputFormat { format: e })?;
    rudof.read_dctap_input(input, &dctap_format.into())?;
    let dctap = rudof.get_dctap().ok_or(RudofError::NoDCTAP)?;
    let converter_tap = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter_tap
        .convert(dctap)
        .map_err(|e| RudofError::Tap2ShExError { error: e.to_string() })?;
    let shex2html_config = config
        .shex2html_config()
        .clone()
        .with_target_folder(output_folder.as_ref());
    let mut converter = ShEx2Html::new(shex2html_config.clone());
    converter
        .convert(&shex)
        .map_err(|e| RudofError::ShEx2HtmlError { error: e.to_string() })?;
    let resolved_template = match template_folder {
        Some(tf) => tf.to_path_buf(),
        None => shex2html_config
            .template_folder
            .map(PathBuf::from)
            .ok_or(RudofError::NoTemplateFolder)?,
    };
    converter
        .export_schema(resolved_template)
        .map_err(|e| RudofError::ShEx2HtmlError { error: e.to_string() })?;
    Ok(())
}

pub fn run_shex2sparql(
    input: &InputSpec,
    format: &InputConvertFormat,
    base: &Option<IriS>,
    shape: Option<IriRef>,
    config: &RudofConfig,
    reader_mode: &ReaderMode,
    writer: &mut Box<dyn Write>
) -> Result<(), RudofError> {
    let schema_format = format
        .to_shex_format()
        .map_err(|e| RudofError::UnsupportedShExInputFormat { format: e })?;
    let mut rudof = Rudof::new(config)?;
    parse_shex_schema(&mut rudof, input, &schema_format, base, reader_mode, config).map_err(|e| {
        RudofError::ShExCParserError {
            error: e.to_string(),
            source_name: input.to_string(),
        }
    })?;
    let schema = rudof.get_shex().ok_or(RudofError::NoShExSchema)?;
    let converter = ShEx2Sparql::new(&config.shex2sparql_config());
    let sparql = converter
        .convert(schema, shape)
        .map_err(|e| RudofError::ShEx2SparqlError { error: e.to_string() })?;
    write!(writer, "{sparql}").map_err(|e| RudofError::WritingOutputError { error: e.to_string() })?;
    Ok(())
}

pub fn run_tap2shex(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    writer: &mut Box<dyn Write>,
    formatter: &ShExFormatter,
) -> Result<(), RudofError> {
    let mut rudof = Rudof::new(config)?;
    let tap_format = format
        .to_dctap_format()
        .map_err(|e| RudofError::UnsupportedDCTAPInputFormat { format: e })?;
    rudof.read_dctap_input(input_path, &tap_format.into())?;
    let dctap = rudof.get_dctap().ok_or(RudofError::NoDCTAP)?;
    let converter = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter
        .convert(dctap)
        .map_err(|e| RudofError::Tap2ShExError { error: e.to_string() })?;
    let result_schema_format = result_format
        .to_shex_format()
        .map_err(|e| RudofError::UnsupportedShExInputFormat { format: e })?;
    serialize_shex_rudof(&rudof, &shex, &result_schema_format, &formatter, writer)?;
    Ok(())
}

pub fn run_tap2uml(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    maybe_shape: &Option<String>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    writer: &mut Box<dyn Write>
) -> Result<(), RudofError> {
    let mut rudof = Rudof::new(config)?;
    let tap_format = format
        .to_dctap_format()
        .map_err(|e| RudofError::UnsupportedDCTAPInputFormat { format: e })?;
    rudof.read_dctap_input(input_path, &tap_format.into())?;
    let dctap = rudof.get_dctap().ok_or(RudofError::NoDCTAP)?;
    let converter_shex = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter_shex
        .convert(dctap)
        .map_err(|e| RudofError::Tap2ShExError { error: e.to_string() })?;
    let mut converter_uml = ShEx2Uml::new(&config.shex2uml_config());
    converter_uml
        .convert(&shex)
        .map_err(|e| RudofError::ShEx2UmlError { error: e.to_string() })?;
    generate_uml_output(
        converter_uml,
        maybe_shape,
        writer,
        result_format,
        config.shex2uml_config().plantuml_path(),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn run_shacl_convert(
    input: &InputSpec,
    input_format: &ShaclFormat,
    base: &Option<IriS>,
    output_format: &ShaclFormat,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
    writer: &mut Box<dyn Write>
) -> Result<(), RudofError> {
    let mut rudof = Rudof::new(config)?;
    let mime_type = input_format.mime_type();
    let mut reader = input.open_read(Some(mime_type), "SHACL shapes").map_err(|e| {
        RudofError::SHACLParseError { error: (e.to_string()) }
    })?;
    let input_format = shacl_format_convert(*input_format)?;
    let base = get_base(input, config, base)?;
    rudof.read_shacl(
        &mut reader,
        &input.to_string(),
        &input_format,
        base.as_deref(),
        reader_mode,
    )?;
    let output_format = shacl_format_convert(*output_format)?;
    rudof.serialize_shacl(&output_format, writer)?;
    Ok(())
}
