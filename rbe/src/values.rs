use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Values<K, V, Ctx>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
{
    values: Vec<(K, V, Ctx)>,
}

impl<K, V, Ctx> Values<K, V, Ctx>
where
    K: Hash + Eq + Display + Default + Clone,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
{
    pub fn from(values: &[(K, V, Ctx)]) -> Values<K, V, Ctx> {
        Values {
            values: values.to_vec(),
        }
    }
}

impl<K, V, Ctx> Display for Values<K, V, Ctx>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (iri, o, ctx) in &self.values {
            write!(f, "{iri} {o} {ctx}|")?;
        }
        Ok(())
    }
}
