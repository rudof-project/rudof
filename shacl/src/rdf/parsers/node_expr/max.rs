use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, node_expr_from_term};

pub(crate) fn max<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_max())?
        .map(|term| {
            let max = node_expr_from_term(&term, rdf)?;
            Ok(ASTNodeExpr::Max(max.into()))
        })
        .transpose()
}