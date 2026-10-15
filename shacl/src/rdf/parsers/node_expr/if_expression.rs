use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, optional_expr, node_expr_from_term};

pub(crate) fn if_expresion<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_if())?
        .map(|term| {
            let if_condition = node_expr_from_term(&term, rdf)?;
            let then = optional_expr::<RDF>(&neighs, &ShaclNodeExprVocab::shnex_then(), rdf)?;
            let else_expr = optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_else(), rdf)?;

            if then.is_none() && else_expr.is_none() {
                return Err(RDFError::DefaultError {
                    msg: "shnex:if requires at least one shnex:then or shnex:else".to_string(),
                })
            }

            Ok(ASTNodeExpr::IfExpression {
                if_condition: if_condition.into(),
                then: then.map(Box::new),
                else_expression: else_expr.map(Box::new),
            })
        })
        .transpose()
}