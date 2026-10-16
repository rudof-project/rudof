use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, values};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn filter_shape<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    if values::<RDF>(neighs, &ShaclNodeExprVocab::shnex_filter_shape()).is_some() {
        todo!("shnex:filterShape is not supported yet");
    }
    Ok(None)
}
