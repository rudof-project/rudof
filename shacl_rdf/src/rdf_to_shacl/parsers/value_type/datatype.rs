use prefixmap::IriRef;
use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn datatype<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IrisPropertyParser::new(ShaclVocab::sh_datatype().clone()).map(|ns| {
        ns.into_iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri)))
            .collect()
    })
}
