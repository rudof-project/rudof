use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::BoolsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn unique_lang<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    BoolsPropertyParser::new(ShaclVocab::sh_unique_lang().clone())
        .map(|ns| ns.into_iter().map(Component::UniqueLang).collect())
}
