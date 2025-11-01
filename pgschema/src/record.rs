use crate::key::Key;
use crate::value::Value;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet as Set;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    map: HashMap<Key, Set<Value>>,
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

impl Record {
    pub fn keys(&self) -> Set<&Key> {
        self.map.keys().collect()
    }
    pub fn new() -> Self {
        Record {
            map: HashMap::new(),
        }
    }

    pub fn with_key_value(mut self, key: &str, value: Value) -> Self {
        self.insert(Key::new(key), value);
        self
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.map.entry(key).or_default().insert(value);
    }

    pub fn insert_values(&mut self, key: Key, values: Set<Value>) {
        self.map.entry(key).or_default().extend(values);
    }

    pub fn get(&self, key: &Key) -> Option<&Set<Value>> {
        self.map.get(key)
    }

    /*pub fn remove(&mut self, key: &Key) -> Option<Set<Value>> {
        self.map.remove(key)
    }*/

    pub fn combine(&self, other: &Record) -> Record {
        let mut combined = Record::new();
        for (key1, values1) in &self.map {
            if let Some(values2) = other.get(key1) {
                let combined_values = values1.intersection(values2).cloned().collect::<Set<_>>();
                combined.insert_values(key1.clone(), combined_values);
            } else {
                combined.insert_values(key1.clone(), values1.clone());
            }
        }
        for (key2, values2) in &other.map {
            if self.get(key2).is_some() {
                continue; // Already handled in the first loop
            }
            combined.insert_values(key2.clone(), values2.clone());
        }
        combined
    }

    pub fn union(&self, other: &Record) -> Record {
        let mut result = Record::new();
        for (key1, values1) in &self.map {
            if let Some(values2) = other.get(key1) {
                let combined_values = values1.union(values2).cloned().collect::<Set<_>>();
                result.insert_values(key1.clone(), combined_values);
            } else {
                result.insert_values(key1.clone(), values1.clone());
            }
        }
        for (key2, values2) in &other.map {
            if self.get(key2).is_some() {
                continue; // Already handled in the first loop
            }
            result.insert_values(key2.clone(), values2.clone());
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Set<Value>)> {
        self.map.iter()
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.map.is_empty() {
            return write!(f, "{{}}");
        }
        let entries: Vec<String> = self
            .map
            .iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(k, v)| format!("{}: {}", k, show_values(v)))
            .collect();
        write!(f, "{{{}}}", entries.join(", "))?;
        Ok(())
    }
}

fn show_values(values: &Set<Value>) -> String {
    if values.is_empty() {
        return "[]".to_string();
    }
    if values.len() == 1 {
        return values.iter().next().unwrap().to_string();
    }
    let sorted_values: Vec<&Value> = values.iter().sorted().collect();
    format!(
        "[{}]",
        sorted_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let mut record1 = Record::new();
        record1.insert(Key::new("k1"), Value::str("v1"));
        record1.insert(Key::new("k1"), Value::str("v2"));
        record1.insert(Key::new("k2"), Value::str("v3"));

        let mut record2 = Record::new();
        record2.insert(Key::new("k1"), Value::str("v2"));
        record2.insert(Key::new("k1"), Value::str("v4"));
        record2.insert(Key::new("k3"), Value::str("v5"));

        let expected = Record {
            map: [
                (Key::new("k1"), Set::from([Value::str("v2")])),
                (Key::new("k2"), Set::from([Value::str("v3")])),
                (Key::new("k3"), Set::from([Value::str("v5")])),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        let combined = record1.combine(&record2);
        assert_eq!(combined, expected);
    }

    #[test]
    fn test_union() {
        let mut record1 = Record::new();
        record1.insert(Key::new("k1"), Value::str("v1"));
        record1.insert(Key::new("k1"), Value::str("v2"));
        record1.insert(Key::new("k2"), Value::str("v3"));

        let mut record2 = Record::new();
        record2.insert(Key::new("k1"), Value::str("v2"));
        record2.insert(Key::new("k1"), Value::str("v4"));
        record2.insert(Key::new("k3"), Value::str("v5"));

        let expected = Record {
            map: [
                (
                    Key::new("k1"),
                    Set::from([Value::str("v2"), Value::str("v1"), Value::str("v4")]),
                ),
                (Key::new("k2"), Set::from([Value::str("v3")])),
                (Key::new("k3"), Set::from([Value::str("v5")])),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        let result = record1.union(&record2);
        assert_eq!(result, expected);
    }
}
