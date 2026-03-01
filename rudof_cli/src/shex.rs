use crate::ColorSupport;
use crate::writer::get_writer;
use anyhow::{Result, bail};
use iri_s::IriS;
use rudof_lib::{
    InputSpec, Rudof, RudofConfig, ShExFormatter, data::get_data_rudof, data_format::DataFormat, parse_shape_selector,
    result_shex_validation_format::ResultShExValidationFormat, shapemap_format::ShapeMapFormat as CliShapeMapFormat,
    shex::validate_shex, shex_format::ShExFormat as CliShExFormat, sort_by_result_shape_map::SortByResultShapeMap,
};
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::ReaderMode;
use shex_ast::ShExFormat;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use tracing::trace;

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

    rudof_lib::shex::parse_shex_schema(&mut rudof, input, schema_format, base, reader_mode, config)
        .map_err(anyhow::Error::from)?;

    if let Some(shape_label) = shape {
        let shape_selector = parse_shape_selector(shape_label)?;
        let formatter = match color {
            ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            ColorSupport::WithColor => ShExFormatter::default(),
        };
        rudof_lib::shex::serialize_shape_current_shex_rudof(
            &rudof,
            &shape_selector,
            result_schema_format,
            &formatter,
            &mut writer,
        )
        .map_err(anyhow::Error::from)?;
    }
    if show_schema {
        let formatter = match color {
            ColorSupport::NoColor => ShExFormatter::default().without_colors(),
            ColorSupport::WithColor => ShExFormatter::default(),
        };
        rudof_lib::shex::serialize_current_shex_rudof(&rudof, result_schema_format, &formatter, &mut writer)
            .map_err(anyhow::Error::from)?;
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

fn show_extends_table<R: Write>(writer: &mut R, extends_count: HashMap<usize, usize>) -> Result<()> {
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
    trace!(
        "Running ShEx validation with schema: {:?}, base_schema: {:?} and data: {:?}, base_data: {:?}",
        schema, base_schema, data, base_data
    );

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
        &mut writer,
    )
    .map_err(anyhow::Error::from)
}
