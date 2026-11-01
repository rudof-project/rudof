use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{list_with_budget, parse_components_for_iri, terms_as_nodes};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ParseBudget;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::sync::Arc;

pub(crate) fn and<RDF: FocusRDF>(budget: Arc<ParseBudget>) -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_and(),
        list_with_budget(budget).flat_map(|ls| {
            let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
            Ok(ASTComponent::And(shapes))
        }),
    )
}
