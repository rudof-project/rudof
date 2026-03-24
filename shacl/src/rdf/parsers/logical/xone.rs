use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{parse_components_for_iri, terms_as_nodes};
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn xone<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_xone().clone(),
        ListParser::new().flat_map(|ls| {
            let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?; // ls.into_iter().map(|t| t.try_into()).collect();
            Ok(ASTComponent::Xone(shapes))
        })
    )
}