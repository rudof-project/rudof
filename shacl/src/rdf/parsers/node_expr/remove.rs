use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, required_nodes, node_expr_from_term};

pub(crate) fn remove<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_remove())?
        .map(|term| {
            let remove = node_expr_from_term(&term, rdf)?;
            let nodes = required_nodes::<RDF>(&neighs, rdf, "shnex:remove")?;
            Ok(ASTNodeExpr::Remove {
                remove: remove.into(),
                nodes: nodes.into(),
            })
        })
        .transpose()
}