use prefixmap::IriRef;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::IrisPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use shacl_ast::component::Component;

pub(crate) fn datatype<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component<RDF>>> {
    IrisPropertyParser::new(ShaclVocab::sh_datatype().clone()).map(|ns| {
        ns.into_iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri)))
            .collect()
    })
}
