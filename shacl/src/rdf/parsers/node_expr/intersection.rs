use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, expr_list, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn intersection<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_intersection())?
        .map(|term| {
            let intersection = ASTNodeExpr::Intersection(expr_list(&term, rdf)?);
            Ok(intersection)
        })
        .transpose()
}
