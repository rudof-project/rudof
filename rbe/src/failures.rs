use crate::Context;
use crate::Key;
use crate::Ref;
use crate::Value;
use crate::match_cond::MatchKind;
use crate::rbe_cond::RbeCond;
use crate::rbe_error::RbeError;
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

type RbeAndRbeError<K, V, R, Ctx, P> = (Box<RbeCond<K, V, R, Ctx, P>>, RbeError<K, V, R, Ctx, P>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fs: Vec<RbeAndRbeError<K, V, R, Ctx, P>>,
}

impl<K, V, R, Ctx, P> Failures<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    pub fn new() -> Self {
        Self { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: RbeCond<K, V, R, Ctx, P>, err: RbeError<K, V, R, Ctx, P>) {
        self.fs.push((Box::new(expr), err));
    }

    /// Renders these failures the same way `Display` does, except every key
    /// and value is rendered through the caller-supplied closures instead
    /// of `Display`. Lets a caller with more context (e.g. a `PrefixMap`)
    /// show qualified names instead of full IRIs.
    pub fn show_qualified(&self, show_key: &impl Fn(&K) -> String, show_value: &impl Fn(&V) -> String) -> String {
        self.fs
            .iter()
            .map(|(expr, err)| {
                format!(
                    "Error at {}: {}\n",
                    expr.show_qualified(show_key, show_value),
                    err.show_qualified(show_key, show_value)
                )
            })
            .collect()
    }
}

impl<K, V, R, Ctx, P> Default for Failures<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, R, Ctx, P> Display for Failures<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (expr, err) in &self.fs {
            writeln!(dest, "Error at {expr}: {err}")?;
        }
        Ok(())
    }
}
