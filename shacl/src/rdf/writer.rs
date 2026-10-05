use crate::ir::IRSchema;
use crate::rdf::error::ShaclWriterError;
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use std::io::Write;

pub struct ShaclWriter<RDF: BuildRDF> {
    rdf: RDF,
}

impl<RDF: BuildRDF> ShaclWriter<RDF> {
    pub fn new() -> Self {
        Self { rdf: RDF::empty() }
    }

    pub fn register(&mut self, ir: &IRSchema) -> Result<(), ShaclWriterError> {
        self.rdf = ir.build_graph()?;
        Ok(())
    }

    pub fn serialize<W: Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<(), ShaclWriterError> {
        self.rdf
            .serialize(format, writer)
            .map_err(ShaclWriterError::from_rdf_err::<RDF>)
    }
}

impl<RDF: BuildRDF> Default for ShaclWriter<RDF> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTComponent, ASTPropertyShape, ASTShape};
    use crate::ir::error::IRError;
    use crate::rdf::ShaclParser;
    use rudof_iri::iri;
    use rudof_rdf::rdf_core::SHACLPath;
    use rudof_rdf::rdf_core::term::Object;
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};

    const SHAPES: &str = r#"
        @prefix :    <http://example.org/> .
        @prefix sh:  <http://www.w3.org/ns/shacl#> .

        :TestShape a sh:NodeShape ;
            sh:property :TestShape-property1 .

        :TestShape-property1
            sh:path ( :a :b ) ;
            sh:subsetOf [ sh:inversePath :c ] ;
            sh:nodeKind sh:Literal .
    "#;

    fn property_shape(shapes: &str) -> ASTPropertyShape {
        let graph = OxigraphInMemory::from_str(shapes, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let ast = ShaclParser::new(graph).parse().unwrap();
        let id = Object::iri(iri!("http://example.org/TestShape-property1"));
        match ast.get_shape(&id).unwrap() {
            ASTShape::PropertyShape(ps) => (**ps).clone(),
            _ => unreachable!(),
        }
    }

    /// Composite property paths used to abort the IR writer with `unimplemented!()`,
    /// both for `sh:path` and for the SHACL 1.2 `sh:subsetOf` parameter.
    #[test]
    fn composite_paths_survive_a_serialization_round_trip() {
        let before = property_shape(SHAPES);

        let graph = OxigraphInMemory::from_str(SHAPES, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let ast = ShaclParser::new(graph).parse().unwrap();
        let ir: crate::ir::IRSchema = ast.try_into().map_err(|e: IRError| e.to_string()).unwrap();

        let mut writer: ShaclWriter<OxigraphInMemory> = ShaclWriter::new();
        writer.register(&ir).unwrap();
        let mut serialized = Vec::new();
        writer.serialize(&RDFFormat::Turtle, &mut serialized).unwrap();

        let after = property_shape(&String::from_utf8(serialized).unwrap());

        let expected_path = SHACLPath::sequence(vec![
            SHACLPath::iri(iri!("http://example.org/a")),
            SHACLPath::iri(iri!("http://example.org/b")),
        ]);
        assert_eq!(*before.path(), expected_path);
        assert_eq!(*after.path(), expected_path);

        let subset_of = |ps: &ASTPropertyShape| {
            ps.components()
                .iter()
                .find_map(|c| match c {
                    ASTComponent::SubsetOf(path) => Some(path.clone()),
                    _ => None,
                })
                .expect("sh:subsetOf component lost")
        };
        let expected_subset_of = SHACLPath::inverse(SHACLPath::iri(iri!("http://example.org/c")));
        assert_eq!(subset_of(&before), expected_subset_of);
        assert_eq!(subset_of(&after), expected_subset_of);

        // sh:nodeKind used to be written with the sh:datatype predicate
        assert!(
            after
                .components()
                .iter()
                .any(|c| matches!(c, ASTComponent::NodeKind(_))),
            "sh:nodeKind lost on round trip: {:?}",
            after.components()
        );
    }
}
