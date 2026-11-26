use crate::ast::ASTTarget;
use crate::rdf::parsers::node_expr::node_expr_from_term;
use crate::types::Target;
use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{ParseBudget, ValuesPropertyParser};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use std::marker::PhantomData;
use std::sync::Arc;

pub(crate) fn targets_node<RDF: FocusRDF>(budget: Arc<ParseBudget>) -> impl RDFNodeParse<RDF, Output = Vec<ASTTarget>> {
    TargetsNodeParser {
        budget,
        _marker: PhantomData,
    }
}

struct TargetsNodeParser<RDF> {
    budget: Arc<ParseBudget>,
    _marker: PhantomData<RDF>,
}

impl<RDF: FocusRDF> RDFNodeParse<RDF> for TargetsNodeParser<RDF> {
    type Output = Vec<ASTTarget>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus().cloned();
        let values = ValuesPropertyParser::new(ShaclVocab::sh_target_node()).parse_focused(rdf)?;

        let mut targets = Vec::with_capacity(values.len());
        for value in values.iter() {
            let expr = self
                .budget
                .spend(rdf, |rdf, steps| node_expr_from_term(value, rdf, 0, steps));

            targets.push(match expr {
                Ok(expr) => Target::Node(expr),
                // An ill-formed target is kept, to report a violation for it instead of
                // failing the whole parse
                Err(_) => Target::WrongNode(RDF::term_as_object(value)?),
            });
        }

        // The node expressions move the focus to the nodes they are written on
        if let Some(focus) = &focus {
            rdf.set_focus(focus);
        }

        Ok(targets)
    }
}
