use std::cell::RefCell;
use std::rc::Rc;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::UniqueLang;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;
use srdf::model::sparql::Sparql;
use srdf::model::Term;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf + Clone + 'static, E: Engine<R>> NativeValidator<R, E> for UniqueLang {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let langs: Rc<RefCell<Vec<_>>> = Rc::new(RefCell::new(Vec::new()));
        let unique_lang = |value_node: &TObjectRef<R>| {
            if let Some(lang) = value_node.literal() {
                let lang = lang.clone();
                let mut langs_borrowed = langs.borrow_mut();
                match langs_borrowed.contains(&lang) {
                    true => return true,
                    false => langs_borrowed.push(lang),
                }
            }
            false
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            unique_lang,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql + Clone + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate_native(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            subsetting,
        )
    }
}
