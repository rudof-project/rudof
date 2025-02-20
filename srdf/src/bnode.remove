pub trait BNode<'a> {
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

impl<'a> BNode<'a> for SBNode<'a> {
    fn label(&self) -> &'a str {
        self.s
    }
}
