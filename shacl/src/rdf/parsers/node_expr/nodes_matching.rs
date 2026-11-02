use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_iri_or_blanknode;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn nodes_matching<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    _: &mut RDF,
    _: &RDF::Term,
    _: usize,
    _: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_nodes_matching())?
        .map(|term| {
            let iri = term_to_iri_or_blanknode::<RDF>(&term)?;
            Ok(ASTNodeExpr::NodesMatching(iri))
        })
        .transpose()
}
