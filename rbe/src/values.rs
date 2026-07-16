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

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Renders these values the same way `Display` does, except every key
    /// and value is rendered through the caller-supplied closures instead
    /// of `Display`. Lets a caller with more context (e.g. a `PrefixMap`)
    /// show qualified names instead of full IRIs.
    pub fn show_qualified(&self, show_key: &impl Fn(&K) -> String, show_value: &impl Fn(&V) -> String) -> String {
        self.values
            .iter()
            .map(|(iri, o, ctx)| format!("{} {} {ctx}|", show_key(iri), show_value(o)))
            .collect()
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
