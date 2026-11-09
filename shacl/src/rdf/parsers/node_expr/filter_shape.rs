use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, required_nodes, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_iri_or_blanknode;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn filter_shape<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_filter_shape())?
        .map(|term| {
            let filter_shape = term_to_iri_or_blanknode::<RDF>(&term)?;
            let nodes = required_nodes::<RDF>(neighs, rdf, ShaclNodeExprVocab::SHNEX_FILTER_SHAPE, depth, steps)?;
            Ok(ASTNodeExpr::Filter {
                filter_shape,
                nodes: nodes.into(),
            })
        })
        .transpose()
}
