use crate::ir::components::LanguageIn;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator, validate_with};
use crate::validator::engine::Engine;
use crate::validator::iteration::ValueNodeIteration;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::literal::Literal;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for LanguageIn {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            |vn| {
                if let Ok(lit) = S::term_as_literal(vn) {
                    return match lit.lang() {
                        None => true,
                        Some(lang) => {
                            let lang_str = lang.to_string().to_lowercase();
                            !self.langs().iter().any(|l| {
                                let l_str = l.to_string().to_lowercase();
                                lang_str == l_str || lang_str.starts_with(&format!("{}-", l_str))
                            })
                        },
                    };
                }
                true
            },
            &format!(
                "LanguageIn constraint not satisfied. Expected one of {}",
                self.langs()
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            maybe_path,
        )
    }
}
