use std::path::Path;
use std::process::Command;

fn main() {
    let test_suite = "https://github.com/shexSpec/test-suite/raw/gh-pages/tests.tar.gz";
    let target_folder = "target/test_suite";
    let target_name = "target/test_suite/tests.tar.gz";
    if Path::new(target_folder).exists() {
        panic!("Test-suite folder already exists");
    }
    let _download = Command::new("curl")
        .arg("--create-dirs")
        .arg("-O")
        .arg("--output-dir")
        .arg(target_folder)
        .arg(test_suite)
        .spawn()
        .expect(format!("curl failed to download file from {test_suite}").as_str())
        .wait_with_output()
        .unwrap();

    let _uncompress = Command::new("gunzip")
        .arg(target_name)
        .spawn()
        .expect(format!("gunzip failed to uncompress {target_name}").as_str())
        .wait_with_output()
        .unwrap();

}
