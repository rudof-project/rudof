pub mod rbe_test;
pub mod rbe_tests;
pub mod rbe_test_result;
pub mod rbe_test_results;


pub use rbe_test::*;
pub use rbe_tests::*;
pub use rbe_test_result::*;
pub use rbe_test_results::*;
use serde_derive::{Serialize, Deserialize};

/// TODO: I would prefer this type to be a String or &str, but it must be Eq, Hash, Clone and with &str I have some lifetime issues...
type TestType = String;




#[derive(Clone, Debug, Serialize, Deserialize)]
enum MatchResult {
    BooleanResult(bool)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use log::*;
    use test_log::test;
    use super::*;

    #[test]
    fn basic_test() {
        let str = 
r#"name: basic
rbe: !Symbol
  value: foo
  card:
    min: 1
    max: !IntMax 1
bag:
- - foo
  - 1
open: false
match_result: !BooleanResult true
"#;
      let rbe_test: RbeTest = serde_yaml::from_str(str).unwrap();
      assert_eq!(rbe_test.run(), RbeTestResult::passed("basic".to_string()))
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

    #[test_log::test]
    fn load_slice_1() {

        let str = indoc! {r#"
        tests:
        - name: basic
          rbe: !Symbol
            value: foo
            card:
              min: 1
              max: !IntMax 1
          bag:
          - - foo
            - 1
          open: false
          match_result: !BooleanResult true"#
        };
        let mut tests = RbeTests::new();
        tests.load_slice("test", str.as_bytes()).unwrap();
        let t0 = &tests.tests().next().unwrap();
        assert_eq!("test", t0.group());
        assert_eq!(RbeTestResult::passed("basic".to_string()), t0.run());
    }

    #[test]
    fn run_test_suite() {
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run();
        for t in results.failed() {
           info!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), rbe_tests.total());
        assert_eq!(results.count_failed(), 0);
    }

    #[test]
    fn run_single() {
        let name = "a_1_u_with_a_1_b_1_pass".to_string();
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run_by_name(name);
        for t in results.failed() {
           info!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), 1);
        assert_eq!(results.count_failed(), 0);
    }    

}