use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, non_negative, required_nodes};

pub(crate) fn limit<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_limit())?
        .map(|term| {
            let limit = non_negative::<RDF>(&term, "shnex:limit")?;
            let nodes = required_nodes::<RDF>(neighs, rdf, "shnex:limit")?;
            Ok(ASTNodeExpr::Limit {
                limit: limit.into(),
                nodes: nodes.into(),
            })
        })
        .transpose()
}