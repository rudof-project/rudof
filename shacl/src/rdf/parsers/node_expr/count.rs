use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, node_expr_from_term, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn count<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_count())?
        .map(|term| {
            let count = node_expr_from_term(&term, rdf)?;
            Ok(ASTNodeExpr::Count(count.into()))
        })
        .transpose()
}
