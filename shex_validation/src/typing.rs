use std::collections::HashMap;

use colored::{Color, Colorize};
use either::Either;
use prefixmap::PrefixMap;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::ir::shape_label::ShapeLabel;
use shex_ast::{Node, ShapeLabelIdx};

use crate::reason::Reason;
use crate::validator_error::ValidatorError;

/// ShEx's instantiation of `rudof_typing`'s generic validation outcome:
/// either the reasons why a `(Node, ShapeLabelIdx)` pair conforms, or the
/// errors why it doesn't.
pub type ValidationResult = rudof_typing::ValidationResult<ValidatorError, Reason>;

/// ShEx's instantiation of the generic memoization-cache trait. See
/// [`rudof_typing::Typing`] for the generic contract.
pub use rudof_typing::Typing;

/// ShEx's instantiation of the generic memoization cache: a plain
/// `HashMap`-backed [`Typing`] that also notifies an optional
/// [`TypingObserver`] on every insert.
pub type ObservableTyping = rudof_typing::ObservableTyping<Node, ShapeLabelIdx, ValidatorError, Reason>;

/// ShEx's instantiation of the generic typing-observer trait, as the exact
/// `dyn` type stored in [`crate::ValidatorConfig`] and passed around as
/// `Arc<TypingObserver>`. A new observer (see [`ConsoleTypingObserver`]
/// below) implements `rudof_typing::TypingObserver<Node, ShapeLabelIdx,
/// ValidatorError, Reason>` directly, which coerces to this type.
pub type TypingObserver = dyn rudof_typing::TypingObserver<Node, ShapeLabelIdx, ValidatorError, Reason>;

/// Qualifies a shape label against a prefixmap, mirroring `ShapeLabel`'s own
/// `Display` impl but relativizing the `Iri` case.
fn qualify_shape_label(label: &ShapeLabel, prefixmap: &PrefixMap) -> String {
    match label {
        ShapeLabel::Iri(iri) => prefixmap.qualify(iri),
        ShapeLabel::BNode(bnode) => bnode.to_string(),
        ShapeLabel::Start => "Start".to_string(),
    }
}

/// Default [`TypingObserver`] for `show_intermediate_results`: prints each
/// `(node, shape)` result to stderr as soon as it's cached, so a user can
/// follow validation progress instead of waiting for the final report.
/// Nodes and shape labels are qualified against their prefixmaps, and the
/// conformant/non-conformant verdict is colorized.
#[derive(Debug)]
pub struct ConsoleTypingObserver {
    labels: HashMap<ShapeLabelIdx, String>,
    nodes_prefixmap: PrefixMap,
    conformant_color: Color,
    non_conformant_color: Color,
}

impl ConsoleTypingObserver {
    pub fn new(
        schema: &SchemaIR,
        nodes_prefixmap: PrefixMap,
        conformant_color: Color,
        non_conformant_color: Color,
    ) -> Self {
        let shapes_prefixmap = schema.prefixmap();
        let mut labels = HashMap::new();
        for (label, _iri, _expr) in schema.shapes() {
            if let Ok(idx) = schema.get_shape_label_idx(label) {
                labels.insert(idx, qualify_shape_label(label, &shapes_prefixmap));
            }
        }
        ConsoleTypingObserver {
            labels,
            nodes_prefixmap,
            conformant_color,
            non_conformant_color,
        }
    }
}

impl rudof_typing::TypingObserver<Node, ShapeLabelIdx, ValidatorError, Reason> for ConsoleTypingObserver {
    fn on_insert(&self, key: &(Node, ShapeLabelIdx), value: &ValidationResult) {
        let (node, idx) = key;
        let node = node.show_qualified(&self.nodes_prefixmap);
        let shape = self.labels.get(idx).map(String::as_str).unwrap_or("?");
        match value {
            Either::Right(reasons) => {
                let status = "conformant".color(self.conformant_color);
                eprintln!(
                    "[validating] {node} @ {shape} -> {status} ({} reason(s))",
                    reasons.len()
                );
            },
            Either::Left(errors) => {
                let status = "non-conformant".color(self.non_conformant_color);
                eprintln!("[validating] {node} @ {shape} -> {status} ({} error(s))", errors.len());
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rudof_iri::IriS;
    use shex_ast::BNode;

    #[test]
    fn qualify_shape_label_qualifies_iris_against_the_prefixmap() {
        let prefixmap: PrefixMap = std::collections::HashMap::from([("ex", "http://a.example/")])
            .try_into()
            .unwrap();
        let label = ShapeLabel::iri(IriS::new_unchecked("http://a.example/S"));
        assert_eq!(qualify_shape_label(&label, &prefixmap), "ex:S");
    }

    #[test]
    fn qualify_shape_label_leaves_bnodes_and_start_untouched() {
        let prefixmap = PrefixMap::default();
        assert_eq!(
            qualify_shape_label(&ShapeLabel::from_bnode(BNode::from("b1")), &prefixmap),
            "_:b1"
        );
        assert_eq!(qualify_shape_label(&ShapeLabel::Start, &prefixmap), "Start");
    }
}
