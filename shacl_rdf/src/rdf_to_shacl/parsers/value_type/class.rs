use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::ObjectsPropertyParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn class<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_class().clone()).map(|ns| ns.into_iter().map(Component::Class).collect())
}
