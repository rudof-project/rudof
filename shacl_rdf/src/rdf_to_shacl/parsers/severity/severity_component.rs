use prefixmap::IriRef;
use rdf::rdf_core::FocusRDF;
use rdf::rdf_core::parser::rdf_node_parser::constructors::SingleIriPropertyParser;
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use shacl_ast::ShaclVocab;
use shacl_ast::severity::Severity;

pub(crate) fn severity<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Severity> {
    SingleIriPropertyParser::new(ShaclVocab::sh_severity().clone()).map(|iri| match iri.as_str() {
        "http://www.w3.org/ns/shacl#Violation" => Severity::Violation,
        "http://www.w3.org/ns/shacl#Warning" => Severity::Warning,
        "http://www.w3.org/ns/shacl#Info" => Severity::Info,
        _ => Severity::Generic(IriRef::iri(iri)),
    })
}
