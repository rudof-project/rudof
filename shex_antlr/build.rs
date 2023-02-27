use std::path::Path;
use std::process::Command;

fn main() {
    let antlr_path = "jar/antlr4-4.8-2-SNAPSHOT-complete.jar";
    if !Path::new(antlr_path).exists() {
        panic!("Latest custom ANTLR build is not exists at {antlr_path}. Please download it as described at README.md");
    }

    println!("cargo:rerun-if-changed=grammar/ShExDoc.g4");

    let _command = Command::new("java")
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-visitor")
        .arg("-o src/grammar")
        .arg("-Xlog")
        .arg("grammar/ShExDoc.g4")
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output();
}
