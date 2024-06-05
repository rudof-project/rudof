use crate::{FailedTestResult, PassedTestResult, RbeTestResult};

#[derive(Default)]
pub struct RbeTestsResults {
    passed: Vec<PassedTestResult>,
    failed: Vec<FailedTestResult>,
}

impl RbeTestsResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, result: &RbeTestResult) {
        match &result {
            RbeTestResult::Passed(result) => {
                self.passed.push(result.clone());
            }
            RbeTestResult::Failed(result) => {
                self.failed.push(result.clone());
            }
        }
    }

    pub fn count_passed(&self) -> usize {
        self.passed.len()
    }

    pub fn count_failed(&self) -> usize {
        self.failed.len()
    }

    pub fn failed(&self) -> std::slice::Iter<'_, FailedTestResult> {
        self.failed.iter()
    }
}
