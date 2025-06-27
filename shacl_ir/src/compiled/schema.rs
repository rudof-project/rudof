use prefixmap::PrefixMap;
use shacl_rdf::ShaclParser;
use srdf::{RDFFormat, Rdf, ReaderMode, SRDFGraph};
use std::collections::HashMap;
use std::io;

use shacl_ast::Schema;

use super::compiled_shacl_error::CompiledShaclError;
use super::shape::CompiledShape;

#[derive(Debug)]
pub struct SchemaIR<S: Rdf> {
    // imports: Vec<IriS>,
    // entailments: Vec<IriS>,
    shapes: HashMap<S::Term, CompiledShape<S>>,
    prefixmap: PrefixMap,
    base: Option<S::IRI>,
}

impl<S: Rdf> SchemaIR<S> {
    pub fn new(
        shapes: HashMap<S::Term, CompiledShape<S>>,
        prefixmap: PrefixMap,
        base: Option<S::IRI>,
    ) -> SchemaIR<S> {
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
    ) -> Result<SchemaIR<SRDFGraph>, CompiledShaclError> {
        let mut rdf = SRDFGraph::new();
        rdf.merge_from_reader(read, format, base, reader_mode)
            .map_err(CompiledShaclError::RdfGraphError)?;
        let schema = ShaclParser::new(rdf)
            .parse()
            .map_err(CompiledShaclError::ShaclParserError)?;
        let schema_ir: SchemaIR<SRDFGraph> = schema.try_into()?;
        Ok(schema_ir)
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<SchemaIR<SRDFGraph>, CompiledShaclError> {
        Self::from_reader(std::io::Cursor::new(&data), format, base, reader_mode)
    }

    pub fn prefix_map(&self) -> PrefixMap {
        self.prefixmap.clone()
    }

    pub fn base(&self) -> &Option<S::IRI> {
        &self.base
    }

    pub fn iter(&self) -> impl Iterator<Item = (&S::Term, &CompiledShape<S>)> {
        self.shapes.iter()
    }

    /// Iterate over all shapes that have at least one target.
    pub fn iter_with_targets(&self) -> impl Iterator<Item = (&S::Term, &CompiledShape<S>)> {
        self.shapes
            .iter()
            .filter(|(_, shape)| !shape.targets().is_empty())
    }

    pub fn get_shape(&self, sref: &S::Term) -> Option<&CompiledShape<S>> {
        self.shapes.get(sref)
    }
}

impl<S: Rdf> TryFrom<Schema<S>> for SchemaIR<S> {
    type Error = CompiledShaclError;

    fn try_from(schema: Schema<S>) -> Result<Self, Self::Error> {
        let mut shapes = HashMap::default();

        for (rdf_node, shape) in schema.iter() {
            let term = rdf_node.clone().into();
            let shape = CompiledShape::compile(shape.to_owned(), &schema)?;
            shapes.insert(term, shape);
        }

        let prefixmap = schema.prefix_map();

        let base = schema.base().map(Into::into);

        Ok(SchemaIR::new(shapes, prefixmap, base))
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

    fn load_schema(shacl_schema: &str) -> SchemaIR<SRDFGraph> {
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
