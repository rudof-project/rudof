use crate::ast::ASTComponent;
use crate::rdf::parsers::utils::{parse_components_for_iri, terms_as_nodes};
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn or<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    parse_components_for_iri(
        ShaclVocab::sh_or().clone(),
        ListParser::new().flat_map(|ls| {
            let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
            Ok(ASTComponent::Or(shapes))
        }),
    )
}
