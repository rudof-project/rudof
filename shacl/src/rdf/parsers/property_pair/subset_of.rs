use crate::ast::ASTComponent;
use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{ParseBudget, ShaclPathParser};
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use std::marker::PhantomData;
use std::sync::Arc;

/// Parses the values of `sh:subsetOf`.
///
/// Unlike the other property pair components, the values of `sh:subsetOf` are
/// SHACL property paths and not plain predicate IRIs, so they can be blank nodes
/// heading a path expression and `IrisPropertyParser` is not enough.
///
/// https://www.w3.org/TR/shacl12-core/#SubsetOfConstraintComponent
struct SubsetOfParser<RDF: FocusRDF> {
    budget: Arc<ParseBudget>,
    _marker: PhantomData<RDF>,
}

impl<RDF: FocusRDF> RDFNodeParse<RDF> for SubsetOfParser<RDF> {
    type Output = Vec<ASTComponent>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus().cloned().ok_or(RDFError::NoFocusNodeError)?;
        let values = rdf.objects_for(&focus, &ShaclVocab::sh_subset_of().into())?;

        let mut components = Vec::new();
        for value in values {
            // Parsing a path moves the focus to the path node
            let path = ShaclPathParser::with_budget(value, self.budget.clone()).parse_focused(rdf)?;
            components.push(ASTComponent::SubsetOf(path));
        }

        // The component parsers run in sequence over the same focus node, so the
        // focus must be restored for the ones that come after this parser
        rdf.set_focus(&focus);
        Ok(components)
    }
}

pub(crate) fn subset_of<RDF: FocusRDF>(budget: Arc<ParseBudget>) -> impl RDFNodeParse<RDF, Output = Vec<ASTComponent>> {
    SubsetOfParser {
        budget,
        _marker: PhantomData,
    }
}
