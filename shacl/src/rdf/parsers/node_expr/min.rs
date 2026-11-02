use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, node_expr_from_term, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn min<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_min())?
        .map(|term| {
            let min = node_expr_from_term(&term, rdf, depth, steps)?;
            Ok(ASTNodeExpr::Min(min.into()))
        })
        .transpose()
}
