use crate::ast::ASTComponent;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IntegersPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn max_length<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    IntegersPropertyParser::new(ShaclVocab::sh_max_length().clone())
        .map(|ns| ns.into_iter().map(ASTComponent::MaxLength).collect())
}
