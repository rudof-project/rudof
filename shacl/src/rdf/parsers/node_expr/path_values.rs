use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{Neighs, optional_expr, single};
use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ShaclPathParser;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};

pub(crate) fn path_values<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    _: &RDF::Term,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_path_values())?
        .map(|term| {
            let fnode = optional_expr(neighs, &ShaclNodeExprVocab::shnex_focus_node(), rdf)?;
            let path = ShaclPathParser::new(term).parse_focused(rdf)?;
            Ok(ASTNodeExpr::PathValues {
                path,
                focus_node: fnode.map(Box::new),
            })
        })
        .transpose()
}
