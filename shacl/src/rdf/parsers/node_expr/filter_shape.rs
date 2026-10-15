use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{values, Neighs};

pub(crate) fn filter_shape<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_filter_shape())
        .map(|_terms| todo!("shnex:filterShape is not supported yet"));
    Ok(None)
}