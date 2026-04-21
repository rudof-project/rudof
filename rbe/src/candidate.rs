use std::{fmt::Display, ops::Deref};

use crate::{Component, Context, Key, MatchCond, Ref, Value};

type CandidateItem<K, V, R, Ctx> = (K, V, Component, MatchCond<K, V, R, Ctx>);

// TODO: We are not using the struct yet
#[derive(Debug, Clone)]
pub struct Candidate<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    values: Vec<CandidateItem<K, V, R, Ctx>>,
}

impl<K, V, R, Ctx> Display for Candidate<K, V, R, Ctx>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Candidate")?;
        for (key, value, component, cond) in self.values.iter() {
            write!(f, "Candidate value: {key} {value} {component} {cond}")?;
        }
        Ok(())
    }
}

impl<K, V, R, Ctx> Deref for Candidate<K, V, R, Ctx>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
{
    type Target = Vec<(K, V, Component, MatchCond<K, V, R, Ctx>)>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
