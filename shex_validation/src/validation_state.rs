use rbe::Pending;

use crate::{ResultMap, MAX_STEPS};
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
    max_steps: usize,
    step_counter: usize,
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
            max_steps: MAX_STEPS,
            step_counter: 0,
        }
    }

    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn set_current_goal(&mut self, n: &N, s: &S) {
        self.current_goal = Some(((*n).clone(), (*s).clone()));
    }

    pub fn add_ok(&mut self, n: N, s: S) {
        self.result_map.add_ok(n, s);
    }

    pub fn more_pending(&self) -> bool {
        self.result_map.more_pending()
    }

    pub fn add_pending(&mut self, n: N, s: S) {
        self.result_map.add_pending(n, s);
    }

    pub fn pop_pending(&mut self) -> Option<(N, S)> {
        self.result_map.pop_pending()
    }

    pub fn steps(&self) -> usize {
        self.step_counter
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }
}
