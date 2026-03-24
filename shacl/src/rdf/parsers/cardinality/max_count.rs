use crate::ast::ASTComponent;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IntegersPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn max_count<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    IntegersPropertyParser::new(ShaclVocab::sh_max_count().clone())
        .map(|ns| ns.into_iter().map(ASTComponent::MaxCount).collect())
}