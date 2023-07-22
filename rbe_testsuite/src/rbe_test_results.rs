use crate::{RbeTestResult, PassedTestResult, FailedTestResult};

pub struct RbeTestsResults {
    passed: Vec<PassedTestResult>,
    failed: Vec<FailedTestResult>,
 }
 
 
 
 impl <'a> RbeTestsResults {
     pub fn new() -> RbeTestsResults {
         RbeTestsResults {
         passed: Vec::new(),
         failed: Vec::new()
     }
    }
 
    pub fn add_result(&mut self, result: &RbeTestResult) {
      match &result {
         RbeTestResult::Passed(result) => { 
            self.passed.push(result.clone());
        },
         RbeTestResult::Failed(result) => { 
            self.failed.push(result.clone());
        },
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
 