
pub trait IRI<'a> {
    fn to_str(&self) -> &'a str ;
}

#[derive(Debug, PartialEq)]
pub struct SIRI<'a> {
    s: &'a str
}

impl <'a> SIRI<'a> {
    pub fn from (s: &'a str) -> SIRI<'a> {
        SIRI { s: s }
    }
}

impl <'a> IRI<'a> for SIRI<'a> {
    fn to_str(&self) -> &'a str { self.s }
}


