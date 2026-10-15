use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::Neighs;

pub(crate) fn empty<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    let res = if neighs.is_empty() {
        Some(ASTNodeExpr::Empty)
    } else { None };
    Ok(res)
}