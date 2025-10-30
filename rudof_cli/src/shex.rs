use crate::writer::get_writer;
use crate::ColorSupport;
use anyhow::Context;
use anyhow::{Result, bail};
use iri_s::IriS;
use iri_s::mime_type::MimeType;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, RudofError, ShExFormatter, 
    data_format::DataFormat, parse_shape_selector, shex::validate_shex, 
    shapemap_format::ShapeMapFormat as CliShapeMapFormat, shex_format::ShExFormat as CliShExFormat, sort_by_result_shape_map::SortByResultShapeMap,
    result_shex_validation_format::ResultShExValidationFormat, data::get_data_rudof
};
use shex_ast::shapemap::ShapeSelector;
use shex_ast::{Schema, ShExFormat};
use srdf::{RDFFormat, ReaderMode};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use tracing::trace;
use url::Url;

#[allow(clippy::too_many_arguments)]
pub fn run_shex(
    input: &InputSpec,
    schema_format: &CliShExFormat,
    shape: &Option<String>,
    base: &Option<IriS>,
    result_schema_format: &CliShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    show_schema: bool,
    compile: bool,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let begin = Instant::now();
    let (mut writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;

    parse_shex_schema_rudof(&mut rudof, input, schema_format, base, reader_mode, config)?;
    if let Some(shape_label) = shape {
        let shape_selector = parse_shape_selector(shape_label)?;
        show_shape_shex_schema_rudof(
            &rudof,
            &shape_selector,
            result_schema_format,
            &mut *writer,
            color.clone(),
        )?;
    }
    if show_schema {
        show_shex_schema_rudof(&rudof, result_schema_format, &mut *writer, color)?;
    }
    if show_time {
        let elapsed = begin.elapsed();
        let _ = writeln!(io::stdout(), "elapsed: {:.03?} sec", elapsed.as_secs_f64());
    }
    if compile && config.show_ir() {
        if let Some(shex_ir) = rudof.get_shex_ir() {
            trace!("Schema compiled to IR");
            writeln!(io::stdout(), "ShEx Internal Representation:")?;
            writeln!(io::stdout(), "{shex_ir}")?;

            if config.show_extends() {
                show_extends_table(&mut io::stderr(), shex_ir.count_extends())?;
            }

            if config.show_imports() {
                writeln!(
                    &mut writer,
                    "Local shapes: {}/Total shapes {}",
                    shex_ir.local_shapes_count(),
                    shex_ir.total_shapes_count()
                )?;
            }
            if config.show_shapes() {
                for (label, source, _expr) in shex_ir.shapes() {
                    writeln!(
                        &mut writer,
                        "{label}{}",
                        if shex_ir.imported_schemas().is_empty() {
                            String::new()
                        } else {
                            format!(" from {source}")
                        }
                    )?;
                }
            }
            if config.show_dependencies() {
                writeln!(&mut writer, "\nDependencies:")?;
                for (source, posneg, target) in shex_ir.dependencies() {
                    writeln!(io::stdout(), "{source}-{posneg}->{target}")?;
                }
                writeln!(&mut writer, "---end dependencies\n")?;
            }
        } else {
            bail!("Internal error: Schema was not compiled to IR")
        }
        Ok(())
    } else {
        Ok(())
    }
}

pub fn show_shex_schema_rudof(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    mut writer: &mut dyn Write,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_current_shex(&shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn show_shape_shex_schema_rudof(
    rudof: &Rudof,
    shape: &ShapeSelector,
    result_schema_format: &CliShExFormat,
    mut writer: &mut dyn Write,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_shape_current_shex(shape, &shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn show_shex_schema(
    rudof: &Rudof,
    shex: &Schema,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_shex(shex, &shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn parse_shex_schema_rudof(
    rudof: &mut Rudof,
    input: &InputSpec,
    schema_format: &CliShExFormat,
    base: &Option<IriS>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let reader = input
        .open_read(Some(schema_format.mime_type()), "ShEx schema")
        .context(format!("Get reader from input: {input}"))?;
    let schema_format = shex_format_convert(schema_format);
    let base = get_base(config, base)?;
    rudof.read_shex(
        reader,
        &schema_format,
        Some(base.as_str()),
        reader_mode,
        Some(&input.source_name()),
    )?;
    if config.shex_config().check_well_formed() {
        let shex_ir = rudof.get_shex_ir().unwrap();
        if shex_ir.has_neg_cycle() {
            let neg_cycles = shex_ir.neg_cycles();
            bail!("Schema contains negative cycles: {neg_cycles:?}");
        }
    }
    Ok(())
}

fn get_base(config: &RudofConfig, base: &Option<IriS>) -> Result<IriS, RudofError> {
    if let Some(base) = base {
        Ok(base.clone())
    } else if let Some(base) = config.shex_config().base.as_ref() {
        Ok(base.clone())
    } else {
        let cwd = env::current_dir().map_err(|e| RudofError::CurrentDirError {
            error: format!("{e}"),
        })?;
        // Note: we use from_directory_path to convert a directory to a file URL that ends with a trailing slash
        // from_url_path would not add the trailing slash and would fail when resolving relative IRIs
        let url =
            Url::from_directory_path(&cwd).map_err(|_| RudofError::ConvertingCurrentFolderUrl {
                current_dir: cwd.to_string_lossy().to_string(),
            })?;
        let iri = IriS::from_url(&url);
        Ok(iri)
    }
}

fn show_extends_table<R: Write>(
    writer: &mut R,
    extends_count: HashMap<usize, usize>,
) -> Result<()> {
    for (key, value) in extends_count.iter() {
        writeln!(writer, "Shapes with {key} extends = {value}")?;
    }
    Ok(())
}

pub fn shex_format_convert(shex_format: &CliShExFormat) -> ShExFormat {
    match shex_format {
        CliShExFormat::ShExC => ShExFormat::ShExC,
        CliShExFormat::ShExJ => ShExFormat::ShExJ,
        CliShExFormat::Turtle => ShExFormat::RDFFormat(RDFFormat::Turtle),
        _ => ShExFormat::ShExC,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_validate_shex(
    schema: &Option<InputSpec>,
    schema_format: &Option<CliShExFormat>,
    base_schema: &Option<IriS>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base_data: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap: &Option<InputSpec>,
    shapemap_format: &CliShapeMapFormat,
    _debug: u8,
    result_format: &ResultShExValidationFormat,
    sort_by: &SortByResultShapeMap,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let mut rudof = Rudof::new(config)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;

    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        base_data,
        endpoint,
        reader_mode,
        config,
        false,
    )?;

    validate_shex(
        &mut rudof, 
        schema, 
        schema_format, 
        base_schema,  
        reader_mode, 
        maybe_node, 
        maybe_shape, 
        shapemap, 
        shapemap_format, 
        result_format, 
        sort_by, 
        config, 
        &mut writer
    ).map_err(anyhow::Error::from)
        
}