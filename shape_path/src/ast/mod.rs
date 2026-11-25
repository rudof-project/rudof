//! Abstract syntax of [ShExPath](https://shexspec.github.io/spec/ShExPath).
//!
//! The types follow the nonterminals of the spec's grammar, which are those of the reference
//! implementation's parser, `ShapePathParser.jison`.

mod axis;
mod deref;
mod filter;
mod keywords;

pub use axis::*;
pub use filter::*;
pub use keywords::*;

/// A ShExPath.
///
/// ```text
/// shapePath        ::= sequenceStep ("," sequenceStep)*
/// sequenceStep     ::= unionStep ("union" unionStep)*
/// unionStep        ::= intersectionStep ("intersection" intersectionStep)*
/// intersectionStep ::= startStep nextStep*
/// ```
///
/// A junction holds at least two operands; a single operand is that operand.
#[derive(Debug, Clone, PartialEq)]
pub enum ShExPath {
    /// `,`: every item each operand selected, in order, keeping duplicates.
    Sequence(Vec<ShExPath>),
    /// `union`: every item either operand selected, with duplicates removed.
    Union(Vec<ShExPath>),
    /// `intersection`: the items every operand selected.
    Intersection(Vec<ShExPath>),
    /// Steps applied left to right, each to every item the previous one produced.
    Path(Vec<Step>),
}

impl ShExPath {
    /// A path made of `steps`.
    pub fn path(steps: impl IntoIterator<Item = Step>) -> Self {
        ShExPath::Path(steps.into_iter().collect())
    }
}

/// A step of a path: which items are reachable from each item it is applied to, and the filters
/// that keep some of them.
///
/// ```text
/// step ::= "child::"? termType filter*
///        | "child::"? attributeOrAny termType? filter*
///        | nonChildAxis termType? filter*
///        | "(" shapePath ")" termType? filter*
/// ```
///
/// A term type is kept as the first filter, since it is "exactly the filter `[type = T]`".
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub selector: Selector,
    pub filters: Vec<Filter>,
}

impl Step {
    /// A step without filters.
    pub fn new(selector: Selector) -> Self {
        Step {
            selector,
            filters: Vec::new(),
        }
    }

    /// A `child::` step without filters.
    pub fn child(attribute: AttributeOrAny) -> Self {
        Step::new(Selector::Child(attribute))
    }

    /// A step on `axis` without filters.
    pub fn axis(axis: Axis) -> Self {
        Step::new(Selector::Axis(axis))
    }

    /// This step with `filter` added after its filters.
    #[must_use]
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }
}

/// Which items a step reaches from each item it is applied to.
#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    /// `child::attribute` or `child::*`.
    Child(AttributeOrAny),
    /// Any other axis.
    Axis(Axis),
    /// `"(" shapePath ")"`: a parenthesized path.
    Path(Box<ShExPath>),
}

/// `attributeOrAny ::= "*" | attribute`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeOrAny {
    /// `*`: every property value of an object, or every member of an array.
    Any,
    /// The value of that ShExJ property.
    Attribute(Attribute),
}
