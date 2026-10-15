use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{values, Neighs};

pub(crate) fn match_all<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_match_all())
        .map(|_terms| todo!("shnex:matchAll is not yet implemented"));
    Ok(None)
}