
#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn list_all_files() {
        const TESTS_FOLDER = "testsuite/tests"
        let paths = fs::read_dir("./").unwrap();
        
        for path in paths {
         println!("Name: {}", path.unwrap().path().display())
        }

        assert_eq!(2 + 2, 4);
    }
}
