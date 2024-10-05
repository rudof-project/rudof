use std::cell::RefCell;
use std::rc::Rc;

use shacl_ast::compiled::component::UniqueLang;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::helpers::validate_with;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<S: SRDFBasic> Validator<S> for UniqueLang {
    fn validate(
        &self,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let langs: Rc<RefCell<Vec<_>>> = Rc::new(RefCell::new(Vec::new()));

        let unique_lang = |value_node: &S::Term| {
            if let Some(literal) = S::term_as_literal(value_node) {
                if let Some(lang) = S::lang(&literal) {
                    let mut langs_borrowed = langs.borrow_mut();

                    if langs_borrowed.contains(&lang) {
                        return true;
                    } else {
                        langs_borrowed.push(lang);
                    }
                }
            }
            false
        };

        validate_with(value_nodes, &ValueNodeIteration, unique_lang)
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for UniqueLang {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, NativeEngine, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        self.validate(store, SparqlEngine, value_nodes)
    }
}
