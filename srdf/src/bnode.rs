use std::fmt::{Debug, Display};

pub trait BlankNode: Debug + Display + PartialEq {
    fn new(id: impl Into<String>) -> Self;
    fn id(&self) -> &str;
}

pub trait BNodeRef<'a> {
    fn label(&self) -> &'a str;
}

#[derive(Debug, PartialEq)]
pub struct SBNode<'a> {
    s: &'a str,
}

impl<'a> SBNode<'a> {
    pub fn from(s: &'a str) -> SBNode<'a> {
        SBNode { s }
    }
}

impl<'a> BNodeRef<'a> for SBNode<'a> {
    fn label(&self) -> &'a str {
        self.s
    }
}
