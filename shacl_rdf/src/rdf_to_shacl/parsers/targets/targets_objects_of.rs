use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::target::Target;

pub(crate) fn targets_objects_of<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>> {
    IrisPropertyParser::new(ShaclVocab::sh_target_objects_of().clone()).flat_map(move |ts| {
        let result = ts.into_iter().map(|t| Target::ObjectsOf(t.into())).collect();
        Ok(result)
    })
}
