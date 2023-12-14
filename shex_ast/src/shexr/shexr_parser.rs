use std::collections::HashSet;

use super::shexr_error::{Nodes, ShExRError};
use crate::{Node, Schema};
use iri_s::IriS;
use srdf::srdf_parser::RDFParseError;
use srdf::SRDF;
use std::marker::PhantomData;

pub struct ShExRParser<RDF>
where
    RDF: SRDF,
{
    rdf: RDF,
}

/*impl<S> RDFParser<S> for ShExRParser
where
    S: SRDF,
{
    type Output = Schema;

    fn parse(&mut self, rdf: S) -> Result<Self::Output, <S>::Err> {
        todo!()
    }
}*/

impl<RDF> ShExRParser<RDF>
where
    RDF: SRDF,
{
    pub fn new(rdf: RDF) -> ShExRParser<RDF> {
        ShExRParser { rdf }
    }

    pub fn sx_schema() -> RDF::Term {
        let iri_s = IriS::new_unchecked("http://www.w3.org/ns/shex#Schema");
        let iri = RDF::iri_s2iri(&iri_s);
        let term = RDF::iri_as_term(iri);
        term
    }

    pub fn rdf_type() -> RDF::IRI {
        let rdf_type = IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
        let iri = RDF::iri_s2iri(&rdf_type);
        iri
    }

    pub fn parse(&self) -> Result<Schema, ShExRError> {
        let schema_nodes = self
            .rdf
            .subjects_with_predicate_object(&Self::rdf_type(), &Self::sx_schema())
            .map_err(|e| ShExRError::SRDFError { err: e.to_string() })?;
        match schema_nodes.len() {
            0 => Err(ShExRError::NoSchemaNodes),
            1 => {
                let node = schema_nodes.into_iter().next().unwrap();
                self.parse_schema(node)
            }
            _ => {
                let nodes = Nodes::new(Self::subjects_to_nodes(schema_nodes));
                Err(ShExRError::MoreThanOneSchema { nodes })
            }
        }
    }

    fn subjects_to_nodes(subjects: HashSet<RDF::Subject>) -> Vec<Node> {
        subjects
            .into_iter()
            .map(|s| Self::subject_to_node(s))
            .collect()
    }

    fn subject_to_node(subject: RDF::Subject) -> Node {
        if let Some(iri) = RDF::subject_as_iri(&subject) {
            todo!()
        } else if let Some(bnode) = RDF::subject_as_bnode(&subject) {
            todo!()
        } else {
            panic!("Subject is neither iri not bnode?")
        }
    }

    pub fn parse_schema(&self, node: RDF::Subject) -> Result<Schema, ShExRError> {
        Ok(Schema::new())
    }
}
