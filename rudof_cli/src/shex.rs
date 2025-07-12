use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use crate::anyhow::{bail, Result};
use crate::cli::MimeType;
use crate::writer::get_writer;
use crate::{base_convert, ColorSupport};
use crate::{cli::RDFReaderMode, CliShExFormat, InputSpec};
use anyhow::Context;
use rudof_lib::{Rudof, RudofConfig, ShExFormat, ShExFormatter};
use shex_ast::ShapeExprLabel;

#[allow(clippy::too_many_arguments)]
pub fn run_shex(
    input: &InputSpec,
    schema_format: &CliShExFormat,
    result_schema_format: &CliShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    show_schema: bool,
    compile: bool,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let begin = Instant::now();
    let (writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);

    parse_shex_schema_rudof(&mut rudof, input, schema_format, config)?;
    if show_schema {
        show_shex_schema_rudof(&rudof, result_schema_format, writer, color)?;
    }
    if show_time {
        let elapsed = begin.elapsed();
        let _ = writeln!(io::stderr(), "elapsed: {:.03?} sec", elapsed.as_secs_f64());
    }
    let schema_resolved = rudof.shex_schema_without_imports()?;
    if config.show_extends() {
        show_extends_table(&mut io::stderr(), schema_resolved.count_extends())?;
    }

    if config.show_imports() {
        writeln!(
            io::stderr(),
            "Local shapes: {}/Total shapes {}",
            schema_resolved.local_shapes_count(),
            schema_resolved.total_shapes_count()
        )?;
    }
    if config.show_shapes() {
        for (shape_label, (_shape_expr, iri)) in schema_resolved.shapes() {
            let label = match shape_label {
                ShapeExprLabel::IriRef { value } => {
                    schema_resolved.resolve_iriref(value).as_str().to_string()
                }
                ShapeExprLabel::BNode { value } => format!("{value}"),
                ShapeExprLabel::Start => "Start".to_string(),
            };
            writeln!(io::stderr(), "{label} from {iri}")?
        }
    }
    if compile && config.show_ir() {
        writeln!(io::stdout(), "\nIR:")?;
        if let Some(shex_ir) = rudof.get_shex_ir() {
            writeln!(io::stdout(), "ShEx IR:")?;
            writeln!(io::stdout(), "{shex_ir}")?;
        } else {
            bail!("Internal error: No ShEx schema read")
        }
    }
    if compile && config.show_dependencies() {
        writeln!(io::stdout(), "\nDependencies:")?;
        if let Some(shex_ir) = rudof.get_shex_ir() {
            for (source, posneg, target) in shex_ir.dependencies() {
                writeln!(io::stdout(), "{source}-{posneg}->{target}")?;
            }
        } else {
            bail!("Internal error: No ShEx schema read")
        }
        writeln!(io::stdout(), "---end dependencies\n")?;
    }
    Ok(())
}

// TODO: Replace by show_schema_rudof
/*pub(crate) fn show_shex_schema(
    schema: &SchemaJson,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    match result_schema_format {
        CliShExFormat::Internal => {
            writeln!(writer, "{schema:?}")?;
            Ok(())
        }
        CliShExFormat::ShExC => {
            let formatter = match color {
                ColorSupport::NoColor => ShExFormatter::default().without_colors(),
                ColorSupport::WithColor => ShExFormatter::default(),
            };
            let str = formatter.format_schema(schema);
            writeln!(writer, "{str}")?;
            Ok(())
        }
        CliShExFormat::ShExJ => {
            let str = serde_json::to_string_pretty(&schema)?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
        CliShExFormat::Simple => {
            let mut simplified = SimpleReprSchema::new();
            simplified.from_schema(schema);
            let str = serde_json::to_string_pretty(&simplified)?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
        _ => bail!("Not implemented conversion to {result_schema_format} yet"),
    }
} */

pub(crate) fn show_shex_schema_rudof(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_shex(&shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn parse_shex_schema_rudof(
    rudof: &mut Rudof,
    input: &InputSpec,
    schema_format: &CliShExFormat,
    config: &RudofConfig,
) -> Result<()> {
    let reader = input
        .open_read(Some(&schema_format.mime_type()), "ShEx schema")
        .context(format!("Get reader from input: {input}"))?;
    let schema_format = shex_format_convert(schema_format);
    let shex_config = config.shex_config();
    let base = base_convert(&shex_config.base);
    rudof.read_shex(reader, &schema_format, base)?;
    if config.shex_config().check_well_formed() {
        let shex_ir = rudof.get_shex_ir().unwrap();
        if shex_ir.has_neg_cycle() {
            let neg_cycles = shex_ir.neg_cycles();
            bail!("Schema contains negative cycles: {neg_cycles:?}");
        }
    }
    Ok(())
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

fn shex_format_convert(shex_format: &CliShExFormat) -> ShExFormat {
    match shex_format {
        CliShExFormat::ShExC => ShExFormat::ShExC,
        CliShExFormat::ShExJ => ShExFormat::ShExJ,
        CliShExFormat::Turtle => ShExFormat::Turtle,
        _ => ShExFormat::ShExC,
    }
}
