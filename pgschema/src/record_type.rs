use either::Either::{self, Left};
use tracing::debug;

use crate::evidence::Evidence;
use crate::key::Key;
use crate::pgs_error::PgsError;
use crate::record::Record;
use crate::value_type::ValueType;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordType {
    map: BTreeMap<Key, ValueType>,
    open: bool,
}

impl RecordType {
    pub fn keys(&self) -> HashSet<&Key> {
        self.map.keys().collect()
    }
    pub fn new() -> Self {
        RecordType {
            map: BTreeMap::new(),
            open: false,
        }
    }

    pub fn with_open(mut self) -> Self {
        self.open = true;
        self
    }

    /// Creates an empty RecordType.
    pub fn empty() -> Self {
        RecordType::new()
    }

    /// Creates a RecordType with a single key-value pair.
    pub fn with_key_value(mut self, key: &str, value_type: ValueType) -> Self {
        self.map.insert(Key::new(key), value_type);
        self
    }

    /// Checks if the RecordType conforms to the given Record.
    pub fn conforms(&self, record: &Record) -> Either<Vec<PgsError>, Vec<Evidence>> {
        debug!(
            "Checking conformance of record: {} with {} open? {}",
            record, self, self.open
        );
        let record_keys = record.keys();
        let record_type_keys = self.keys();
        let (missing_record_type_keys, extra_record_keys) =
            compare_keys(&record_type_keys, &record_keys);
        debug!(
            "Missing record type keys: {:?}, Extra record keys: {:?}",
            missing_record_type_keys, extra_record_keys
        );
        if non_empty(&missing_record_type_keys) {
            debug!(
                "No conforms with missing keys: {:?}",
                missing_record_type_keys
            );
            return Left(vec![PgsError::MissingKeys {
                keys: format!("{:?}", missing_record_type_keys),
                record_type: self.to_string(),
            }]);
        }
        if non_empty(&extra_record_keys) && !self.open {
            debug!(
                "No conforms with extra keys: {:?} and not open",
                extra_record_keys
            );
            return Left(vec![PgsError::ExtraKeysNotOpen {
                keys: format!("{:?}", extra_record_keys),
                record_type: self.to_string(),
            }]);
        }
        for (key, value_type) in self.map.iter() {
            if let Some(value_set) = record.get(key) {
                let result = value_type.conforms(value_set);
                if result.is_left() {
                    debug!(
                        "Value type doesn't conform: {}, result: {:?}",
                        value_type, result
                    );
                    return result;
                } else {
                    debug!("Value {:?} conforms to type {value_type}", value_set);
                    // If the value type conforms, we can collect evidence
                    // let evidence = result.right().unwrap();
                    continue;
                }
            } else {
                // In principle, this should not happen as the keys are checked first
                // but we handle it anyway
                if !self.open {
                    return Either::Left(vec![PgsError::KeyNotFoundClosedRecordType {
                        key: key.clone(),
                        record_type: self.to_string(),
                    }]); // Key not found in RecordType and open is false
                } else {
                    // If open, we can ignore keys not in RecordType
                    continue;
                }
            }
        }
        Either::Right(vec![])
    }

    pub fn combine(&self, other: &RecordType) -> Self {
        let mut result = RecordType::new();
        for (key, value_type) in &self.map {
            if let Some(other_value_type) = other.map.get(key) {
                let combined_type =
                    ValueType::intersection(value_type.clone(), other_value_type.clone());
                result.map.insert(key.clone(), combined_type);
            } else {
                result.map.insert(key.clone(), value_type.clone());
            }
        }
        for (key, value_type) in &other.map {
            if !self.map.contains_key(key) {
                result.map.insert(key.clone(), value_type.clone());
            }
        }
        result
    }

    /// Inserts a key-value pair into the RecordType.
    pub fn insert(&mut self, key: Key, value_type: ValueType) {
        self.map.insert(key, value_type);
    }
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries: Vec<String> = self
            .map
            .iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        write!(f, "{{{}}}", entries.join(", "))
    }
}

fn compare_keys(
    record_type_keys: &HashSet<&Key>,
    record_keys: &HashSet<&Key>,
) -> (HashSet<Key>, HashSet<Key>) {
    let mut missing_record_type_keys = HashSet::new();
    let mut extra_record_keys = HashSet::new();
    for key in record_type_keys {
        if record_keys.contains(key) {
            continue;
        } else {
            missing_record_type_keys.insert((*key).clone());
        }
    }
    for key in record_keys {
        if !record_type_keys.contains(key) {
            extra_record_keys.insert((*key).clone());
        }
    }
    (missing_record_type_keys, extra_record_keys)
}

fn non_empty<T>(set: &HashSet<T>) -> bool {
    !set.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::{card::Card, value::Value, value_type::ValueType};

    use super::*;

    #[test]
    fn test_record_type_conforms() {
        let mut record_type = RecordType::new();
        record_type
            .map
            .insert(Key::new("name"), ValueType::string(Card::One));
        record_type
            .map
            .insert(Key::new("age"), ValueType::integer(Card::One));

        let mut record = Record::new();
        record.insert(Key::new("name"), Value::str("Alice"));
        record.insert(Key::new("age"), Value::int(42));

        assert!(record_type.conforms(&record).is_right());
    }

    #[test]
    fn test_record_type_not_conforms() {
        let record_type = RecordType::new().with_key_value("name", ValueType::string(Card::One));
        let record = Record::new().with_key_value("name", Value::int(42));
        assert!(record_type.conforms(&record).is_left());
    }

    #[test]
    fn test_record_type_missing_keys() {
        let record_type = RecordType::new()
            .with_key_value("name", ValueType::string(Card::One))
            .with_key_value("age", ValueType::integer(Card::One));
        let record = Record::new().with_key_value("name", Value::str("Alice"));
        assert!(record_type.conforms(&record).is_left());
    }

    #[test]
    fn test_record_type_name_age_not_conforms() {
        let record_type = RecordType::new()
            .with_key_value("name", ValueType::string(Card::One))
            .with_key_value("age", ValueType::integer(Card::One));

        let record = Record::new()
            .with_key_value("name", Value::str("Alice"))
            .with_key_value("age", Value::str("Other"));
        assert!(record_type.conforms(&record).is_left());
    }
}
