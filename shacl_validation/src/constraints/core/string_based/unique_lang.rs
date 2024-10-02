use std::cell::RefCell;
use std::rc::Rc;

use shacl_ast::compiled::component::UniqueLang;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::runner::native::NativeValidatorRunner;
use crate::runner::sparql::SparqlValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDFBasic + 'static> Validator<S> for UniqueLang {
    fn validate(
        &self,
        store: &S,
        runner: impl ValidatorRunner<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(ValidationResults::default());
        }

        let langs = Rc::new(RefCell::new(Vec::new()));

        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let langs = Rc::clone(&langs);
                let mut langs = langs.borrow_mut();

                if let Some(literal) = S::term_as_literal(value_node) {
                    if let Some(lang) = S::lang(&literal) {
                        if langs.contains(&lang) {
                            Some(ValidationResult::new(focus_node, Some(value_node)))
                        } else {
                            langs.push(lang);
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: SRDF + 'static> NativeValidator<S> for UniqueLang {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, NativeValidatorRunner, value_nodes)
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for UniqueLang {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        self.validate(store, SparqlValidatorRunner, value_nodes)
    }
}
