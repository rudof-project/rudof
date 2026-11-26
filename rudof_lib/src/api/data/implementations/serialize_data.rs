use crate::{Result, Rudof, errors::DataError, formats::ResultDataFormat};
use colored::{Color, Colorize};
use rudof_rdf::rdf_core::{RDFFormat, visualizer::VisualRDFGraph};
use rudof_viz::{DiagramScope, VizEngine};
use serde_json::Value;
use std::io;

pub fn serialize_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: Option<&ResultDataFormat>,
    viz_engine: Option<&VizEngine>,
    writer: &mut W,
) -> Result<()> {
    let result_data_format = result_data_format.copied().unwrap_or_default();
    let viz_engine = viz_engine.copied().unwrap_or_default();

    let data = rudof.data.as_ref().ok_or(Box::new(DataError::NoDataLoaded))?;

    if data.is_rdf() {
        serialize_rdf_data(rudof, result_data_format, viz_engine, writer)
    } else {
        serialize_pg_data(rudof, result_data_format, writer)
    }
}

#[cfg(not(feature = "pgschema"))]
fn serialize_pg_data<W: io::Write>(
    _rudof: &mut Rudof,
    _result_data_format: ResultDataFormat,
    _writer: &mut W,
) -> Result<()> {
    Err(Box::new(DataError::NoPgDataLoaded))?
}

#[cfg(feature = "pgschema")]
fn serialize_pg_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: ResultDataFormat,
    writer: &mut W,
) -> Result<()> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_pg() {
        Err(Box::new(DataError::NoPgDataLoaded))?
    }

    let graph = data.unwrap_pg_mut();
    let failed = |error: String| {
        Box::new(DataError::FailedSerializingData {
            format: result_data_format.to_string(),
            error,
        })
    };

    // Property graphs are written as JSON with `json`, and otherwise in
    // YARS-PG, the syntax they are read from (RDF formats don't apply).
    let serialized = match result_data_format {
        ResultDataFormat::Json => serde_json::to_string_pretty(&graph.to_json()).map_err(|e| failed(e.to_string()))?,
        _ => graph.to_yarspg(),
    };
    writeln!(writer, "{serialized}").map_err(|e| failed(e.to_string()))?;

    Ok(())
}

fn serialize_rdf_data<W: io::Write>(
    rudof: &mut Rudof,
    result_data_format: ResultDataFormat,
    viz_engine: VizEngine,
    writer: &mut W,
) -> Result<()> {
    let pretty_json = rudof.config.rdf_data().pretty_json();
    let colorize_json = rudof.config.rdf_data().colorize_json();

    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoRdfDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    if result_data_format.is_rdf_format() {
        let rdf_format: RDFFormat = result_data_format.try_into()?;
        if rdf_format == RDFFormat::JsonLd && pretty_json {
            let mut buf = Vec::new();
            data.unwrap_rdf_mut().serialize(&rdf_format, &mut buf).map_err(|e| {
                Box::new(DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })
            })?;
            write_pretty_json(writer, &buf, colorize_json).map_err(|e| {
                Box::new(DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })
            })?;
        } else {
            data.unwrap_rdf_mut().serialize(&rdf_format, writer).map_err(|e| {
                Box::new(DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })
            })?;
        }
    } else {
        let visualization_config = rudof.config.rdf_data().rdf_visualization_config().clone();
        let converter = VisualRDFGraph::from_rdf(data.unwrap_rdf_mut(), visualization_config).map_err(|e| {
            Box::new(DataError::FailedSerializingData {
                format: result_data_format.to_string(),
                error: e.to_string(),
            })
        })?;

        if result_data_format.is_image_visualization_format() {
            converter
                .as_image(
                    writer,
                    result_data_format.try_into()?,
                    &DiagramScope::all(),
                    viz_engine,
                    plantuml_path(rudof),
                )
                .map_err(|e| {
                    Box::new(DataError::FailedSerializingData {
                        format: result_data_format.to_string(),
                        error: e.to_string(),
                    })
                })?;
        } else {
            converter.as_plantuml(writer, &DiagramScope::All).map_err(|e| {
                Box::new(DataError::FailedSerializingData {
                    format: result_data_format.to_string(),
                    error: e.to_string(),
                })
            })?
        }
    }

    Ok(())
}

/// Re-parses compact JSON/JSON-LD bytes and re-emits them indented, optionally styled with
/// ANSI colors (keys cyan, strings green, numbers yellow, booleans magenta, null dimmed).
pub(crate) fn write_pretty_json<W: io::Write>(writer: &mut W, compact: &[u8], colorize: bool) -> io::Result<()> {
    let value: Value = serde_json::from_slice(compact).map_err(io::Error::other)?;
    write_json_value(writer, &value, colorize, 0)?;
    writeln!(writer)
}

const JSON_INDENT: &str = "  ";

fn write_json_value<W: io::Write>(writer: &mut W, value: &Value, colorize: bool, level: usize) -> io::Result<()> {
    match value {
        Value::Null => write_json_scalar(writer, "null", colorize, Color::BrightBlack),
        Value::Bool(b) => write_json_scalar(writer, &b.to_string(), colorize, Color::Magenta),
        Value::Number(n) => write_json_scalar(writer, &n.to_string(), colorize, Color::Yellow),
        Value::String(s) => {
            let quoted = serde_json::to_string(s).map_err(io::Error::other)?;
            write_json_scalar(writer, &quoted, colorize, Color::Green)
        },
        Value::Array(items) => {
            if items.is_empty() {
                return write!(writer, "[]");
            }
            writeln!(writer, "[")?;
            let last = items.len() - 1;
            for (i, item) in items.iter().enumerate() {
                write!(writer, "{}", JSON_INDENT.repeat(level + 1))?;
                write_json_value(writer, item, colorize, level + 1)?;
                writeln!(writer, "{}", if i < last { "," } else { "" })?;
            }
            write!(writer, "{}]", JSON_INDENT.repeat(level))
        },
        Value::Object(map) => {
            if map.is_empty() {
                return write!(writer, "{{}}");
            }
            writeln!(writer, "{{")?;
            let last = map.len() - 1;
            for (i, (key, val)) in map.iter().enumerate() {
                write!(writer, "{}", JSON_INDENT.repeat(level + 1))?;
                let key_str = serde_json::to_string(key).map_err(io::Error::other)?;
                if colorize {
                    write!(writer, "{}", key_str.cyan().bold())?;
                } else {
                    write!(writer, "{key_str}")?;
                }
                write!(writer, ": ")?;
                write_json_value(writer, val, colorize, level + 1)?;
                writeln!(writer, "{}", if i < last { "," } else { "" })?;
            }
            write!(writer, "{}}}", JSON_INDENT.repeat(level))
        },
    }
}

fn write_json_scalar<W: io::Write>(writer: &mut W, text: &str, colorize: bool, color: Color) -> io::Result<()> {
    if colorize {
        write!(writer, "{}", text.color(color))
    } else {
        write!(writer, "{text}")
    }
}

/// The PlantUML jar used to render diagrams, configured in the `shex2uml`
/// section of the configuration.
#[cfg(feature = "conversion")]
fn plantuml_path(rudof: &Rudof) -> std::path::PathBuf {
    rudof.config.shex2uml().plantuml_path().clone()
}

/// The PlantUML jar used to render diagrams. The `shex2uml` section of the
/// configuration needs the `conversion` feature, so this is its default: the
/// `PLANTUML` environment variable, or `plantuml.jar`.
#[cfg(not(feature = "conversion"))]
fn plantuml_path(_rudof: &Rudof) -> std::path::PathBuf {
    std::env::var("PLANTUML").map_or_else(|_| "plantuml.jar".into(), Into::into)
}

#[cfg(test)]
mod tests {
    use super::write_pretty_json;

    #[test]
    fn pretty_json_indents_nested_objects_and_arrays() {
        let compact =
            br#"[{"@id":"http://example.org/alice","http://example.org/knows":[{"@id":"http://example.org/bob"}]}]"#;
        let mut out = Vec::new();
        write_pretty_json(&mut out, compact, false).unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            r#"[
  {
    "@id": "http://example.org/alice",
    "http://example.org/knows": [
      {
        "@id": "http://example.org/bob"
      }
    ]
  }
]
"#
        );
    }

    #[test]
    fn pretty_json_handles_empty_objects_and_arrays() {
        let mut out = Vec::new();
        write_pretty_json(&mut out, br#"{"a":[],"b":{}}"#, false).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "{\n  \"a\": [],\n  \"b\": {}\n}\n");
    }
}
