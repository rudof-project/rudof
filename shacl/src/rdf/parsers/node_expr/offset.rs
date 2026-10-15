use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs, non_negative, required_nodes};

pub(crate) fn offset<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_offset())?
        .map(|term| {
            let offset = non_negative::<RDF>(&term, "shnex:offset")?;
            let nodes = required_nodes::<RDF>(&neighs, rdf, "shnex:offset")?;
            Ok(ASTNodeExpr::Offset {
                offset,
                nodes: nodes.into(),
            })
        })
        .transpose()
}