use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_string;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs};

pub(crate) fn var<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_var())?
        .map(|term| {
            let var = term_to_string::<RDF>(&term)?;
            if var.is_empty() {
                Err(RDFError::DefaultError {
                    msg: "shnex:var must be a non-empty string".to_string(),
                })
            } else {
                Ok(ASTNodeExpr::Var(var))
            }
        })
        .transpose()
}