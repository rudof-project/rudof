use rbe::RbeError;

use crate::TestType;

#[derive(PartialEq, Debug, Clone)]
pub enum RbeTestResult {
    Passed(PassedTestResult),
    Failed(FailedTestResult)
}

impl RbeTestResult {
    pub fn passed(name: String) -> RbeTestResult {
        RbeTestResult::Passed(
            PassedTestResult { name: name }
        )
    }

    pub fn failed(name: String, err: RbeError<TestType>) -> RbeTestResult {
        RbeTestResult::Failed(
            FailedTestResult { 
                name: name,
                error: err
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PassedTestResult {
    name: String
}

impl PassedTestResult {
    pub fn name(&self) -> String {
       self.name.clone()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FailedTestResult {
    name: String,
    error: RbeError<TestType>
}

impl FailedTestResult {
    pub fn name(&self) -> String {
        self.name.clone()
     }

     pub fn err(&self) -> RbeError<TestType> {
        self.error.clone()
     }
}