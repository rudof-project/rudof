pub mod rbe_test;
pub mod rbe_tests;
pub mod rbe_test_result;
pub mod rbe_test_results;
pub mod match_result;

pub use rbe_test::*;
pub use rbe_tests::*;
pub use rbe_test_result::*;
pub use rbe_test_results::*;
pub use match_result::*;

type KeyType = String;
type ValueType = String;
type RefType = String;


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use log::*;
    use rbe::{rbe::Rbe, Max, Bag};
    use test_log::test;
    use super::*;

    #[test]
    fn basic_test() {
        let str = indoc!{
          r#"name: basic
             rbe: !Symbol
              key: foo
              cond: {}
              card:
                min: 1
                max: !IntMax 1
             bag:
               - - foo
                 - 1
             open: false
             match_result: !Pass
            "#};
      let rbe_test: RbeTest = serde_yaml::from_str(str).unwrap();
      assert_eq!(rbe_test.run(), RbeTestResult::passed("basic".to_string()))
   }

   // The following test is useful to generate YAML serializations but is not a proper test
   #[test]
   fn check_serialization() {

    let values = vec![
      Rbe::symbol("a".to_string(), 1, Max::IntMax(1)),
      Rbe::symbol("b".to_string(), 2, Max::IntMax(3))
      ];
    let mut rbe_test = RbeTest::default();
    rbe_test.set_group("test".to_string());
    rbe_test.set_name("basic".to_string());
    rbe_test.set_full_name("test/basic".to_string());
    rbe_test.set_rbe(Rbe::and(values.into_iter()));
    rbe_test.set_bag(vec![("a".to_string(), "23".to_string()),
                          ("b".to_string(), "44".to_string())
                          ]);
    let mut ts = Vec::new();
    ts.push(rbe_test);
    let mut rbe_tests = RbeTests::default();
    rbe_tests.with_tests(ts);
    let serialized = serde_yaml::to_string(&rbe_tests).unwrap();
    println!("---\n{serialized}");
    assert_eq!(serialized.len() > 0, true);
  } 

    #[test]
    fn load_slice_1() {
        let str = indoc! {r#"
        tests:
        - name: basic
          rbe: !Symbol
            key: foo
            cond: {}
            card:
              min: 1
              max: 1
          bag:
          - - foo
            - 1
          open: false
          match_result: !Pass
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
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run();
        for t in results.failed() {
           info!("Failed: {}: error: {}", t.name(), t.err());
        }
        let failed: Vec<&FailedTestResult> = results.failed().collect();
        let empty: Vec<&FailedTestResult> = Vec::new();
        assert_eq!(failed, empty);
        assert_eq!(results.count_passed(), rbe_tests.total());
        assert_eq!(results.count_failed(), 0);
    }

    // The following test can be use to check a single test case
    #[test]
    fn run_single() {
        let name = "a_1_1_with_a_2_fail".to_string();
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