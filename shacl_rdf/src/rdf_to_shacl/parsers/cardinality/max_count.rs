use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IntegersPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn max_count<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IntegersPropertyParser::new(ShaclVocab::sh_max_count().clone())
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
}
