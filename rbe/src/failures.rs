use crate::Context;
use crate::Key;
use crate::Ref;
use crate::Value;
use crate::rbe_cond::RbeCond;
use crate::rbe_error::RbeError;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

type RbeAndRbeError<K, V, R, Ctx> = (Box<RbeCond<K, V, R, Ctx>>, RbeError<K, V, R, Ctx>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fs: Vec<RbeAndRbeError<K, V, R, Ctx>>,
}

impl<K, V, R, Ctx> Failures<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    pub fn new() -> Self {
        Self { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: RbeCond<K, V, R, Ctx>, err: RbeError<K, V, R, Ctx>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl<K, V, R, Ctx> Default for Failures<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, R, Ctx> Display for Failures<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (expr, err) in &self.fs {
            writeln!(dest, "Error at {expr}: {err}")?;
        }
        Ok(())
    }
}
