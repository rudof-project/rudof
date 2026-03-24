use crate::ast::ASTComponent;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::LiteralsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn min_inclusive<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    LiteralsPropertyParser::new(ShaclVocab::sh_min_inclusive().clone())
        .map(|ns| {
            ns.into_iter()
                .map(ASTComponent::MinInclusive)
                .collect()
        })
}