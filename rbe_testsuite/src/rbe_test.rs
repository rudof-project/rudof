use std::vec::IntoIter;

use rbe::{rbe::Rbe, rbe_error::RbeError, RbeMatcher};
use serde_derive::{Deserialize, Serialize};
use crate::{MatchResult, RbeTestResult, KeyType, ValueType, RefType};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RbeTest {
    
    #[serde(skip)]
    group: String,
    
    #[serde(default)]
    name: String,
    
    #[serde(skip)]
    full_name: String,
    
    rbe: Rbe<KeyType, ValueType, RefType>,
    
    bag: Vec<(KeyType, ValueType)>,

    open: bool,
    
    match_result: MatchResult
}

impl RbeTest {

    pub fn new() -> RbeTest {
        RbeTest::default()
    }

    /// The group name of this test
    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn set_group(&mut self, group: String) {
        self.group = group;
    }

    /// The name of this test.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_full_name(&mut self, full_name: String) {
        self.full_name = full_name;
    }

    pub fn set_rbe(&mut self, rbe: Rbe<KeyType, ValueType, RefType>) 
    {
        self.rbe = rbe;
    }

    pub fn set_bag(&mut self, bag: Vec<(KeyType, ValueType)>) 
    {
        self.bag = bag;
    }

    pub fn set_match_result(&mut self, match_result: MatchResult) 
    {
        self.match_result = match_result;
    }

    /// Runs this test
    pub fn run(&self) -> RbeTestResult {
        let rbe_matcher = RbeMatcher::new()
            .with_rbe(self.rbe.clone())
            .with_open(self.open);
        let v: Vec<(KeyType, ValueType)> = self.bag.clone() ;
        let iter: IntoIter<(KeyType,ValueType)> = v.into_iter();
        match (&self.match_result, rbe_matcher.matches(iter)) {
          (MatchResult::Pass, Ok(_)) => {
             RbeTestResult::passed(self.name().to_string())
           },
           (MatchResult::Pass, Err(err)) => {
             RbeTestResult::failed(self.name().to_string(),err) 
           }
           (MatchResult::Fail, Ok(_)) => {
            RbeTestResult::failed(
                self.name().to_string(), 
                RbeError::ShouldFailButPassed { name: self.name.clone() }
            )},
           (MatchResult::Fail, Err(_)) => {
            RbeTestResult::passed(self.name().to_string())
           }
        } 
    }

    /// The full name of this test, which is formed by joining the group
    /// name, the test name and the additional name with a `/`.
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}
