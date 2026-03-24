use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::parse_components_for_iri;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::TermParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn node<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_node().clone(),
        TermParser::new().flat_map(|t| {
            let shape = RDF::term_as_object(&t)
                .map_err(|_| RDFError::FailedTermToRDFNodeError {
                    term: t.to_string()
                })?;
            Ok(ASTComponent::Node(shape))
        })
    )
}