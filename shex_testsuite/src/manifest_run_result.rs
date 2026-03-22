use crate::manifest_error::ManifestError;
use std::{any::Any, collections::HashMap};

#[derive(Debug)]
pub struct ManifestRunResult {
    pub passed: Vec<String>,
    pub skipped: Vec<String>,
    pub failed: Vec<(String, ManifestError)>,
    pub panicked: Vec<(String, Box<dyn Any + Send + 'static>)>,
    pub traits_passed: HashMap<String, Vec<String>>,
    pub traits_skipped: HashMap<String, Vec<String>>,
    pub traits_failed: HashMap<String, Vec<String>>,
    pub traits_panicked: HashMap<String, Vec<String>>,
}

impl Default for ManifestRunResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ManifestRunResult {
    pub fn new() -> ManifestRunResult {
        ManifestRunResult {
            passed: Vec::new(),
            skipped: Vec::new(),
            failed: Vec::new(),
            panicked: Vec::new(),
            traits_passed: HashMap::new(),
            traits_failed: HashMap::new(),
            traits_skipped: HashMap::new(),
            traits_panicked: HashMap::new(),
        }
    }

    pub fn add_passed(&mut self, name: String, traits: Vec<String>) -> &Self {
        self.passed.push(name.clone());
        for trait_ in traits {
            self.traits_passed.entry(trait_).or_default().push(name.clone());
        }
        self
    }

    pub fn add_skipped(&mut self, name: String, traits: Vec<String>) -> &Self {
        self.skipped.push(name.clone());
        for trait_ in traits {
            self.traits_skipped.entry(trait_).or_default().push(name.clone());
        }
        self
    }

    pub fn add_failed(&mut self, name: String, err: ManifestError, traits: Vec<String>) -> &Self {
        self.failed.push((name.clone(), err));
        for trait_ in traits {
            self.traits_failed.entry(trait_).or_default().push(name.clone());
        }
        self
    }

    pub fn add_panicked(&mut self, name: String, err: Box<dyn Any + Send + 'static>, traits: Vec<String>) -> &Self {
        self.panicked.push((name.clone(), err));
        for trait_ in traits {
            self.traits_panicked.entry(trait_).or_default().push(name.clone());
        }
        self
    }
}
