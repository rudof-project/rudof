use crate::types::Target;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn targets_objects_of<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>> {
    IrisPropertyParser::new(ShaclVocab::sh_target_objects_of().clone()).flat_map(move |ts| {
        let result = ts.into_iter().map(Target::ObjectsOf).collect();
        Ok(result)
    })
}
