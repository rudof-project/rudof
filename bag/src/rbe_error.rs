use crate::max::Max;

#[derive(Clone)]
pub enum RbeError<A> {
   UnexpectedEmpty { x: A, open: bool },
   UnexpectedSymbol { x: A, expected: A, open: bool },
   MaxCardinalityZeroFoundValue { x: A },
   RangeNegativeLowerBound { min: usize },
   RangeLowerBoundBiggerMax { min: usize, max: Max }
}
