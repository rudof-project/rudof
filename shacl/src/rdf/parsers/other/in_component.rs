use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{list_with_budget, parse_components_for_iri, term_to_value};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ParseBudget;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::sync::Arc;

pub(crate) fn in_component<RDF: FocusRDF>(
    budget: Arc<ParseBudget>,
) -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_in(),
        list_with_budget(budget).flat_map(|ls| {
            let values = ls
                .iter()
                .flat_map(|t| term_to_value::<RDF>(t, "parsing in list"))
                .collect();
            Ok(ASTComponent::In(values))
        }),
    )
}
