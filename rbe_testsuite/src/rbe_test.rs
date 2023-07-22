use rbe::{Rbe, Bag, RbeError};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashSet, fs};
use std::hash::Hash;
use std::fmt;
use std::path::Path;
use anyhow::{bail, Context, Result};

use crate::{TestType, MatchResult, RbeTestResult, PassedTestResult, FailedTestResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    /// Runs this test
    pub fn run(&self) -> RbeTestResult {
        match (&self.match_result, self.rbe.match_bag(&self.bag, self.open)) {
          (MatchResult::BooleanResult(true), Ok(())) => {
             RbeTestResult::passed(self.name().to_string())
           },
           (MatchResult::BooleanResult(true), Err(err)) => {
             RbeTestResult::failed(self.name().to_string(),err) 
           }
           (MatchResult::BooleanResult(false), Ok(())) => {
            RbeTestResult::failed(
                self.name().to_string(), 
                RbeError::ShouldFailButPassed { name: self.name.clone() }
            )},
           (MatchResult::BooleanResult(false), Err(_)) => {
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
