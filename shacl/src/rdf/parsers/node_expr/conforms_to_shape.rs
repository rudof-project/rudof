use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, expr_list, single};
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn conforms_to_shape<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
    depth: usize,
    steps: &mut usize,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_conforms_to_shape())?
        .map(|term| {
            let args = expr_list::<RDF>(&term, rdf, depth, steps)?;
            let [node, shape] = <[ASTNodeExpr; 2]>::try_from(args).map_err(|args| RDFError::DefaultError {
                msg: format!(
                    "{} must have a list of two node expressions as its value, found {}",
                    ShaclNodeExprVocab::SHNEX_CONFORMS_TO_SHAPE,
                    args.len()
                ),
            })?;

            Ok(ASTNodeExpr::ConformsToShape {
                node: node.into(),
                shape: shape.into(),
            })
        })
        .transpose()
}
