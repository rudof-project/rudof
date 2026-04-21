use std::collections::HashSet;

use crate::manifest_run_result::ManifestRunResult;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum PrintResultMode {
    Basic,
    FailedSimple,
    Failed,
    Passed,
    NotImplemented,
    All,
    TraitsFailed,
    TraitsFailedNames,
    TraitsFailedSkippedNames,
    TraitsPassedAndFailed,
}

fn print_basic(result: &ManifestRunResult) {
    let (npassed, nskipped, nfailed, npanicked) = (
        result.passed.len(),
        result.skipped.len(),
        result.failed.len(),
        result.panicked.len(),
    );
    let overview = format!("Passed: {npassed}, Failed: {nfailed}, Skipped: {nskipped}, Not implemented: {npanicked}",);
    println!("{overview}");
}

fn print_traits_failed(result: &ManifestRunResult) {
    println!("--- Failed with traits ---");
    for (trait_, failed) in &result.traits_failed {
        println!("{trait_}: {}", failed.len());
    }
}

fn print_traits_failed_names(result: &ManifestRunResult) {
    println!("--- Failed with traits ---");
    for (trait_, failed) in &result.traits_failed {
        println!(
            "{trait_}: {}",
            failed.iter().map(|name| name.as_str()).collect::<Vec<_>>().join(", ")
        );
    }
}

fn print_traits_failed_skipped_names(result: &ManifestRunResult) {
    println!("--- Failed with traits ---");
    for (trait_, failed) in &result.traits_failed {
        println!(
            "{trait_}: {}",
            failed.iter().map(|name| name.as_str()).collect::<Vec<_>>().join(", ")
        );
    }
    println!("--- Skipped with traits ---");
    for (trait_, skipped) in &result.traits_skipped {
        println!(
            "{trait_}: {}",
            skipped.iter().map(|name| name.as_str()).collect::<Vec<_>>().join(", ")
        );
    }
}

fn print_traits_passed_and_failed(result: &ManifestRunResult) {
    let traits_passed = result.traits_passed.keys().cloned().collect::<HashSet<_>>();
    let traits_failed = result.traits_failed.keys().cloned().collect::<HashSet<_>>();
    let traits_passed_and_failed = traits_passed.intersection(&traits_failed);
    println!("--- Passed and Failed traits ---");
    for trait_ in traits_passed_and_failed {
        println!("{trait_}");
    }
}

fn print_failed(result: &ManifestRunResult) {
    println!("--- Failed ---");
    for (name, err) in &result.failed {
        println!("{name} {err}");
    }
}

fn print_failed_simple(result: &ManifestRunResult) {
    println!("--- Failed ---");
    let mut sorted_names = result.failed.iter().map(|(name, _)| name).collect::<Vec<&String>>();
    sorted_names.sort();
    for name in &sorted_names {
        println!("{name}");
    }
}

fn print_panicked(result: &ManifestRunResult) {
    println!("--- Not implemented ---");
    for (name, _err) in &result.panicked {
        println!("{name}");
    }
}

fn print_passed(result: &ManifestRunResult) {
    println!("--- Passed ---");
    for name in &result.passed {
        println!("{name}");
    }
}

fn print_result(result: ManifestRunResult, print_result_mode: PrintResultMode) {
    match print_result_mode {
        PrintResultMode::Basic => {
            print_basic(&result);
        },
        PrintResultMode::All => {
            print_passed(&result);
            print_failed(&result);
            print_panicked(&result);
            print_basic(&result);
        },
        PrintResultMode::Failed => {
            print_failed(&result);
            print_basic(&result);
        },
        PrintResultMode::FailedSimple => {
            print_failed_simple(&result);
            print_basic(&result);
        },
        PrintResultMode::Passed => {
            print_passed(&result);
            print_basic(&result);
        },
        PrintResultMode::NotImplemented => {
            print_panicked(&result);
            print_basic(&result);
        },
        PrintResultMode::TraitsFailed => {
            print_traits_failed(&result);
            print_basic(&result);
        },
        PrintResultMode::TraitsFailedNames => {
            print_traits_failed_names(&result);
            print_basic(&result);
        },
        PrintResultMode::TraitsFailedSkippedNames => {
            print_traits_failed_skipped_names(&result);
            print_basic(&result);
        },
        PrintResultMode::TraitsPassedAndFailed => {
            print_traits_passed_and_failed(&result);
            print_basic(&result);
        },
    }
}

impl PrintResultMode {
    pub fn print_result(&self, result: ManifestRunResult) {
        print_result(result, *self);
    }
}
