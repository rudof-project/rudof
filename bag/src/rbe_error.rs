use crate::{max::Max, Cardinality};
use crate::rbe::Rbe;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum RbeError<A> {
    UnexpectedEmpty { x: A, open: bool },
    UnexpectedSymbol { x: A, expected: A, open: bool },
    MaxCardinalityZeroFoundValue { x: A },
    RangeNegativeLowerBound { min: usize },
    RangeLowerBoundBiggerMax { card: Cardinality },
    CardinalityZeroZeroDeriv { x: A },
    CardinalityFail { n: usize, card: Cardinality },
    NonNullable{ rest: Box<Rbe<A>> }
}
