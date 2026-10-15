use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::ListParser;
use rudof_rdf::rdf_core::vocabs::RdfVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{values, Neighs, to_object};

pub(crate) fn list<RDF: FocusRDF>(neighs: &Neighs<RDF>, rdf: &mut RDF, term: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    values::<RDF>(neighs, &RdfVocab::rdf_first())
        .map(|_| {
            rdf.set_focus(term);
            let members = ListParser::new().parse_focused(rdf)?;
            let members = members
                .iter()
                .map(|member| to_object::<RDF>(member))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ASTNodeExpr::List(members))
        })
        .transpose()
}