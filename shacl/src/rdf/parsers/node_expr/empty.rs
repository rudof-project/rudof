use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::Neighs;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn empty<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
    _: usize,
    _: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    let res = neighs.is_empty().then_some(ASTNodeExpr::Empty);
    Ok(res)
}
