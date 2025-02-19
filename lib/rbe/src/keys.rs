use serde_derive::Deserialize;
use serde_derive::Serialize;
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
        Keys {
            keys: keys.to_vec(),
        }
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
