use crate::Context;
use crate::Key;
use crate::Ref;
use crate::State;
use crate::Value;
use crate::rbe_error::RbeError;
use crate::rbe1::Rbe;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

type RbeAndRbeError<K, V, R, Ctx, St> = (Box<Rbe<K, V, R, Ctx, St>>, RbeError<K, V, R, Ctx, St>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fs: Vec<RbeAndRbeError<K, V, R, Ctx, St>>,
}

impl<K, V, R, Ctx, St> Failures<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    pub fn new() -> Self {
        Self { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: Rbe<K, V, R, Ctx, St>, err: RbeError<K, V, R, Ctx, St>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl<K, V, R, Ctx, St> Default for Failures<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, R, Ctx, St> Display for Failures<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (expr, err) in &self.fs {
            writeln!(dest, "Error at {expr}: {err}")?;
        }
        Ok(())
    }
}
