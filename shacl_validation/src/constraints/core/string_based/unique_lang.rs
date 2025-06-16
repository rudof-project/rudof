use std::cell::RefCell;
use std::rc::Rc;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::UniqueLang;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::QueryRDF;
use std::fmt::Debug;

impl<S: Rdf + Debug> Validator<S> for UniqueLang {
    fn validate(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let langs: Rc<RefCell<Vec<_>>> = Rc::new(RefCell::new(Vec::new()));

        let unique_lang = |value_node: &S::Term| {
            let tmp: Result<S::Literal, _> = S::term_as_literal(value_node);
            if let Ok(lang) = tmp {
                let lang = lang.clone();
                let mut langs_borrowed = langs.borrow_mut();
                match langs_borrowed.contains(&lang) {
                    true => return true,
                    false => langs_borrowed.push(lang),
                }
            }
            false
        };

        let message = "UniqueLang not satisfied".to_string();
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            unique_lang,
            &message,
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for UniqueLang {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
