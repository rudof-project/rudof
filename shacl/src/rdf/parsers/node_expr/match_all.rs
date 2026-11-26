use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, optional_expr, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_iri_or_blanknode;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn match_all<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_match_all())?
        .map(|term| {
            let match_all = term_to_iri_or_blanknode::<RDF>(&term)?;
            let nodes = optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_nodes(), rdf, depth, steps)?
                .unwrap_or_else(|| ASTNodeExpr::Var("focusNode".to_string()));
            Ok(ASTNodeExpr::MatchAll {
                match_all,
                nodes: nodes.into(),
            })
        })
        .transpose()
}
