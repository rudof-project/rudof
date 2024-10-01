use indoc::formatdoc;
use shacl_ast::compiled::property_shape::PropertyShape;
use shacl_ast::compiled::shape::Shape;
use srdf::QuerySRDF;
use srdf::SHACLPath;

use crate::constraints::SparqlDeref;
use crate::context::Context;
use crate::helper::sparql::select;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

use super::ValidatorRunner;

pub struct SparqlValidatorRunner;

impl<S: QuerySRDF> ValidatorRunner<S> for SparqlValidatorRunner {
    fn evaluate(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ValidateError> {
        let validator = evaluation_context.component().deref();
        Ok(validator.validate_sparql(evaluation_context, value_nodes)?)
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
        store: &S,
        _shape: &Shape<S>,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn predicate(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _predicate: &S::IRI,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn alternative(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn sequence(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _paths: &[SHACLPath],
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn inverse(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_more(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn one_or_more(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }

    fn zero_or_one(
        &self,
        store: &S,
        _shape: &PropertyShape<S>,
        _path: &SHACLPath,
        _focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        Err(ValidateError::NotImplemented)
    }
}
