use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{ShaclPathParser, SingleValuePropertyParser};
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, SHACLPath};

/// Parses the property value of the focus node as a SHACL path
pub(crate) fn path<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath> {
    SingleValuePropertyParser::new(ShaclVocab::sh_path().clone()).then(ShaclPathParser::new)
}
