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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fs: Vec<(Box<Rbe<K, V, R>>, RbeError<K, V, R>)>,
}

impl<K, V, R> Failures<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new() -> Failures<K, V, R> {
        Failures { fs: Vec::new() }
    }

    pub fn push(&mut self, expr: Rbe<K, V, R>, err: RbeError<K, V, R>) {
        self.fs.push((Box::new(expr), err));
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
            write!(dest, "Error at {expr}: {err}\n")?;
        }
        Ok(())
    }
}
