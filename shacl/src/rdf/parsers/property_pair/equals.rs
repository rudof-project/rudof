use crate::ast::ASTComponent;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn equals<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    IrisPropertyParser::new(ShaclVocab::sh_equals().clone())
        .map(|ns| {
            ns.into_iter()
                .map(|n| {
                    let iri = IriRef::iri(n);
                    ASTComponent::Equals(iri)
                })
                .collect()
        })
}