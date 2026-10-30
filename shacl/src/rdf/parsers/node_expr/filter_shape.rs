use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, values};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn filter_shape<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
    _: usize,
    _: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    if values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_filter_shape()).is_some() {
        return Err(RDFError::DefaultError {
            msg: "shnex:filterShape is not supported yet".to_string(),
        });
    }
    Ok(None)
}
