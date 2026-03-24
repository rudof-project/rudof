use crate::types::Target;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn targets_subjects_of<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>> {
    IrisPropertyParser::new(ShaclVocab::sh_target_subjects_of().clone())
        .flat_map(move |ts| {
            let result = ts.into_iter()
                .map(|t| Target::SubjectsOf(t.into()))
                .collect();
            Ok(result)
        })
}