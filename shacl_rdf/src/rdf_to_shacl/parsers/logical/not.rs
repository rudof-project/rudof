use crate::rdf_to_shacl::parsers::utils::parse_components_for_iri;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::TermParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn not<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(ShaclVocab::sh_not().clone(), TermParser::new().flat_map(cnv_not::<RDF>))
}

fn cnv_not<RDF: Rdf>(t: RDF::Term) -> Result<Component, RDFError> {
    let shape = RDF::term_as_object(&t).map_err(|_| RDFError::FailedTermToRDFNodeError { term: t.to_string() })?;
    Ok(Component::Not(shape))
}
