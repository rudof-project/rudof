use crate::Rudof;

/// How many staging frames are open, zero when nothing is staged.
pub fn staging_depth(rudof: &Rudof) -> usize {
    rudof.staging.len()
}
