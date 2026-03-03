use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ObjectsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn property<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Object>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_property().clone()).map(|ps| ps.into_iter().collect())
}
