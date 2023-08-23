use crate::ResultMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

pub struct ValidationState<N, S>
where
    N: Hash + Eq,
    S: Hash + Eq,
{
    current_goal: Option<(N, S)>,
    result_map: ResultMap<N, S>,
    alternatives: Vec<ResultMap<N, S>>,
}

impl<N, S> ValidationState<N, S>
where
    N: Hash + Eq + Clone + Debug,
    S: Hash + Eq + Clone + Debug,
{
    pub fn new() -> ValidationState<N, S> {
        ValidationState {
            current_goal: None,
            result_map: ResultMap::new(),
            alternatives: Vec::new(),
        }
    }
}
