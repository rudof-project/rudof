use std::{fmt::Display, ops::Deref};

use crate::{Component, Context, Key, MatchCond, Ref, State, Value};

type CandidateItem<K, V, R, Ctx, St> = (K, V, Component, MatchCond<K, V, R, Ctx, St>);

// TODO: We are not using the struct yet
#[derive(Debug, Clone)]
pub struct Candidate<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    values: Vec<CandidateItem<K, V, R, Ctx, St>>,
}

impl<K, V, R, Ctx, St> Display for Candidate<K, V, R, Ctx, St>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
    St: State + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Candidate")?;
        for (key, value, component, cond) in self.values.iter() {
            write!(f, "Candidate value: {key} {value} {component} {cond}")?;
        }
        Ok(())
    }
}

/*impl <K,V,R> IntoIterator for Candidate<K,V,R>
where
  K: Key + Display,
  V: Value + Display,
  R: Ref + Display, {
    type Item = (K, V, Component, MatchCond<K, V, R>);

    type IntoIter = ;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}*/

impl<K, V, R, Ctx, St> Deref for Candidate<K, V, R, Ctx, St>
where
    K: Key + Display,
    V: Value + Display,
    R: Ref + Display,
    Ctx: Context + Display,
    St: State + Display,
{
    type Target = Vec<(K, V, Component, MatchCond<K, V, R, Ctx, St>)>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
