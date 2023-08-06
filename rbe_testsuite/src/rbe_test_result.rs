use rbe::rbe_error::RbeError;
use crate::{KeyType, ValueType, RefType};

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

    pub fn failed(name: String, err: RbeError<KeyType, ValueType, RefType>) -> RbeTestResult {
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
    error: RbeError<KeyType, ValueType, RefType>
}

impl FailedTestResult {
    pub fn name(&self) -> String {
        self.name.clone()
     }

     pub fn err(&self) -> RbeError<KeyType, ValueType, RefType> {
        self.error.clone()
     }
}