use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    ParseBudget, ShaclPathParser, SingleValuePropertyParser,
};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, SHACLPath};
use std::sync::Arc;

pub(crate) fn path<RDF: FocusRDF>(budget: Arc<ParseBudget>) -> impl RDFNodeParse<RDF, Output = SHACLPath> {
    SingleValuePropertyParser::new(ShaclVocab::sh_path())
        .then(move |term| ShaclPathParser::with_budget(term, budget.clone()))
}
