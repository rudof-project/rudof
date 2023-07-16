use crate::max::Max;

#[derive(PartialEq, Eq, Clone)]
pub enum RbeError<A> {
   UnexpectedEmpty { x: A, open: bool },
   UnexpectedSymbol { x: A, expected: A, open: bool },
   MaxCardinalityZeroFoundValue { x: A },
   RangeNegativeLowerBound { min: usize },
   RangeLowerBoundBiggerMax { min: usize, max: Max },
   CardinalityZeroZeroDeriv { x: A }
}
