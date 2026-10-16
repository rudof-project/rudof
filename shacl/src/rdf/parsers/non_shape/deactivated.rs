use crate::ast::ASTComponent;
use crate::rdf::parsers::node_expr::NodeExprsPropertyParser;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn deactivated<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    NodeExprsPropertyParser::new(ShaclVocab::sh_deactivated())
        .map(|ns| ns.into_iter().map(ASTComponent::Deactivated).collect())
}
