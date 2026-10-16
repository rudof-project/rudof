use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, non_negative, required_nodes, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn offset<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_offset())?
        .map(|term| {
            let offset = non_negative::<RDF>(&term, "shnex:offset")?;
            let nodes = required_nodes::<RDF>(neighs, rdf, "shnex:offset")?;
            Ok(ASTNodeExpr::Offset {
                offset,
                nodes: nodes.into(),
            })
        })
        .transpose()
}
