use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, node_expr_from_term, required_nodes, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_bool;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn order_by<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_order_by())?
        .map(|term| {
            let desc = desc::<RDF>(neighs)?;
            let order_by = node_expr_from_term(&term, rdf)?;
            let nodes = required_nodes::<RDF>(neighs, rdf, "shnex:orderBy")?;
            Ok(ASTNodeExpr::OrderBy {
                desc,
                order_by: order_by.into(),
                nodes: nodes.into(),
            })
        })
        .transpose()
}

fn desc<RDF: FocusRDF>(neighs: &Neighs<RDF>) -> Result<Option<bool>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_desc())?
        .map(|term| term_to_bool::<RDF>(&term))
        .transpose()
}
