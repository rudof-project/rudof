use crate::ast::{ASTNodeExpr, ASTTarget};
use crate::types::Target;
use prefixmap::IriRef;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ObjectsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn targets_node<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTTarget>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_target_node()).flat_map(|ts| {
        let result = ts.into_iter().map(target_node).collect();
        Ok(result)
    })
}

// An IRI or a literal value for `sh:targetNode` is a well-formed node expression.
// TODO - A blank node would have to be parsed as a nested node expression
// TODO - A triple term as a triple term expression
// TODO - Neither has a parser yet
// TODO - Both are kept as malformed targets
fn target_node(object: Object) -> ASTTarget {
    match object {
        Object::Iri(iri) => Target::Node(ASTNodeExpr::Iri(IriRef::iri(iri))),
        Object::Literal(lit) => Target::Node(ASTNodeExpr::Literal(lit)),
        other => Target::WrongNode(other),
    }
}
