/// Declares one test case per test-suite file, each handed to `$runner`.
///
/// ```ignore
/// suite_tests! {
///     crate::test_runner,
///     "path/to/file",
///     file_001 => "file-001",
///     #[ignore = "not implemented yet"]
///     file_002 => "file-002,
/// }
/// ```
macro_rules! suite_tests {
    (
        $runner:path,
        $dir:literal,
        $($(#[$attr:meta])* $name:ident => $file:literal,)+
    ) => {
        $(
            #[test]
            $(#[$attr])*
            fn $name() -> Result<(), $crate::common::TestSuiteError> {
                $runner(format!("{}{}.ttl", $dir, $file))
            }
        )+
    };
}
