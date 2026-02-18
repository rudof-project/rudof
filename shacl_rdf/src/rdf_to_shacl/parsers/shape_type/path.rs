use rdf::rdf_core::parser::rdf_node_parser::constructors::{ShaclPathParser, SingleValuePropertyParser};
use rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rdf::rdf_core::{FocusRDF, SHACLPath};
use shacl_ast::ShaclVocab;

/// Parses the property value of the focus node as a SHACL path
pub(crate) fn path<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath> {
    SingleValuePropertyParser::new(ShaclVocab::sh_path().clone()).then(ShaclPathParser::new)
}
