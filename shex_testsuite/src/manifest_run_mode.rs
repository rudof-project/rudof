use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ManifestRunMode {
    CollectErrors,
    FailFirstError,
}
