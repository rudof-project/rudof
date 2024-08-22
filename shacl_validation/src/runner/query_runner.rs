use indoc::formatdoc;
use shacl_ast::property_shape::PropertyShape;
use srdf::QuerySRDF;
use srdf::SHACLPath;

use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::sparql::select;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

use super::ValidatorRunner;

pub struct QueryValidatorRunner;

impl<S: QuerySRDF + 'static> ValidatorRunner<S> for QueryValidatorRunner {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ValidateError> {
        let component: Box<dyn SparqlConstraintComponent<S>> =
            evaluation_context.component().into();
        Ok(component.evaluate_sparql(validation_context, evaluation_context, value_nodes)?)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &S::Term) -> Result<Targets<S>, ValidateError> {
        if S::term_is_bnode(node) {
            return Err(ValidateError::TargetNodeBlankNode);
        }

        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented)
    }

    fn target_class(&self, store: &S, class: &S::Term) -> Result<Targets<S>, ValidateError> {
        if !S::term_is_iri(class) {
            return Err(ValidateError::TargetClassNotIri);
        }

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {} .
            }}
        ", class};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented)
    }

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
    ) -> Result<Targets<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented)
    }

    fn target_object_of(&self, store: &S, predicate: &S::IRI) -> Result<Targets<S>, ValidateError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this .
            }}
        ", predicate};

        select(store, query, "this")?;

        Err(ValidateError::NotImplemented)
    }

    fn implicit_target_class(
        &self,
        _store: &S,
        _shape: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn predicate(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _predicate: &S::IRI,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn alternative(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn sequence(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn inverse(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn one_or_more(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_one(
        &self,
        _store: &S,
        _shape: &PropertyShape,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }
}
