use std::path::Path;
use crate::common::manifest::NodeExprManifest;
use crate::common::TestSuiteError;

/// Runs every `sht:EvalNodeExpr` entry of a node expression test manifest
pub(crate) fn node_expr_runner(path: String) -> Result<(), TestSuiteError> {
    let mut manifest = NodeExprManifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let actual = manifest.evaluate(&test)?;

        let equals = if test.ignore_order {
          let mut actual = actual
              .iter()
              .map(|t| t.to_string())
              .collect::<Vec<_>>();
          let mut expected = test
              .expected
              .iter()
              .map(|t| t.to_string())
              .collect::<Vec<_>>();
            actual.sort();
            expected.sort();
            actual == expected
        } else {
            actual == test.expected
        };

        if !equals {
            println!("❌ {}", test.label);
            println!("Expected output nodes:\n{:#?}", test.expected);
            println!("Actual output nodes:\n{actual:#?}");
            return Err(TestSuiteError::NotEquals);
        }
    }
    
    Ok(())

}