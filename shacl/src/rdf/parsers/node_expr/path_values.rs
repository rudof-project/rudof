use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, optional_expr, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::parse_path;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn path_values<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_path_values())?
        .map(|term| {
            let fnode = optional_expr(neighs, &ShaclNodeExprVocab::shnex_focus_node(), rdf, depth, steps)?;
            // The path shares the expression's budget, so one bound covers both
            let path = parse_path(&term, rdf, depth, steps)?;
            Ok(ASTNodeExpr::PathValues {
                path,
                focus_node: fnode.map(Box::new),
            })
        })
        .transpose()
}
