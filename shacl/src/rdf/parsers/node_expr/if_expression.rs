use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, node_expr_from_term, optional_expr, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn if_expresion<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_if())?
        .map(|term| {
            let if_condition = node_expr_from_term(&term, rdf, depth, steps)?;
            let then = optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_then(), rdf, depth, steps)?;
            let else_expr = optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_else(), rdf, depth, steps)?;

            if then.is_none() && else_expr.is_none() {
                return Err(RDFError::DefaultError {
                    msg: "shnex:if requires at least one shnex:then or shnex:else".to_string(),
                });
            }

            Ok(ASTNodeExpr::IfExpression {
                if_condition: if_condition.into(),
                then: then.map(Box::new),
                else_expression: else_expr.map(Box::new),
            })
        })
        .transpose()
}
