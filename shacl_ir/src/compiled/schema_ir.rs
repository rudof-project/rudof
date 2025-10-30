use super::compiled_shacl_error::CompiledShaclError;
use super::shape::ShapeIR;
use crate::dependency_graph::DependencyGraph;
use crate::shape_label_idx::ShapeLabelIdx;
use either::Either::{self, Left, Right};
use iri_s::IriS;
use prefixmap::PrefixMap;
use shacl_ast::Schema;
use shacl_rdf::ShaclParser;
use srdf::{RDFFormat, RDFNode, Rdf, ReaderMode, SRDFGraph};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::io;
use tracing::trace;

#[derive(Clone, Debug)]
pub struct SchemaIR {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shape_labels_map: HashMap<RDFNode, ShapeLabelIdx>,
    shapes: HashMap<ShapeLabelIdx, ShapeIR>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
    dependency_graph: DependencyGraph,
    shape_label_counter: usize,
}

impl SchemaIR {
    pub fn new(prefixmap: PrefixMap, base: Option<IriS>) -> SchemaIR {
        SchemaIR {
            shape_labels_map: HashMap::new(),
            shapes: HashMap::new(),
            prefixmap,
            base,
            dependency_graph: DependencyGraph::new(),
            shape_label_counter: 0,
        }
    }

    pub fn from_reader<R: io::Read>(
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR, Box<CompiledShaclError>> {
        let mut rdf = SRDFGraph::new();
        rdf.merge_from_reader(read, format, base, reader_mode)
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
        Self::from_reader(std::io::Cursor::new(&data), format, base, reader_mode)
    }

    /// Adds a shape index for the given `RDFNode` if it does not already exist.
    /// Returns  `Right(ShapeLabelIdx)` if a new index was created or `Left(ShapeLabelIdx)` with the existing one.
    pub fn add_shape_idx(
        &mut self,
        sref: RDFNode,
    ) -> Result<Either<ShapeLabelIdx, ShapeLabelIdx>, Box<CompiledShaclError>> {
        match self.shape_labels_map.entry(sref) {
            Entry::Occupied(entry) => Ok(Either::Left(*entry.get())),
            Entry::Vacant(entry) => {
                let label_idx = ShapeLabelIdx::new(self.shape_label_counter);
                self.shape_label_counter += 1;
                entry.insert(label_idx);
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
        self.shape_labels_map.iter().map(move |(node, label_idx)| {
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
        self.shape_labels_map.get(sref).map(|label_idx| {
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
                    let (_idx, deps) =
                        ShapeIR::compile(shape.to_owned(), schema, &idx, &mut schema_ir)?;
                    for (pos_neg, label_idx) in deps {
                        schema_ir.dependency_graph.add_edge(idx, label_idx, pos_neg);
                    }
                }
                Left(idx) => {
                    trace!("Shape {} already compiled with {}, skipping", rdf_node, idx);
                }
            }
        }
        Ok(schema_ir)
    }

    pub fn get_shape_from_idx(&self, shape_idx: &ShapeLabelIdx) -> Option<&ShapeIR> {
        self.shapes.get(shape_idx)
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
            writeln!(f, "{node} -> {shape}")?;
        }
        writeln!(f, "Dependency graph: {}", self.dependency_graph)?;
        Ok(())
    }
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
        let reader = Cursor::new(shacl_schema);
        let rdf_format = RDFFormat::Turtle;
        let base = None;

        let rdf =
            SRDFGraph::from_reader(reader, &rdf_format, base, &ReaderMode::default()).unwrap();

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
