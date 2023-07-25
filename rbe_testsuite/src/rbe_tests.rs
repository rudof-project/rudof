use std::{collections::HashSet, path::Path, fs};
use anyhow::{bail, Context, Result};

use serde_derive::{Serialize, Deserialize};

use crate::{RbeTest, RbeTestsResults};

/// A collection of rbe tests.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
        RbeTests::default()
    }

    pub fn with_tests(&mut self, tests:Vec<RbeTest>) {
        // TODO: Add visited
        self.tests = tests;
    }

    pub fn tests(&self) -> std::slice::Iter<'_, RbeTest> {
        self.tests.iter()
    }

    pub fn total(&self) -> usize {
        self.tests.len()
    }

    pub fn run(&self) -> RbeTestsResults {
        let mut results = RbeTestsResults::new();
        for test in &self.tests {
          let result = test.run();
          results.add_result(&result);
       }
       results
    }

    pub fn run_by_name(&self, name: String) -> RbeTestsResults {
        let mut results = RbeTestsResults::new();
        for test in &self.tests {
            if test.name() == name {
                let result = test.run();
                results.add_result(&result);
            }
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
            t.set_group(group_name.to_string());
            if t.name().is_empty() {
                t.set_name(format!("{}", index));
                index += 1;
            }
            t.set_full_name(format!("{}/{}", t.group(), t.name()));
            if self.visited.contains(t.full_name()) {
                bail!("found duplicate tests for name '{}'", t.full_name());
            }
            self.visited.insert(t.full_name().to_string());
        }

        self.tests.extend(tests.tests);
        Ok(())
    }

}
