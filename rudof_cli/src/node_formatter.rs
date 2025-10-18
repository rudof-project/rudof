// Formatting logic for CLI output
use anyhow::*;
use srdf::NeighsRDF;
use std::io::Write;

use rudof_lib::node_info::NodeInfo;

// Format a single node's information to a writer
pub fn format_node_info<S: NeighsRDF, W: Write>(
    node_info: &NodeInfo<S>,
    rdf: &S,
    writer: &mut W,
) -> Result<()> {
    writeln!(writer, "Information about {}", node_info.subject_qualified)?;

    // Show outgoing arcs
    if !node_info.outgoing.is_empty() {
        writeln!(writer, "Outgoing arcs")?;
        writeln!(writer, "{}", node_info.subject_qualified)?;

        let mut preds: Vec<_> = node_info.outgoing.keys().collect();
        preds.sort();

        for pred in preds {
            writeln!(writer, " -{}-> ", rdf.qualify_iri(pred))?;
            if let Some(objs) = node_info.outgoing.get(pred) {
                for o in objs {
                    writeln!(writer, "      {}", rdf.qualify_term(o))?;
                }
            } else {
                bail!("Not found values for {pred} in map")
            }
        }
    }

    // Show incoming arcs
    if !node_info.incoming.is_empty() {
        writeln!(writer, "Incoming arcs")?;
        let object: S::Term = node_info.subject.clone().into();
        writeln!(writer, "{}", rdf.qualify_term(&object))?;

        let mut preds: Vec<_> = node_info.incoming.keys().collect();
        preds.sort();

        for pred in preds {
            writeln!(writer, "  <-{}-", rdf.qualify_iri(pred))?;
            if let Some(subjs) = node_info.incoming.get(pred) {
                for s in subjs {
                    writeln!(writer, "      {}", rdf.qualify_subject(s))?;
                }
            } else {
                bail!("Not found values for {pred} in map")
            }
        }
    }

    Ok(())
}

// Format multiple node information results
pub fn format_node_info_list<S: NeighsRDF, W: Write>(
    node_infos: &[NodeInfo<S>],
    rdf: &S,
    writer: &mut W,
) -> Result<()> {
    for node_info in node_infos {
        format_node_info(node_info, rdf, writer)?;
    }
    Ok(())
}
