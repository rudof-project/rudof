use crate::rdf_to_shacl::parsers::utils::{parse_components_for_iri, terms_as_nodes};
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn or<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(
        ShaclVocab::sh_or().clone(),
        ListParser::new().flat_map(cnv_or_list::<RDF>),
    )
}

fn cnv_or_list<RDF: Rdf>(ls: Vec<RDF::Term>) -> Result<Component, RDFError> {
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
    Ok(Component::Or { shapes })
}
