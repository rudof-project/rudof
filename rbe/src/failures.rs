use crate::rbe1::Rbe;
use crate::rbe_error::RbeError;
use crate::Key;
use crate::Ref;
use crate::Value;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

type RbeAndRbeError<K, V, R> = (Box<Rbe<K, V, R>>, RbeError<K, V, R>);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fs: Vec<RbeAndRbeError<K, V, R>>,
}

impl<K, V, R> Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new() -> Self {
        Self { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: Rbe<K, V, R>, err: RbeError<K, V, R>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl<K, V, R> Default for Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, R> Display for Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (expr, err) in &self.fs {
            writeln!(dest, "Error at {expr}: {err}")?;
        }
        Ok(())
    }
}
