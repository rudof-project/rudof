use crate::{
    Result, Rudof,
    api::shexmap::ShexmapMaterialization,
    errors::ShExMapError,
    formats::{InputSpec, ResultDataFormat},
    types::Data,
    utils::get_base_iri,
};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_iri::MimeType;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat, Rdf};
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::shexmap::{
    AnalysisReport, Bindings, MaterializerOptions, ShExMapError as CoreError, analyse, bind, materializer, parse_term,
    term::term_from_json,
};
use shex_ast::{Node, Schema as ShExSchema, ShapeExprLabel};
use std::collections::HashMap;
use std::io;

fn core(e: CoreError) -> ShExMapError {
    ShExMapError::Failed { error: e.to_string() }
}

/// A node from its text: `_:label`, `<iri>`, `prefix:local` (with the schema's prefixes) or a
/// bare IRI, relative ones resolved against `base`.
fn parse_node(text: &str, base: &IriS, prefixes: Option<&PrefixMap>) -> std::result::Result<Object, ShExMapError> {
    let text = text.trim();
    let err = |error: String| ShExMapError::InvalidNode {
        node: text.to_string(),
        error,
    };
    if let Some(label) = text.strip_prefix("_:") {
        return Ok(Object::bnode(label.to_string()));
    }
    if text.starts_with('"') {
        return parse_term(text).map_err(|e| err(e.to_string()));
    }
    if text.starts_with('<') && text.ends_with('>') {
        return IriS::from_str_base_iri(&text[1..text.len() - 1], Some(base))
            .map(Object::iri)
            .map_err(|e| err(e.to_string()));
    }
    if let Some(pm) = prefixes
        && let Some((prefix, local)) = text.split_once(':')
        && !local.starts_with("//")
        && let Ok(iri) = pm.resolve_prefix_local(prefix, local)
    {
        return Ok(Object::iri(iri));
    }
    IriS::from_str_base_iri(text, Some(base))
        .map(Object::iri)
        .map_err(|e| err(e.to_string()))
}

/// A shape label from its text; `None` for `START` or nothing.
fn parse_shape(
    text: Option<&str>,
    base: &IriS,
    prefixes: Option<&PrefixMap>,
) -> std::result::Result<Option<ShapeExprLabel>, ShExMapError> {
    let Some(text) = text.map(str::trim).filter(|t| !t.is_empty() && *t != "START") else {
        return Ok(None);
    };
    match parse_node(text, base, prefixes) {
        Ok(Object::Iri(iri)) => Ok(Some(ShapeExprLabel::iri(iri))),
        Ok(Object::BlankNode(b)) => Ok(Some(ShapeExprLabel::BNode {
            value: shex_ast::BNode::new(&b),
        })),
        Ok(_) => Err(ShExMapError::InvalidShapeLabel {
            label: text.to_string(),
            error: "a shape label is an IRI or a blank node".to_string(),
        }),
        Err(e) => Err(ShExMapError::InvalidShapeLabel {
            label: text.to_string(),
            error: e.to_string(),
        }),
    }
}

fn rdf_data(rudof: &Rudof) -> std::result::Result<&sparql_service::RdfData, ShExMapError> {
    match &rudof.data {
        Some(Data::RDFData(rdf)) => Ok(rdf),
        _ => Err(ShExMapError::NoRdfDataLoaded),
    }
}

fn schema<'a>(rudof: &'a Rudof, which: &str) -> std::result::Result<&'a ShExSchema, ShExMapError> {
    rudof
        .shex_schema
        .as_ref()
        .ok_or_else(|| ShExMapError::NoShExSchemaLoaded {
            which: which.to_string(),
        })
}

pub fn shexmap_bind(
    rudof: &mut Rudof,
    focus: &str,
    shape: Option<&str>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
    strict: bool,
    validate: bool,
) -> Result<()> {
    let base_nodes = get_base_iri(rudof, base_nodes)?;
    let base_shapes = get_base_iri(rudof, base_shapes)?;
    let schema_prefixes = schema(rudof, "input")?.prefixmap();
    let focus_node = parse_node(focus, &base_nodes, rdf_data(rudof)?.prefixmap().as_ref())?;
    let start = parse_shape(shape, &base_shapes, schema_prefixes.as_ref())?;

    if validate
        && let (Some(validator), Some(schema_ir)) = (rudof.shex_validator.as_mut(), rudof.shex_schema_ir.as_ref())
    {
        let Some(Data::RDFData(rdf)) = &rudof.data else {
            return Err(ShExMapError::NoRdfDataLoaded.into());
        };
        let label = match &start {
            None => ShapeLabel::Start,
            Some(ShapeExprLabel::IriRef { value }) => ShapeLabel::iri(
                value
                    .get_iri()
                    .map_err(|e| ShExMapError::InvalidShapeLabel {
                        label: format!("{value}"),
                        error: e.to_string(),
                    })?
                    .clone(),
            ),
            Some(ShapeExprLabel::BNode { value }) => ShapeLabel::from_bnode(value.clone()),
            Some(ShapeExprLabel::Start) => ShapeLabel::Start,
        };
        let node = Node::new(focus_node.clone());
        let nodes_prefixmap = rdf.prefixmap();
        let result = validator
            .validate_node_shape(&node, &label, rdf.as_ref(), schema_ir, &nodes_prefixmap)
            .map_err(|e| ShExMapError::Failed { error: e.to_string() })?;
        if let Some(status) = result.get_info(&node, &label)
            && !status.is_conformant()
        {
            return Err(ShExMapError::Validation {
                node: focus_node.to_string(),
                shape: schema_ir.show_label(&label),
                status: status.to_string(),
            }
            .into());
        }
    }

    let schema = schema(rudof, "input")?;
    let rdf = rdf_data(rudof)?;
    let bindings = bind(rdf, schema, &focus_node, start.as_ref(), strict).map_err(core)?;
    rudof.shexmap_bindings = Some(bindings);
    Ok(())
}

pub fn shexmap_load_bindings<R: io::Read>(rudof: &mut Rudof, mut reader: R) -> Result<()> {
    let mut text = String::new();
    reader
        .read_to_string(&mut text)
        .map_err(|e| ShExMapError::Bindings { error: e.to_string() })?;
    let bindings = Bindings::loads(&text).map_err(|e| ShExMapError::Bindings { error: e.to_string() })?;
    rudof.shexmap_bindings = Some(bindings);
    Ok(())
}

pub fn shexmap_serialize_bindings<W: io::Write>(rudof: &Rudof, writer: &mut W, pretty: bool) -> Result<()> {
    let bindings = rudof.shexmap_bindings.as_ref().ok_or(ShExMapError::NoBindings)?;
    let text = bindings.dumps(pretty);
    writeln!(writer, "{text}").map_err(|e| ShExMapError::Bindings { error: e.to_string() })?;
    Ok(())
}

fn rdf_format(result_format: Option<&ResultDataFormat>) -> std::result::Result<(RDFFormat, String), ShExMapError> {
    let format = result_format.copied().unwrap_or_default();
    let rdf: RDFFormat =
        format.try_into().map_err(
            |e: Box<crate::errors::DataError>| ShExMapError::FailedSerializingGraph {
                format: format.to_string(),
                error: e.to_string(),
            },
        )?;
    Ok((rdf, format.to_string()))
}

pub fn shexmap_materialize<W: io::Write>(
    rudof: &mut Rudof,
    root: Option<&str>,
    shape: Option<&str>,
    base_nodes: Option<&str>,
    base_shapes: Option<&str>,
    static_vars: &HashMap<String, String>,
    into: Option<&InputSpec>,
    strict: bool,
    result_format: Option<&ResultDataFormat>,
    writer: &mut W,
) -> Result<ShexmapMaterialization> {
    let base_nodes = get_base_iri(rudof, base_nodes)?;
    let base_shapes = get_base_iri(rudof, base_shapes)?;
    let schema = schema(rudof, "output")?;
    let prefixes = schema.prefixmap();
    let root_node = root
        .map(|r| parse_node(r, &base_nodes, prefixes.as_ref()))
        .transpose()?;
    let start = parse_shape(shape, &base_shapes, prefixes.as_ref())?;
    let bindings = rudof.shexmap_bindings.as_ref().ok_or(ShExMapError::NoBindings)?;
    let (format, format_name) = rdf_format(result_format)?;

    let mut options = MaterializerOptions::default();
    for (var, text) in static_vars {
        let term = match serde_json::from_str::<serde_json::Value>(text) {
            Ok(v @ serde_json::Value::Object(_)) => term_from_json(&v),
            _ => parse_term(text),
        }
        .map_err(|e| ShExMapError::InvalidStaticVariable {
            variable: var.clone(),
            error: e.to_string(),
        })?;
        options.static_vars.insert(var.clone(), term);
    }
    let mut m = materializer::Materializer::new(schema, options);

    let mut graph = match into {
        Some(spec) => {
            let mut reader = spec.open_read(Some(format.mime_type()), "RDF data").map_err(|e| {
                ShExMapError::FailedReadingTarget {
                    target: spec.source_name(),
                    error: e.to_string(),
                }
            })?;
            OxigraphInMemory::from_reader(
                &mut reader,
                &spec.source_name(),
                &format,
                Some(base_nodes.as_str()),
                &ReaderMode::Strict,
            )
            .map_err(|e| ShExMapError::FailedReadingTarget {
                target: spec.source_name(),
                error: e.to_string(),
            })?
        },
        None => OxigraphInMemory::new(),
    };
    for (prefix, ns) in m.prefixes() {
        graph.add_prefix(prefix, &IriS::new_unchecked(ns));
    }
    let (added, removed) = if into.is_some() {
        m.update(&mut graph, &bindings.tree, root_node.as_ref(), start.as_ref())
            .map_err(core)?
    } else {
        let triples = m
            .materialize(&bindings.tree, root_node.as_ref(), start.as_ref())
            .map_err(core)?;
        for t in &triples {
            materializer::add_triple(&mut graph, t).map_err(core)?;
        }
        (triples, Vec::new())
    };
    if strict && m.accepts.len() > 1 {
        return Err(ShExMapError::Failed {
            error: format!("the bindings fit the output schema in {} ways", m.accepts.len()),
        }
        .into());
    }
    graph
        .serialize(&format, writer)
        .map_err(|e| ShExMapError::FailedSerializingGraph {
            format: format_name,
            error: e.to_string(),
        })?;
    let chosen = m
        .chosen
        .map(|i| m.accepts[i].clone())
        .expect("a materialization was chosen");
    Ok(ShexmapMaterialization {
        chosen,
        alternatives: m.accepts.len(),
        added,
        removed,
    })
}

pub fn shexmap_check(
    rudof: &Rudof,
    input_schema: &ShExSchema,
    input_shape: Option<&str>,
    output_shape: Option<&str>,
    base_shapes: Option<&str>,
    static_vars: &[String],
) -> Result<AnalysisReport> {
    let output_schema = schema(rudof, "output")?;
    let base = match base_shapes {
        Some(b) => IriS::new(b).map_err(|e| ShExMapError::InvalidShapeLabel {
            label: b.to_string(),
            error: e.to_string(),
        })?,
        None => rudof
            .config
            .shex()
            .base()
            .cloned()
            .or_else(|| output_schema.base())
            .unwrap_or_else(|| IriS::new_unchecked("http://example.org/")),
    };
    let input_start = parse_shape(input_shape, &base, input_schema.prefixmap().as_ref())?;
    let output_start = parse_shape(output_shape, &base, output_schema.prefixmap().as_ref())?;
    analyse(
        input_schema,
        output_schema,
        input_start.as_ref(),
        output_start.as_ref(),
        static_vars,
    )
    .map_err(|e| core(e).into())
}
