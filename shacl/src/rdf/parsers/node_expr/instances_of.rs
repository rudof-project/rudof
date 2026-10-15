use prefixmap::IriRef;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_iri;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;
use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::{single, Neighs};

pub(crate) fn instances_of<RDF: FocusRDF>(neighs: &Neighs<RDF>, _: &mut RDF, _: &RDF::Term) -> Result<Option<ASTNodeExpr>, RDFError> {
    single::<RDF>(neighs, &ShaclNodeExprVocab::shnex_instances_of())?
        .map(|term| {
            let iri = term_to_iri::<RDF>(&term)?;
            Ok(ASTNodeExpr::InstancesOf(IriRef::iri(iri)))
        })
        .transpose()
}