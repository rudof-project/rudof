use prefixmap::IriRef;
use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn less_than_or_equals<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IrisPropertyParser::new(ShaclVocab::sh_less_than_or_equals().clone()).map(|ns| {
        ns.into_iter()
            .map(|n| {
                let iri = IriRef::iri(n);
                Component::LessThanOrEquals(iri)
            })
            .collect()
    })
}
