use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::LiteralsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn max_inclusive<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    LiteralsPropertyParser::new(ShaclVocab::sh_max_inclusive().clone())
        .map(|ns| ns.into_iter().map(Component::MaxInclusive).collect())
}
