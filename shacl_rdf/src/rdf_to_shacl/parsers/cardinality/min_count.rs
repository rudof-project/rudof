use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IntegersPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn min_count<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IntegersPropertyParser::new(ShaclVocab::sh_min_count().clone())
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
}
