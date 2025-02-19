use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Values<K, V>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
{
    values: Vec<(K, V)>,
}

impl<K, V> Values<K, V>
where
    K: Hash + Eq + Display + Default + Clone,
    V: Hash + Default + Eq + Display + Clone,
{
    pub fn from(values: &[(K, V)]) -> Values<K, V> {
        Values {
            values: values.to_vec(),
        }
    }
}

impl<K, V> Display for Values<K, V>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (iri, o) in &self.values {
            write!(f, "{iri} {o}|")?;
        }
        Ok(())
    }
}
