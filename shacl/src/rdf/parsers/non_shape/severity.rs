use crate::types::Severity;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::SingleIriPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::FocusRDF;

pub(crate) fn severity<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Severity> {
    SingleIriPropertyParser::new(ShaclVocab::sh_severity().clone())
        .map(|iri| match iri.as_str() {
            ShaclVocab::SH_VIOLATION => Severity::Violation,
            ShaclVocab::SH_WARNING => Severity::Warning,
            ShaclVocab::SH_INFO => Severity::Info,
            ShaclVocab::SH_DEBUG => Severity::Debug,
            ShaclVocab::SH_TRACE => Severity::Trace,
            _ => Severity::Generic(iri)
        })
}