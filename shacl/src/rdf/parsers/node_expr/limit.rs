use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, non_negative, required_nodes, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn limit<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_limit())?
        .map(|term| {
            let limit = non_negative::<RDF>(&term, "shnex:limit")?;
            let nodes = required_nodes::<RDF>(neighs, rdf, "shnex:limit", depth, steps)?;
            Ok(ASTNodeExpr::Limit {
                limit,
                nodes: nodes.into(),
            })
        })
        .transpose()
}
