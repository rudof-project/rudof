use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{parse_components_for_iri, term_to_value};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn in_component<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_in().clone(),
        ListParser::new().flat_map(|ls| {
            let values = ls
                .iter()
                .flat_map(|t| term_to_value::<RDF>(t, "parsing in list"))
                .collect();
            Ok(ASTComponent::In(values))
        }),
    )
}
