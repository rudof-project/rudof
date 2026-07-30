use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Keys<K>
where
    K: Display,
{
    keys: Vec<K>,
}

impl<K> Keys<K>
where
    K: Eq + Display + Default + Clone,
{
    pub fn from(keys: &[K]) -> Keys<K> {
        Keys { keys: keys.to_vec() }
    }

    /// Renders these keys the same way `Display` does, except each key is
    /// rendered through `show_key` instead of `Display`. Lets a caller with
    /// more context (e.g. a `PrefixMap`) show qualified names instead of
    /// full IRIs.
    pub fn show_qualified(&self, show_key: &impl Fn(&K) -> String) -> String {
        self.keys.iter().map(|k| format!("{}|", show_key(k))).collect()
    }
}

impl<K: Display> Display for Keys<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in &self.keys {
            write!(f, "{k}|")?;
        }
        Ok(())
    }
}
