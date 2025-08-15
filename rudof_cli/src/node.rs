extern crate anyhow;
use anyhow::*;
use prefixmap::IriRef;
use shapemap::NodeSelector;
use shex_ast::ObjectValue;
use srdf::NeighsRDF;
use std::result::Result::Ok;

use std::{io::Write, path::PathBuf};

use rudof_lib::{Rudof, RudofConfig, ShapeMapParser};

use crate::data_format::DataFormat;
use crate::{
    data::get_data_rudof, data_format, input_spec::InputSpec, node_selector::parse_node_selector,
    writer::get_writer, RDFReaderMode, ShowNodeMode,
};

#[allow(clippy::too_many_arguments)]
pub fn run_node(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    node_str: &str,
    predicates: &Vec<String>,
    show_node_mode: &ShowNodeMode,
    show_hyperlinks: &bool,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;
    let data = rudof.get_rdf_data();
    let node_selector = parse_node_selector(node_str)?;
    tracing::debug!("Node info with node selector: {node_selector:?}");
    show_node_info(
        node_selector,
        predicates,
        data,
        show_node_mode,
        show_hyperlinks,
        &mut writer,
    )
}

fn show_node_info<S, W: Write>(
    node_selector: NodeSelector,
    predicates: &Vec<String>,
    rdf: &S,
    show_node_mode: &ShowNodeMode,
    _show_hyperlinks: &bool,
    writer: &mut W,
) -> Result<()>
where
    S: NeighsRDF,
{
    for node in node_selector.iter_node(rdf) {
        let subject = node_to_subject(node, rdf)?;
        writeln!(
            writer,
            "Information about {}",
            rdf.qualify_subject(&subject)
        )?;

        // Show outgoing arcs
        match show_node_mode {
            ShowNodeMode::Outgoing | ShowNodeMode::Both => {
                writeln!(writer, "Outgoing arcs")?;
                let map = if predicates.is_empty() {
                    match rdf.outgoing_arcs(subject.clone()) {
                        Result::Ok(rs) => rs,
                        Err(e) => bail!("Error obtaining outgoing arcs of {subject}: {e}"),
                    }
                } else {
                    let preds = cnv_predicates(predicates, rdf)?;
                    match rdf.outgoing_arcs_from_list(&subject, &preds) {
                        Result::Ok((rs, _)) => rs,
                        Err(e) => bail!("Error obtaining outgoing arcs of {subject}: {e}"),
                    }
                };
                writeln!(writer, "{}", rdf.qualify_subject(&subject))?;
                let mut preds: Vec<_> = map.keys().collect();
                preds.sort();
                for pred in preds {
                    writeln!(writer, " -{}-> ", rdf.qualify_iri(pred))?;
                    if let Some(objs) = map.get(pred) {
                        for o in objs {
                            writeln!(writer, "      {}", rdf.qualify_term(o))?;
                        }
                    } else {
                        bail!("Not found values for {pred} in map")
                    }
                }
            }
            _ => {
                // Nothing to do
            }
        }

        // Show incoming arcs
        match show_node_mode {
            ShowNodeMode::Incoming | ShowNodeMode::Both => {
                writeln!(writer, "Incoming arcs")?;
                let object: S::Term = subject.clone().into();
                let map = match rdf.incoming_arcs(object.clone()) {
                    Result::Ok(m) => m,
                    Err(e) => bail!("Can't get outgoing arcs of node {subject}: {e}"),
                };
                writeln!(writer, "{}", rdf.qualify_term(&object))?;
                for pred in map.keys() {
                    writeln!(writer, "  <-{}-", rdf.qualify_iri(pred))?;
                    if let Some(subjs) = map.get(pred) {
                        for s in subjs {
                            writeln!(writer, "      {}", rdf.qualify_subject(s))?;
                        }
                    } else {
                        bail!("Not found values for {pred} in map")
                    }
                }
            }
            _ => {
                // Nothing to do
            }
        }
    }
    Ok(())
}

pub fn node_to_subject<S>(node: &ObjectValue, rdf: &S) -> Result<S::Subject>
where
    S: NeighsRDF,
{
    match node {
        ObjectValue::IriRef(iri_ref) => {
            let iri: S::IRI = match iri_ref {
                IriRef::Iri(iri_s) => iri_s.clone().into(),
                IriRef::Prefixed { prefix, local } => {
                    let iri_s = rdf.resolve_prefix_local(prefix, local)?;
                    iri_s.into()
                }
            };
            let term: S::Term = iri.into().into();
            match S::term_as_subject(&term) {
                Ok(subject) => Ok(subject),
                Err(_) => bail!("node_to_subject: Can't convert term {term} to subject"),
            }
        }
        ObjectValue::Literal(lit) => Err(anyhow!("Node must be an IRI, but found a literal {lit}")),
    }
}

fn cnv_predicates<S>(predicates: &Vec<String>, rdf: &S) -> Result<Vec<S::IRI>>
where
    S: NeighsRDF,
{
    let mut vs = Vec::new();
    for s in predicates {
        let iri_ref = parse_iri_ref(s)?;
        let iri_s = match iri_ref {
            IriRef::Prefixed { prefix, local } => {
                rdf.resolve_prefix_local(prefix.as_str(), local.as_str())?
            }
            IriRef::Iri(iri) => iri,
        };
        vs.push(iri_s.into())
    }
    Ok(vs)
}

fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    let iri = ShapeMapParser::parse_iri_ref(iri)?;
    Ok(iri)
}
