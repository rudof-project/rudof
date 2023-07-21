use rbe::{Rbe, Bag, RbeError};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashSet, fs};
use std::hash::Hash;
use std::fmt;
use std::path::Path;
use anyhow::{bail, Context, Result};

/// TODO: I would prefer this type to be a String or &str, but it must be Eq, Hash, Clone and with &str I have some lifetime issues...
type TestType = i32;

/// A collection of rbe tests.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RbeTests
// where T: Hash + Eq + Clone + fmt::Debug
{
    // #[serde(default, rename = "test", borrow)]
    tests: Vec<RbeTest>,
    
    #[serde(skip)]
    visited: HashSet<String>,
}

impl RbeTests
// where T: Hash + Eq + Clone + fmt::Debug 
{
    pub fn new() -> RbeTests {
        RbeTests {
            tests: Vec::new(),
            visited: HashSet::new()
        }
    }

    pub fn run(&self) -> RbeTestsResults {
        let mut results = RbeTestsResults::new();
        for test in &self.tests {
          let result = test.run();
          results.add_result(&result);
       }
       results
    }

    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let data = fs::read(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let group_name = path
            .file_stem()
            .with_context(|| {
                format!("failed to get file name of {}", path.display())
            })?
            .to_str()
            .with_context(|| {
                format!("invalid UTF-8 found in {}", path.display())
            })?;
        self.load_slice(&group_name, &data)
            .with_context(|| format!("error loading {}", path.display()))?;
        Ok(())
    }

    /// Load all of the encoded tests in `data` into this collection.
    /// The given group name is assigned to all loaded tests.
    pub fn load_slice(&mut self, group_name: &str, data: &[u8]) -> Result<()> {
        let data = std::str::from_utf8(&data).with_context(|| {
            format!("data in {} is not valid UTF-8", group_name)
        })?;
        let mut index = 1;
        let mut tests: RbeTests =
            serde_yaml::from_str(&data).with_context(|| {
                format!("error decoding YAML for '{}'", group_name)
            })?;
        for t in &mut tests.tests {
            t.group = group_name.to_string();
            if t.name.is_empty() {
                t.name = format!("{}", index);
                index += 1;
            }
            t.full_name = format!("{}/{}", t.group, t.name);
            if self.visited.contains(t.full_name()) {
                bail!("found duplicate tests for name '{}'", t.full_name());
            }
            self.visited.insert(t.full_name().to_string());
        }

        self.tests.extend(tests.tests);
        Ok(())
    }

}

pub struct RbeTestsResults {
   results: Vec<RbeTestResult>,
   pub passed: usize, 
   pub failed: usize
}

#[derive(PartialEq, Debug, Clone)]
pub enum RbeTestResult {
    Passed { name: String },
    Failed { name: String, error: RbeError<TestType> }
}

impl RbeTestsResults {
    fn new() -> RbeTestsResults {
        RbeTestsResults {
        results: Vec::new(),
        passed: 0,
        failed: 0
    }
   }

   fn add_result(&mut self, result: &RbeTestResult) {
     self.results.push(result.clone());
     match &result {
        RbeTestResult::Passed { .. } => { self.passed += 1 },
        RbeTestResult::Failed { .. } => { self.failed += 1},
    }
   }
   
}

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

    /// The name of this test.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Runs this test
    pub fn run(&self) -> RbeTestResult {
        match (&self.match_result, self.rbe.match_bag(&self.bag, self.open)) {
          (MatchResult::BooleanResult(true), Ok(())) => {
             RbeTestResult::Passed { name: self.name().to_string() }
           },
           (MatchResult::BooleanResult(true), Err(err)) => {
                    RbeTestResult::Failed { name: self.name().to_string(), error: err }
           }
           (MatchResult::BooleanResult(false), Ok(())) => {
            todo!();
           },
           (MatchResult::BooleanResult(false), Err(err)) => {
            RbeTestResult::Passed { name: self.name().to_string() }
           }
        } 
    }

    /// The full name of this test, which is formed by joining the group
    /// name, the test name and the additional name with a `/`.
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum MatchResult {
    BooleanResult(bool)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn basic_test() {
        let str = 
r#"name: basic
rbe: !Symbol
  value: 22
  card:
    min: 1
    max: !IntMax 1
bag:
- - 22
  - 1
open: false
match_result: !BooleanResult true
"#;
      let rbe_test: RbeTest = serde_yaml::from_str(str).unwrap();
      assert_eq!(rbe_test.run(), RbeTestResult::Passed{ name: "basic".to_string() })
   }

   /*
   #[test]
   fn check_serialization() {
    let rbe_test: RbeTest<i32> = RbeTest {
        group: "test".to_string(),
        name: "basic".to_string(),
        full_name: "test/basic".to_string(),
        rbe: Rbe::symbol(22, 1, Max::IntMax(1)),
        bag: Bag::from([22]),
        open: false,
        match_result: MatchResult::BooleanResult(true),
    };
    let mut ts = Vec::new();
    ts.push(rbe_test);
    let rbe_tests = RbeTests {
        tests: ts,
        visited: HashSet::new()
    };
    let serialized = serde_yaml::to_string(&rbe_tests).unwrap();
    println!("---\n{serialized}");
    assert_eq!(serialized, "".to_string());
  } */


    #[test]
    fn load_slice_1() {

        let str = indoc! {r#"
        tests:
        - name: basic
          rbe: !Symbol
            value: 22
            card:
              min: 1
              max: !IntMax 1
          bag:
          - - 22
            - 2
          open: false
          match_result: !BooleanResult false"#
        };
        let mut tests = RbeTests::new();
        tests.load_slice("test", str.as_bytes()).unwrap();
        let t0 = &tests.tests[0];
        assert_eq!("test", t0.group());
        assert_eq!(RbeTestResult::Passed{ name: "basic".to_string()}, t0.run())
    }

    #[test]
    fn run_test_suite() {
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run();
        assert_eq!(results.passed, 2);
        assert_eq!(results.failed, 0);
    }

}