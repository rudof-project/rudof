use rbe::{rbe::Rbe, Bag, DerivError};
use serde_derive::{Deserialize, Serialize};
use crate::{TestType, MatchResult, RbeTestResult};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RbeTest {
    
    #[serde(skip)]
    group: String,
    
    #[serde(default)]
    name: String,
    
    #[serde(skip)]
    full_name: String,
    
    rbe: Rbe<TestType>,
    
    bag: Bag<TestType>,

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

    pub fn set_rbe(&mut self, rbe: Rbe<TestType>) 
    {
        self.rbe = rbe;
    }

    pub fn set_bag(&mut self, bag: Bag<TestType>) 
    {
        self.bag = bag;
    }

    pub fn set_match_result(&mut self, match_result: MatchResult) 
    {
        self.match_result = match_result;
    }

    /// Runs this test
    pub fn run(&self) -> RbeTestResult {
        match (&self.match_result, self.rbe.match_bag(&self.bag, self.open)) {
          (MatchResult::Pass, Ok(())) => {
             RbeTestResult::passed(self.name().to_string())
           },
           (MatchResult::Pass, Err(err)) => {
             RbeTestResult::failed(self.name().to_string(),err) 
           }
           (MatchResult::Fail, Ok(())) => {
            RbeTestResult::failed(
                self.name().to_string(), 
                DerivError::ShouldFailButPassed { name: self.name.clone() }
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