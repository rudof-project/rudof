use crate::State;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Values<K, V, Ctx, St>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
    St: State,
{
    values: Vec<(K, V, Ctx)>,
    #[serde(skip)]
    _phantom: PhantomData<St>,
}

impl<K, V, Ctx, St> Values<K, V, Ctx, St>
where
    K: Hash + Eq + Display + Default + Clone,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
    St: State,
{
    pub fn from(values: &[(K, V, Ctx)]) -> Values<K, V, Ctx, St> {
        Values {
            values: values.to_vec(),
            _phantom: PhantomData,
        }
    }
}

impl<K, V, Ctx, St> Display for Values<K, V, Ctx, St>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Display + Clone,
    Ctx: Display + Clone,
    St: State,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (iri, o, ctx) in &self.values {
            write!(f, "{iri} {o} {ctx}|")?;
        }
        Ok(())
    }
}
