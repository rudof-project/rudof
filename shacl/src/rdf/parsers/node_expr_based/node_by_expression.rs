use crate::ast::ASTComponent;
use crate::rdf::parsers::node_expr::NodeExprsPropertyParser;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ParseBudget;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::sync::Arc;

pub(crate) fn node_by_expression<RDF: FocusRDF>(
    budget: Arc<ParseBudget>,
) -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    NodeExprsPropertyParser::new(ShaclVocab::sh_node_by_expression(), budget).map(|ns| {
        ns.into_iter()
            .map(|(_, expr)| ASTComponent::NodeByExpression(expr))
            .collect()
    })
}
