use iri_s::IriS;
use prefixmap::PrefixMap;
use shacl_rdf::ShaclParser;
use srdf::{RDFFormat, RDFNode, Rdf, ReaderMode, SRDFGraph};
use std::collections::HashMap;
use std::io;

use shacl_ast::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug)]
pub struct SchemaIR {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<RDFNode, CompiledShape>,
    prefixmap: PrefixMap,
    base: Option<IriS>,
}

impl SchemaIR {
    pub fn new(
        shapes: HashMap<RDFNode, CompiledShape>,
        prefixmap: PrefixMap,
        base: Option<IriS>,
    ) -> SchemaIR {
        SchemaIR {
            shapes,
            prefixmap,
            base,
        }
    }

    pub fn from_reader<R: io::Read>(
        read: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR, CompiledShaclError> {
        let mut rdf = SRDFGraph::new();
        rdf.merge_from_reader(read, format, base, reader_mode)
            .map_err(CompiledShaclError::RdfGraphError)?;
        let schema = ShaclParser::new(rdf)
            .parse()
            .map_err(CompiledShaclError::ShaclParserError)?;
        let schema_ir: SchemaIR = schema.try_into()?;
        Ok(schema_ir)
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR, CompiledShaclError> {
        Self::from_reader(std::io::Cursor::new(&data), format, base, reader_mode)
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<IriS> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RDFNode, &CompiledShape)> {
        self.shapes.iter()
    }

    /// Iterate over all shapes that have at least one target.
    pub fn iter_with_targets(&self) -> impl Iterator<Item = (&RDFNode, &CompiledShape)> {
        self.shapes
            .iter()
            .filter(|(_, shape)| !shape.targets().is_empty())
    }

    pub fn get_shape(&self, sref: &RDFNode) -> Option<&CompiledShape> {
        self.shapes.get(sref)
    }

    pub fn compile<RDF: Rdf>(schema: &Schema<RDF>) -> Result<SchemaIR, CompiledShaclError> {
        let mut shapes = HashMap::default();

        for (rdf_node, shape) in schema.iter() {
            let term = rdf_node.clone();
            let shape = CompiledShape::compile(shape.to_owned(), schema)?;
            shapes.insert(term, shape);
        }

        let prefixmap = schema.prefix_map();

        let base = schema.base();

        Ok(SchemaIR::new(shapes, prefixmap, base))
    }
}

impl<RDF: Rdf> TryFrom<Schema<RDF>> for SchemaIR {
    type Error = CompiledShaclError;

    fn try_from(schema: Schema<RDF>) -> Result<Self, Self::Error> {
        Self::compile(&schema)
    }
}

impl<RDF: Rdf> TryFrom<&Schema<RDF>> for SchemaIR {
    type Error = CompiledShaclError;

    fn try_from(schema: &Schema<RDF>) -> Result<Self, Self::Error> {
        Self::compile(schema)
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
