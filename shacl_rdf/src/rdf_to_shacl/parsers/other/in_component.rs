use crate::rdf_to_shacl::parsers::utils::{parse_components_for_iri, term_to_value};
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn in_component<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(
        ShaclVocab::sh_in().clone(),
        ListParser::new().flat_map(cnv_in_list::<RDF>),
    )
}

fn cnv_in_list<RDF: Rdf>(ls: Vec<RDF::Term>) -> Result<Component, RDFError> {
    let values = ls
        .iter()
        .flat_map(|t| term_to_value::<RDF>(t, "parsing in list"))
        .collect();
    Ok(Component::In { values })
}
