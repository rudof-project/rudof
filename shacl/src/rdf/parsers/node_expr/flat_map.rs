use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, optional_expr, node_expr_from_term};

pub(crate) fn flat_map<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_flat_map())?
        .map(|term| {
            let flat_map = node_expr_from_term(&term, rdf)?;
            let nodes = optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_nodes(), rdf)?
                .unwrap_or_else(|| ASTNodeExpr::Var("focusNode".to_string()));
            Ok(ASTNodeExpr::FlatMap {
                flat_map: flat_map.into(),
                nodes: nodes.into(),
            })
        })
        .transpose()
}