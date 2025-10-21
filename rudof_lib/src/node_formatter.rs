use crate::{
    RudofError,
    node_info::{NodeInfo, NodeInfoOptions},
};
use srdf::NeighsRDF;
use std::io::Write;
use termtree::Tree;

// Format a single node's information to a writer
pub fn format_node_info<S: NeighsRDF, W: Write>(
    node_info: &NodeInfo<S>,
    rdf: &S,
    writer: &mut W,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    if options.show_outgoing && !node_info.outgoing.is_empty() {
        writeln!(writer, "Outgoing arcs")?;
        let mut outgoing_tree =
            Tree::new(node_info.subject_qualified.to_string()).with_glyphs(outgoing_glyphs());
        let mut preds: Vec<_> = node_info.outgoing.keys().collect();
        preds.sort();

        for pred in preds {
            let pred_str = rdf.qualify_iri(pred);
            if let Some(objs) = node_info.outgoing.get(pred) {
                for o in objs {
                    outgoing_tree.leaves.push(
                        Tree::new(format!("─ {} ─► {}", pred_str, rdf.qualify_term(o)))
                            .with_glyphs(outgoing_glyphs()),
                    );
                }
            }
        }
        writeln!(writer, "{}", outgoing_tree)?;
    }

    if options.show_incoming && !node_info.incoming.is_empty() {
        writeln!(writer, "Incoming arcs")?;
        let object: S::Term = node_info.subject.clone().into();
        let node_str = format!("{}\n▲", rdf.qualify_term(&object));
        let mut incoming_tree = Tree::new(node_str).with_glyphs(incoming_glyphs());

        let mut preds: Vec<_> = node_info.incoming.keys().collect();
        preds.sort();

        for pred in preds {
            let pred_str = rdf.qualify_iri(pred).to_string();
            if let Some(subjs) = node_info.incoming.get(pred) {
                for s in subjs {
                    incoming_tree.leaves.push(
                        Tree::new(format!("─ {} ── {}", pred_str, rdf.qualify_subject(s)))
                            .with_glyphs(incoming_glyphs()),
                    );
                }
            }
        }
        writeln!(writer, "{}", incoming_tree)?;
    }
    Ok(())
}

// Format multiple node information results
pub fn format_node_info_list<S: NeighsRDF, W: Write>(
    node_infos: &[NodeInfo<S>],
    rdf: &S,
    writer: &mut W,
    options: &NodeInfoOptions,
) -> Result<(), RudofError> {
    for node_info in node_infos {
        format_node_info(node_info, rdf, writer, options)?;
    }
    Ok(())
}

fn outgoing_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}

fn incoming_glyphs() -> termtree::GlyphPalette {
    termtree::GlyphPalette {
        middle_item: "├",
        last_item: "└",
        item_indent: "──",
        middle_skip: "│",
        last_skip: "",
        skip_indent: "   ",
    }
}
