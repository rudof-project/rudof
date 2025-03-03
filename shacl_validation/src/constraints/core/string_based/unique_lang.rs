use std::cell::RefCell;
use std::rc::Rc;

use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::component::UniqueLang;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for UniqueLang {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if !self.unique_lang() {
            return Ok(Default::default());
        }

        let langs: Rc<RefCell<Vec<_>>> = Rc::new(RefCell::new(Vec::new()));

        let unique_lang = |value_node: &Q::Term| {
            let tmp: Result<Q::Literal, _> = value_node.clone().try_into();
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

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            unique_lang,
        )
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for UniqueLang {}
