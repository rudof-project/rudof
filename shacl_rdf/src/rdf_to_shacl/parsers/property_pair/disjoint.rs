use prefixmap::IriRef;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn disjoint<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    IrisPropertyParser::new(ShaclVocab::sh_disjoint().clone()).map(|ns| {
        ns.into_iter()
            .map(|n| {
                let iri = IriRef::iri(n);
                Component::Disjoint(iri)
            })
            .collect()
    })
}
