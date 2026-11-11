use crate::ast::ASTTarget;
use crate::types::Target;
use rudof_rdf::rdf_core::FocusRDF;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ObjectsPropertyParser;
use rudof_rdf::rdf_core::parser::rdf_node_parser::{ParserExt, RDFNodeParse};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;

pub(crate) fn targets_where<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<ASTTarget>> {
    ObjectsPropertyParser::new(ShaclVocab::sh_target_where()).flat_map(|ts| {
        let result = ts.into_iter().map(target_where).collect();
        Ok(result)
    })
}

/// Each value of `sh:targetWhere` is a well-formed shape, so a literal is not one.
fn target_where(object: Object) -> ASTTarget {
    match object {
        shape @ (Object::Iri(_) | Object::BlankNode(_)) => Target::Where(shape),
        other => Target::WrongWhere(other),
    }
}
