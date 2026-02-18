use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::IntegersPropertyParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn max_length<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IntegersPropertyParser::new(ShaclVocab::sh_max_length().clone())
        .map(|ns| ns.into_iter().map(Component::MaxLength).collect())
}
