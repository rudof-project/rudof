use crate::{max::Max, Cardinality};

#[derive(PartialEq, Eq, Clone)]
pub enum RbeError<A> {
    UnexpectedEmpty { x: A, open: bool },
    UnexpectedSymbol { x: A, expected: A, open: bool },
    MaxCardinalityZeroFoundValue { x: A },
    RangeNegativeLowerBound { min: usize },
    RangeLowerBoundBiggerMax { card: Cardinality },
    CardinalityZeroZeroDeriv { x: A },
    CardinalityFail { n: usize, card: Cardinality },
}
