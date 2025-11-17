use super::compiled_shacl_error::CompiledShaclError;
use super::shape::ShapeIR;
use crate::dependency_graph::{DependencyGraph, PosNeg};
use crate::shape_label_idx::ShapeLabelIdx;
use either::Either::{self, Left, Right};
use iri_s::IriS;
use prefixmap::PrefixMap;
use shacl_ast::Schema;
use shacl_rdf::ShaclParser;
use srdf::{RDFFormat, RDFNode, Rdf, ReaderMode, SRDFGraph};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io;
use tracing::{debug, info, trace};

#[derive(Clone, Debug)]
pub struct SchemaIR {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    labels_idx_map: HashMap<RDFNode, ShapeLabelIdx>,
    idx_labels_map: HashMap<ShapeLabelIdx, RDFNode>,
    shapes: HashMap<ShapeLabelIdx, ShapeIR>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
    dependency_graph: DependencyGraph,
    shape_label_counter: usize,
}

impl SchemaIR {
    pub fn new(prefixmap: PrefixMap, base: Option<IriS>) -> SchemaIR {
        SchemaIR {
            labels_idx_map: HashMap::new(),
            idx_labels_map: HashMap::new(),
            shapes: HashMap::new(),
            prefixmap,
            base,
            dependency_graph: DependencyGraph::new(),
            shape_label_counter: 0,
        }
    }

    pub fn from_reader<R: io::Read>(
        read: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR, Box<CompiledShaclError>> {
        let mut rdf = SRDFGraph::new();
        rdf.merge_from_reader(read, source_name, format, base, reader_mode)
            .map_err(|e| CompiledShaclError::RdfGraphError { err: Box::new(e) })?;
        let schema = ShaclParser::new(rdf)
            .parse()
            .map_err(|e| CompiledShaclError::ShaclParserError { err: Box::new(e) })?;
        let schema_ir: SchemaIR = schema.try_into()?;
        Ok(schema_ir)
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR, Box<CompiledShaclError>> {
        Self::from_reader(
            &mut std::io::Cursor::new(&data),
            "String",
            format,
            base,
            reader_mode,
        )
    }

    /// Adds a shape index for the given `RDFNode` if it does not already exist.
    /// Returns  `Right(ShapeLabelIdx)` if a new index was created or `Left(ShapeLabelIdx)` with the existing one.
    pub fn add_shape_idx(
        &mut self,
        sref: RDFNode,
    ) -> Result<Either<ShapeLabelIdx, ShapeLabelIdx>, Box<CompiledShaclError>> {
        match self.labels_idx_map.entry(sref.clone()) {
            Entry::Occupied(entry) => Ok(Either::Left(*entry.get())),
            Entry::Vacant(entry) => {
                let label_idx = ShapeLabelIdx::new(self.shape_label_counter);
                self.shape_label_counter += 1;
                entry.insert(label_idx);
                self.idx_labels_map.insert(label_idx, sref);
                Ok(Either::Right(label_idx))
            }
        }
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<IriS> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RDFNode, &ShapeIR)> {
        self.labels_idx_map.iter().map(move |(node, label_idx)| {
            let shape = self
              .shapes
              .get(label_idx)
              .unwrap_or_else(|| panic!("Internal error: Shape label index {label_idx} for node {node} not found in shapes map: {:?}",self.shapes));
            (node, shape)
        })
    }

    /// Iterate over all shapes that have at least one target.
    pub fn iter_with_targets(&self) -> impl Iterator<Item = (&RDFNode, &ShapeIR)> {
        self.iter().filter(|(_, shape)| !shape.targets().is_empty())
    }

    pub fn get_shape(&self, sref: &RDFNode) -> Option<&ShapeIR> {
        self.labels_idx_map.get(sref).map(|label_idx| {
            self.shapes
                .get(label_idx)
                .unwrap_or_else(|| panic!("Internal error: SHACL/SchemaIR. Shape label index {label_idx} corresponding to {sref} not found in shapes map {:?}", self.shapes))
        })
    }

    pub fn add_shape(
        &mut self,
        idx: ShapeLabelIdx,
        shape: ShapeIR,
    ) -> Result<ShapeLabelIdx, Box<CompiledShaclError>> {
        self.shapes.insert(idx, shape);
        Ok(idx)
    }

    pub fn compile<RDF: Rdf>(schema: &Schema<RDF>) -> Result<SchemaIR, Box<CompiledShaclError>> {
        trace!("Compiling SHACL schema");
        let mut schema_ir = SchemaIR::new(schema.prefix_map(), schema.base());
        for (rdf_node, shape) in schema.iter() {
            match schema_ir.add_shape_idx(rdf_node.clone())? {
                Right(idx) => {
                    trace!("Compiling shape {} with new index {}", rdf_node, idx);
                    let _idx = ShapeIR::compile(shape.to_owned(), schema, &idx, &mut schema_ir)?;
                }
                Left(idx) => {
                    trace!("Shape {} already compiled with {}, skipping", rdf_node, idx);
                }
            }
        }
        schema_ir.build_dependency_graph();
        if schema_ir.dependency_graph.has_cycles() {
            info!(
                "Warning: The dependency graph has cycles. This is known as a recursive schema and the SHACL semantics for these schemas is implementation dependent"
            );
            info!(
                "More information about recursive schemas can be found at https://www.w3.org/TR/shacl/#shapes-recursion"
            );
            debug!(
                "Dependency graph with cycles: {}",
                schema_ir.dependency_graph
            );
        }
        if schema_ir.dependency_graph.has_neg_cycle() {
            info!(
                "Warning: The dependency graph has negative cycles. This may lead to unexpected behavior in SHACL validation due to non-stratified negation"
            );
            let neg_cycles_str: String = schema_ir
                .dependency_graph
                .neg_cycles()
                .iter()
                .map(|cycles| {
                    cycles
                        .iter()
                        .map(show_cycle)
                        .collect::<Vec<_>>()
                        .join("\n ")
                })
                .collect::<Vec<_>>()
                .join("\n---\n");
            debug!("Negative cycles: {}", neg_cycles_str);
        }
        Ok(schema_ir)
    }

    pub fn get_shape_from_idx(&self, shape_idx: &ShapeLabelIdx) -> Option<&ShapeIR> {
        self.shapes.get(shape_idx)
    }

    pub(crate) fn build_dependency_graph(&mut self) {
        let mut dg = DependencyGraph::new();
        let mut visited = HashSet::new();
        for (shape_idx, shape_ir) in self.shapes.iter() {
            // Add edges, we start by positive edges, but the direction can change when there is some negation
            shape_ir.add_edges(*shape_idx, &mut dg, PosNeg::pos(), self, &mut visited);
        }
        self.dependency_graph = dg;
    }
}

impl<RDF: Rdf> TryFrom<Schema<RDF>> for SchemaIR {
    type Error = Box<CompiledShaclError>;

    fn try_from(schema: Schema<RDF>) -> Result<Self, Self::Error> {
        Self::compile(&schema)
    }
}

impl<RDF: Rdf> TryFrom<&Schema<RDF>> for SchemaIR {
    type Error = Box<CompiledShaclError>;

    fn try_from(schema: &Schema<RDF>) -> Result<Self, Self::Error> {
        Self::compile(schema)
    }
}

impl Display for SchemaIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SHACL shapes graph IR",)?;
        for (node, shape) in self.shapes.iter() {
            let node_label = self
                .idx_labels_map
                .get(node)
                .map(|n| n.show_qualified(&self.prefixmap))
                .unwrap_or("?".to_string());
            writeln!(f, "{node_label}[{node}] -> {shape}")?;
        }
        writeln!(f, "Dependency graph: {}", self.dependency_graph)?;
        Ok(())
    }
}

fn show_cycle(cycle: &(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)) -> String {
    let (from, to, shapes) = cycle;
    let shapes_str = shapes
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" -> ");
    format!("Cycle from {} to {}: {}", from, to, shapes_str)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use srdf::RDFFormat;
    use srdf::ReaderMode;
    use srdf::SRDFGraph;

    use shacl_rdf::ShaclParser;

    use super::SchemaIR;

    const SCHEMA: &str = r#"
        @prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix ex: <http://example.org/> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

        ex:PersonShape a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] .

        ex:PersonShape2 a sh:NodeShape ;
            sh:targetClass ex:Person ;
            sh:property [
                sh:path ex:name ;
                sh:datatype xsd:string ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] ;
            sh:property [
                sh:path ex:age ;
                sh:datatype xsd:integer ;
                sh:minCount 1 ;
                sh:maxCount 1 ;
            ] .
    "#;

    fn load_schema(shacl_schema: &str) -> SchemaIR {
        let mut reader = Cursor::new(shacl_schema);
        let rdf_format = RDFFormat::Turtle;
        let base = None;

        let rdf = SRDFGraph::from_reader(
            &mut reader,
            "String",
            &rdf_format,
            base,
            &ReaderMode::default(),
        )
        .unwrap();

        ShaclParser::new(rdf).parse().unwrap().try_into().unwrap()
    }

    #[test]
    fn test_schema_iterator() {
        let schema = load_schema(SCHEMA);
        let actual = schema.iter_with_targets().count();
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
