use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{values, Neighs};

pub(crate) fn find_first<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_find_first())
        .map(|_terms| todo!("shnex:findFirst is not implemented yet"));
    Ok(None)
}