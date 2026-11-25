//! The filters of a step.

use super::{ShExPath, TermType};
use prefixmap::IriRef;

/// A filter of a step.
///
/// ```text
/// filter     ::= "[" filterExpr "]"
/// filterExpr ::= "assert"? shapePath comparison?
///              | "assert"? function comparison?
///              | numericExpr
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// A term type: keeps the items whose ShExJ `type` is that one.
    TermType(TermType),
    /// A bracketed filter expression. `[N]` is read as `[index() = N]`, of which it is shorthand.
    Test {
        /// `assert`: keep every item if the test holds, and otherwise the path is in error.
        assert: bool,
        operand: Operand,
        comparison: Option<Comparison>,
    },
}

impl Filter {
    /// `[operand]`, kept through its effective boolean value.
    pub fn test(operand: Operand) -> Self {
        Filter::Test {
            assert: false,
            operand,
            comparison: None,
        }
    }

    /// `[operand comparator rvalue]`.
    pub fn compare(operand: Operand, comparator: Comparator, rvalue: RValue) -> Self {
        Filter::Test {
            assert: false,
            operand,
            comparison: Some(Comparison { comparator, rvalue }),
        }
    }

    /// `[N]`, shorthand for `[index() = N]`.
    pub fn index(n: i64) -> Self {
        Filter::compare(Operand::Index, Comparator::Equal, RValue::Integer(n))
    }

    /// This filter with `assert` in front. A term type cannot be asserted and is returned unchanged.
    #[must_use]
    pub fn asserted(self) -> Self {
        match self {
            Filter::Test {
                operand, comparison, ..
            } => Filter::Test {
                assert: true,
                operand,
                comparison,
            },
            other @ Filter::TermType(_) => other,
        }
    }
}

/// What a filter expression tests.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A path, evaluated from the item.
    Path(ShExPath),
    /// `index()`: the position of the item in the value being trimmed, from 0.
    Index,
    /// `count()`: the size of the value being trimmed.
    Count,
}

/// `comparison ::= comparitor rvalue`
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub comparator: Comparator,
    pub rvalue: RValue,
}

/// `comparitor ::= "=" | "<" | ">"`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comparator {
    /// `=`
    Equal,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
}

/// `rvalue ::= INTEGER | iri`
#[derive(Debug, Clone, PartialEq)]
pub enum RValue {
    Integer(i64),
    Iri(IriRef),
}
