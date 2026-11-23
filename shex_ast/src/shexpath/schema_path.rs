//! The ShExPath of a shape expression or a triple constraint of a schema, written with the
//! shortcuts a person would use: `@:S`, `@:S~:p`, `@:S~:p/valueExpr`.

use super::parser::shortcuts::{predicate_shortcut, shape_label_shortcut};
use super::{Attribute, AttributeOrAny, Axis, Filter, ShExPath, Step};
use prefixmap::{IriRef, PrefixMap};
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Where a [`SchemaPath`] starts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemaPathRoot {
    /// The shape expression declared with this label: `@label`.
    Shape(IriS),
    /// The `start` shape expression of the schema: `/start`.
    Start,
}

/// A step of a [`SchemaPath`] down from its root.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemaStep {
    /// A triple constraint on `predicate` in the shape: `~predicate`. When the shape has several
    /// constraints on that predicate, `nth` is the position of this one among them, from 0, and
    /// the step is `~predicate/self::[nth]`.
    TripleConstraint { predicate: IriS, nth: Option<usize> },
    /// The value expression of a triple constraint: `/valueExpr`.
    ValueExpr,
    /// An operand of a `ShapeAnd` or `ShapeOr`, from 0: `/shapeExprs/*[i]`.
    Operand(usize),
    /// The shape expression a `ShapeNot` negates: `/shapeExpr`.
    Negated,
}

/// The ShExPath of a shape expression or a triple constraint of a schema.
///
/// It is shown with the shortcuts of ShExPath, and [`SchemaPath::to_shexpath`] is the path the
/// [`ShExPathParser`](super::ShExPathParser) reads from what it shows.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaPath {
    root: SchemaPathRoot,
    steps: Vec<SchemaStep>,
}

impl SchemaPath {
    /// The path of the shape expression declared as `label`.
    pub fn shape(label: IriS) -> Self {
        SchemaPath {
            root: SchemaPathRoot::Shape(label),
            steps: Vec::new(),
        }
    }

    /// The path of the `start` shape expression.
    pub fn start() -> Self {
        SchemaPath {
            root: SchemaPathRoot::Start,
            steps: Vec::new(),
        }
    }

    /// This path followed by `step`.
    pub fn with_step(mut self, step: SchemaStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn root(&self) -> &SchemaPathRoot {
        &self.root
    }

    pub fn steps(&self) -> &[SchemaStep] {
        &self.steps
    }

    /// The path with its IRIs written as prefixed names of `prefixmap` where it can.
    pub fn show_qualified(&self, prefixmap: &PrefixMap) -> String {
        self.show(|iri| prefixmap.qualify(iri))
    }

    /// The path as a [`ShExPath`], with the shortcuts expanded as the parser expands them.
    pub fn to_shexpath(&self) -> ShExPath {
        let mut steps = match &self.root {
            SchemaPathRoot::Shape(label) => shape_label_shortcut(IriRef::Iri(label.clone())),
            SchemaPathRoot::Start => vec![Step::child(AttributeOrAny::Attribute(Attribute::Start))],
        };
        for step in &self.steps {
            match step {
                SchemaStep::TripleConstraint { predicate, nth } => {
                    steps.extend(predicate_shortcut(IriRef::Iri(predicate.clone())));
                    if let Some(nth) = nth {
                        steps.push(Step::axis(Axis::Itself).with_filter(Filter::index(*nth as i64)));
                    }
                },
                SchemaStep::ValueExpr => steps.push(Step::child(AttributeOrAny::Attribute(Attribute::ValueExpr))),
                SchemaStep::Operand(i) => {
                    steps.push(Step::child(AttributeOrAny::Attribute(Attribute::ShapeExprs)));
                    steps.push(Step::child(AttributeOrAny::Any).with_filter(Filter::index(*i as i64)));
                },
                SchemaStep::Negated => steps.push(Step::child(AttributeOrAny::Attribute(Attribute::ShapeExpr))),
            }
        }
        ShExPath::path(steps)
    }

    fn show(&self, show_iri: impl Fn(&IriS) -> String) -> String {
        let mut out = match &self.root {
            SchemaPathRoot::Shape(label) => format!("@{}", show_iri(label)),
            SchemaPathRoot::Start => "/start".to_string(),
        };
        for step in &self.steps {
            match step {
                SchemaStep::TripleConstraint { predicate, nth } => {
                    out.push('~');
                    out.push_str(&show_iri(predicate));
                    if let Some(nth) = nth {
                        out.push_str(&format!("/self::[{nth}]"));
                    }
                },
                SchemaStep::ValueExpr => out.push_str("/valueExpr"),
                SchemaStep::Operand(i) => out.push_str(&format!("/shapeExprs/*[{i}]")),
                SchemaStep::Negated => out.push_str("/shapeExpr"),
            }
        }
        out
    }
}

/// The path with every IRI written in full, as `<iri>`.
impl Display for SchemaPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.show(|iri| format!("<{iri}>")))
    }
}
