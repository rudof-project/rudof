use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardinalityError {
    #[error("Cardinality {c:?} less than min {min:?}")]
    CardinalityLessThanMin { c: usize, min: i32 },

    #[error("Cardinality {c:?} greater than max {max:?}")]
    CardinalityGreaterThanMax { c: usize, max: i32 },
}
