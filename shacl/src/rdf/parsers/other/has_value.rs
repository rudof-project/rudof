use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{parse_components_for_iri, term_to_value};
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::TermParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn has_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_has_value().clone(),
        TermParser::new().flat_map(|term| {
            let value = term_to_value::<RDF>(&term, "parsing hasValue")?;
            Ok(ASTComponent::HasValue(value))
        })
    )
}