use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, values};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn match_all<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    if values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_match_all()).is_some() {
        todo!("shnex:matchAll is not yet implemented");
    }
    Ok(None)
}
