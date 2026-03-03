use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ObjectsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::target::Target;

pub(crate) fn targets_node<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_target_node().clone()).flat_map(|ts| {
        let result = ts.into_iter().map(Target::Node).collect();
        Ok(result)
    })
}
