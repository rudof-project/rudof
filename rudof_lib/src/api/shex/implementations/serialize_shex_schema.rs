use crate::{Result, Rudof, errors::ShExError, formats::ShExFormat, types::ShExStatistics};
use shex_ast::ShExFormatter;
use shex_ast::ShapeLabelIdx;
use shex_ast::ShapeMapParser;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::shapemap::ShapeSelector;
use std::{io, time::Instant};

pub fn serialize_shex_schema<W: io::Write>(
    rudof: &Rudof,
    shape_label: Option<&str>,
    show_schema: Option<bool>,
    show_statistics: Option<bool>,
    show_dependencies: Option<bool>,
    show_time: Option<bool>,
    show_colors: Option<bool>,
    shex_format: Option<&ShExFormat>,
    writer: &mut W,
) -> Result<()> {
    let timer = Instant::now();

    let (shape_label, show_schema, show_statistics, show_dependencies, show_time, show_colors, shex_format) =
        init_defaults(
            shape_label,
            show_schema,
            show_statistics,
            show_dependencies,
            show_time,
            show_colors,
            shex_format,
        );

    if !shape_label.is_empty() {
        serialize_shape(rudof, &shape_label, writer)?;
    }

    if show_schema {
        serialize_schema(rudof, shex_format, show_colors, writer)?;
    }

    if rudof.config.show_ir() {
        serialize_schema_ir(rudof, writer)?;
    }

    if show_statistics {
        serialize_statistics(rudof, writer)?;
    }

    if show_dependencies {
        serialize_dependencies(rudof, writer)?;
    }

    if show_time {
        writeln!(writer, "elapsed: {:.05?} sec", timer.elapsed().as_secs_f64())
            .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    }

    Ok(())
}

fn init_defaults(
    shape_label: Option<&str>,
    show_schema: Option<bool>,
    show_statistics: Option<bool>,
    show_dependencies: Option<bool>,
    show_time: Option<bool>,
    show_colors: Option<bool>,
    shex_format: Option<&ShExFormat>,
) -> (String, bool, bool, bool, bool, bool, ShExFormat) {
    (
        shape_label.map(|s| s.to_string()).unwrap_or_default(),
        show_schema.unwrap_or(true),
        show_statistics.unwrap_or(false),
        show_dependencies.unwrap_or(false),
        show_time.unwrap_or(false),
        show_colors.unwrap_or(false),
        shex_format.copied().unwrap_or_default(),
    )
}

fn get_statistics(rudof: &Rudof) -> Result<ShExStatistics> {
    let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    Ok(ShExStatistics {
        extends_count: shex_schema_ir.count_extends(),
        local_shapes_count: shex_schema_ir.local_shapes_count(),
        total_shapes_count: shex_schema_ir.total_shapes_count(),
        shapes: shex_schema_ir
            .shapes()
            .map(|(l, s, e)| (l.clone(), s.clone(), e.clone()))
            .collect(),
        dependencies: shex_schema_ir.dependencies(),
        has_imports: !shex_schema_ir.imported_schemas().is_empty(),
        neg_cycles: shex_schema_ir.neg_cycles(),
    })
}

fn serialize_statistics<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    let stats = get_statistics(rudof)?;

    writeln!(writer, "\n\n\nStatistics:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

    if rudof.config.show_imports() {
        writeln!(
            writer,
            "- Local shapes: {} / Total shapes {}",
            stats.local_shapes_count, stats.total_shapes_count
        )
        .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    }

    if rudof.config.show_shapes() {
        writeln!(writer, "- Shapes:",).map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        for (label, source, _) in &stats.shapes {
            let from_msg = if stats.has_imports {
                format!(" from {source}")
            } else {
                String::new()
            };
            writeln!(writer, "    - {label} {from_msg}")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        }
    }

    writeln!(writer, "---end statistics\n").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

    Ok(())
}

fn serialize_dependencies<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    writeln!(writer, "\nDependencies:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    for (source, posneg, target) in &get_statistics(rudof)?.dependencies {
        writeln!(writer, "- {source}-{posneg}->{target}")
            .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    }
    writeln!(writer, "---end dependencies\n").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;

    Ok(())
}

fn serialize_schema_ir<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
    writeln!(writer, "# ShEx Schema Internal Representation (IR):")
        .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    writeln!(writer, "{}", shex_schema_ir).map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
    Ok(())
}

fn serialize_schema(
    rudof: &Rudof,
    shex_format: ShExFormat,
    show_colors: bool,
    writer: &mut impl io::Write,
) -> Result<()> {
    let formatter = match show_colors {
        true => ShExFormatter::default(),
        false => ShExFormatter::default().without_colors(),
    };

    let shex_schema = rudof.shex_schema.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    match shex_format {
        ShExFormat::ShExC => {
            formatter
                .write_schema(shex_schema, writer)
                .map_err(|e| ShExError::FailedSerializingShExSchema {
                    format: shex_format.to_string(),
                    error: e.to_string(),
                })?;
        },
        ShExFormat::ShExJ | ShExFormat::Json | ShExFormat::JsonLd => {
            serde_json::to_writer_pretty(writer, &shex_schema).map_err(|e| ShExError::FailedSerializingShExSchema {
                format: shex_format.to_string(),
                error: e.to_string(),
            })?;
        },
        ShExFormat::Internal => {
            let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
            writeln!(writer, "# ShEx Schema Internal Representation (IR):")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            writeln!(writer, "{}", shex_schema_ir)
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        },
        _ => {
            todo!("Implement serialization for ShEx format '{}'", shex_format);
        },
    }

    Ok(())
}

fn serialize_shape<W: io::Write>(rudof: &Rudof, shape_label: &str, writer: &mut W) -> Result<()> {
    let shape_selector = parse_shape_selector(shape_label)?;

    let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    for shape_expr_label in shape_selector.iter_shape() {
        let shape_label =
            ShapeLabel::from_shape_expr_label(shape_expr_label, &shex_schema_ir.prefixmap()).map_err(|e| {
                ShExError::InvalidShapeLabel {
                    label: shape_expr_label.to_string(),
                    error: format!("{e}"),
                }
            })?;
        if let Some((idx, shape_expr)) = shex_schema_ir.find_label(&shape_label) {
            writeln!(writer, "# Shape {shape_label}")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            write!(writer, "  {shape_expr}").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            writeln!(writer, "  # Triple expressions with extends:")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            show_triple_exprs(idx, shex_schema_ir, writer)?;
            writeln!(writer, "Predicates:").map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            let preds = shex_schema_ir.get_preds_extends(idx);
            writeln!(
                writer,
                "  # Predicates: [{}]",
                preds.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")
            )
            .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        } else {
            write!(writer, "Shape {shape_label} not found in schema")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        }
    }
    Ok(())
}

fn show_triple_exprs(idx: &ShapeLabelIdx, schema: &SchemaIR, writer: &mut impl io::Write) -> Result<()> {
    if let Some(triple_exprs) = schema.get_triple_exprs(idx) {
        if let Some(current) = triple_exprs.get(&None) {
            writeln!(
                writer,
                "    Current -> {}",
                current
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        } else {
            writeln!(writer, "    Current -> None?")
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
        }
        if triple_exprs.len() > 1 {
            for (label, exprs) in triple_exprs.iter().filter(|(k, _)| k.is_some()) {
                writeln!(
                    writer,
                    "    {} -> {}",
                    label.as_ref().unwrap(),
                    exprs.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")
                )
                .map_err(|e| ShExError::FailedIoOperation { error: e.to_string() })?;
            }
        }
        Ok(())
    } else {
        Ok(())
    }
}

fn parse_shape_selector(shape_label: &str) -> Result<ShapeSelector> {
    let selector =
        ShapeMapParser::parse_shape_selector(shape_label).map_err(|e| ShExError::ShapeSelectorParseError {
            shape_selector: shape_label.to_string(),
            error: e.to_string(),
        })?;
    Ok(selector)
}
