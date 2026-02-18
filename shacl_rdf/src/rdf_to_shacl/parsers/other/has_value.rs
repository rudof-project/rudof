use crate::rdf_to_shacl::parsers::utils::{parse_components_for_iri, term_to_value};
use rdf::rdf_core::parser::rdf_node_parser::constructors::TermParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn has_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(
        ShaclVocab::sh_has_value().clone(),
        TermParser::new().flat_map(cnv_has_value::<RDF>),
    )
}

fn cnv_has_value<RDF: Rdf>(term: RDF::Term) -> Result<Component, RDFError> {
    let value = term_to_value::<RDF>(&term, "parsing hasValue")?;
    Ok(Component::HasValue { value })
}
