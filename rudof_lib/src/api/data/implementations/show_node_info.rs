use std::io;

use crate::{
    Result, Rudof,
    api::data::implementations::node_neighborhood,
    errors::DataError,
    formats::{IriNormalizationMode, NodeInspectionMode},
    types::{Data, NeighborArc},
};
use prefixmap::PrefixMap;
use rudof_rdf::rdf_core::{ArcDirection, Rdf, term::Object};

/// Writes the neighborhood of every node matched by `node` as a tree.
pub fn show_node_info<W: io::Write>(
    rudof: &mut Rudof,
    node: &str,
    predicates: Option<&[String]>,
    mode: Option<&NodeInspectionMode>,
    depth: Option<usize>,
    _show_hyperlinks: Option<bool>,
    show_colors: Option<bool>,
    iri_mode: IriNormalizationMode,
    writer: &mut W,
) -> Result<()> {
    let prefixmap = data_prefixmap(rudof, show_colors.unwrap_or(false))?;
    let neighborhood = node_neighborhood(rudof, node, predicates, mode, depth, iri_mode)?;

    let mut printer = TreePrinter::new(&prefixmap);
    for arc in neighborhood {
        printer.write_arc(&arc?, writer)?;
    }
    printer.finish(writer)
}

fn data_prefixmap(rudof: &Rudof, show_colors: bool) -> Result<PrefixMap> {
    let Some(Data::RDFData(rdf)) = rudof.data.as_ref() else {
        return Err(Box::new(DataError::NoRdfDataLoaded).into());
    };
    let prefixmap = rdf.prefixmap().unwrap_or_default();
    Ok(if show_colors {
        prefixmap.with_default_colors()
    } else {
        prefixmap.without_colors()
    })
}

const BRANCH_MIDDLE: &str = "├──";
const BRANCH_LAST: &str = "└──";
const SKIP_MIDDLE: &str = "│  ";
const SKIP_LAST: &str = "   ";

/// Renders a depth-first stream of [`NeighborArc`] as a tree, arc by arc.
struct TreePrinter<'a> {
    prefixmap: &'a PrefixMap,
    current: Option<(Object, ArcDirection)>,
    prefix: Vec<&'static str>,
}

impl<'a> TreePrinter<'a> {
    fn new(prefixmap: &'a PrefixMap) -> Self {
        Self {
            prefixmap,
            current: None,
            prefix: Vec::new(),
        }
    }

    fn write_arc<W: io::Write>(&mut self, arc: &NeighborArc, writer: &mut W) -> Result<()> {
        self.open_tree(arc, writer)?;

        self.prefix.truncate(arc.depth - 1);
        for skip in &self.prefix {
            write!(writer, "{skip}").map_err(io_error)?;
        }
        let branch = if arc.is_last { BRANCH_LAST } else { BRANCH_MIDDLE };
        writeln!(writer, "{branch}{}", self.arc_label(arc)).map_err(io_error)?;
        self.prefix.push(if arc.is_last { SKIP_LAST } else { SKIP_MIDDLE });

        Ok(())
    }

    fn arc_label(&self, arc: &NeighborArc) -> String {
        let predicate = self.prefixmap.qualify(&arc.predicate);
        let neighbor = arc.neighbor.show_qualified(self.prefixmap);
        match arc.direction {
            ArcDirection::Outgoing => format!("─ {predicate} ─► {neighbor}"),
            ArcDirection::Incoming => format!("─ {predicate} ── {neighbor}"),
        }
    }

    fn open_tree<W: io::Write>(&mut self, arc: &NeighborArc, writer: &mut W) -> Result<()> {
        let is_same_tree = self
            .current
            .as_ref()
            .is_some_and(|(root, direction)| *root == arc.root && *direction == arc.direction);
        if is_same_tree {
            return Ok(());
        }

        self.finish(writer)?;

        let root = arc.root.show_qualified(self.prefixmap);
        match arc.direction {
            ArcDirection::Outgoing => writeln!(writer, "Outgoing arcs\n{root}"),
            ArcDirection::Incoming => writeln!(writer, "Incoming arcs\n{root}\n▲"),
        }
        .map_err(io_error)?;

        self.current = Some((arc.root.clone(), arc.direction));
        self.prefix.clear();
        Ok(())
    }

    fn finish<W: io::Write>(&mut self, writer: &mut W) -> Result<()> {
        if self.current.is_some() {
            writeln!(writer).map_err(io_error)?;
        }
        Ok(())
    }
}

fn io_error(err: io::Error) -> Box<DataError> {
    Box::new(DataError::FailedIoOperation { error: err.to_string() })
}
