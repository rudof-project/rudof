use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_string;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn var<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
    _: usize,
    _: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
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
