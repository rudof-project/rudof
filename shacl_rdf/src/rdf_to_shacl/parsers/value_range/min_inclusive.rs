use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::LiteralsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn min_inclusive<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component<RDF>>> {
    LiteralsPropertyParser::new(ShaclVocab::sh_min_inclusive().clone())
        .map(|ns| ns.into_iter().map(Component::MinInclusive).collect())
}
