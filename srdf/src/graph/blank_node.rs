#[derive(Debug, PartialEq)]
pub struct SBNode<'a> {
    s: &'a str,
}

impl<'a> SBNode<'a> {
    pub fn from(s: &'a str) -> SBNode<'a> {
        SBNode { s }
    }
}

impl<'a> BlankNode<'a> for SBNode<'a> {
    fn label(&self) -> &'a str {
        self.s
    }
}