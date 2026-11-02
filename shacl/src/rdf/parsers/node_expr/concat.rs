use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, expr_list, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn concat<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_concat())?
        .map(|term| {
            let concat = ASTNodeExpr::Concat(expr_list(&term, rdf, depth, steps)?);
            Ok(concat)
        })
        .transpose()
}
