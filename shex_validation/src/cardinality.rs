use crate::CardinalityError;

fn check_cardinality(
    c: usize,
    min: &Option<i32>,
    max: &Option<i32>,
) -> Result<(), CardinalityError> {
    let min = min.unwrap_or(1);
    if c < min.try_into().unwrap() {
        return Err(CardinalityError::CardinalityLessThanMin { c: c, min: min });
    }
    let max = max.unwrap_or(1);
    if max == -1 {
        // max = -1 means unbounded
        return Ok(());
    }
    if c > max.try_into().unwrap() {
        return Err(CardinalityError::CardinalityGreaterThanMax { c: c, max: max });
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::{*, cardinality::check_cardinality};

    #[test]
    fn test_cardinality() -> Result<(), CardinalityError> {
        let ce = check_cardinality(1, &None, &None)?;
        assert_eq!(ce, ());
        Ok(())
    }

}
