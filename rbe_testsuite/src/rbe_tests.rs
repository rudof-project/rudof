#[cfg(test)]
mod tests {
    use anyhow::{Context, Result, bail};
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use serde::{Deserialize, Serialize};

    use crate::{RbeTest, RbeTestResult, RbeTestsResults};
    use indoc::indoc;
    use rbe::{Bag, Max, rbe::Rbe};

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

        pub fn with_tests(&mut self, tests: Vec<RbeTest>) {
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

        /*pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
            let path = path.as_ref();
            let data =
                fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
            let group_name = path
                .file_stem()
                .with_context(|| format!("failed to get file name of {}", path.display()))?
                .to_str()
                .with_context(|| format!("invalid UTF-8 found in {}", path.display()))?;
            self.load_slice(group_name, &data)
                .with_context(|| format!("error loading {}", path.display()))?;
            Ok(())
        }*/

        /// Load all of the encoded tests in `data` into this collection.
        /// The given group name is assigned to all loaded tests.
        pub fn load_slice(&mut self, group_name: &str, data: &[u8]) -> Result<()> {
            let data = std::str::from_utf8(data).with_context(|| format!("data in {group_name} is not valid UTF-8"))?;
            let mut index = 1;
            let mut tests: RbeTests =
                serde_json::from_str(data).with_context(|| format!("error decoding JSON for '{group_name}'"))?;
            for t in &mut tests.tests {
                t.set_group(group_name.to_string());
                if t.name().is_empty() {
                    t.set_name(format!("{index}"));
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

    // The following test is useful to check serializations but is not a proper test
    // Remember, to see the serliazation, you need to run the test with `cargo test -- --nocapture`
    #[test]
    fn check_serialization() {
        let values = vec![
            Rbe::symbol("a".to_string(), 1, Max::IntMax(1)),
            Rbe::symbol("b".to_string(), 2, Max::IntMax(3)),
        ];
        let mut rbe_test = RbeTest::default();
        rbe_test.set_group("test".to_string());
        rbe_test.set_name("basic".to_string());
        rbe_test.set_full_name("test/basic".to_string());
        rbe_test.set_rbe(Rbe::and(values));
        rbe_test.set_bag(Bag::from(["a".to_string(), "b".to_string()]));
        let ts = vec![rbe_test];
        let mut rbe_tests = RbeTests::default();
        rbe_tests.with_tests(ts);
        let serialized = serde_json::to_string_pretty(&rbe_tests).unwrap();
        println!("Serialized: {serialized}");
        assert_eq!(serialized.is_empty(), false);
    }

    #[test]
    fn load_slice_1() {
        let str = indoc! {r#"{ "tests": [
    {
      "name": "basic",
      "rbe": {
        "And": {
          "values": [
            {
              "Symbol": {
                "value": "a",
                "card": {
                  "min": 1,
                  "max": 1
                }
              }
            },
            {
              "Symbol": {
                "value": "b",
                "card": {
                  "min": 1,
                  "max": 3
                }
              }
            }
          ]
        }
      },
      "bag": [
        [
          "b",
          1
        ],
        [
          "a",
          1
        ]
      ],
      "open": false,
      "match_result": "Pass"
    }
  ]
}
        "#};
        let mut tests = RbeTests::new();
        tests.load_slice("test", str.as_bytes()).unwrap();
        let t0 = &tests.tests().next().unwrap();
        assert_eq!("test", t0.group());
        assert_eq!(RbeTestResult::passed("basic".to_string()), t0.run());
    }

    // Runs all the tests
    #[test]
    fn run_test_suite() {
        let data = include_bytes!("../tests/basic.json");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        //let json = serde_json::to_string_pretty(&rbe_tests).unwrap();
        // println!("Loaded tests: {}", json);
        let results = rbe_tests.run();
        for t in results.failed() {
            tracing::info!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), rbe_tests.total());
        assert_eq!(results.count_failed(), 0);
    }

    // The following test can be use to check a single test case
    #[test]
    fn run_single() {
        let name = "a_1_1_with_a_2_fail".to_string();
        println!("Running single test: {name}");
        let data = include_bytes!("../tests/basic.json");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run_by_name(name);
        for t in results.failed() {
            println!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), 1);
        assert_eq!(results.count_failed(), 0);
    }
}
