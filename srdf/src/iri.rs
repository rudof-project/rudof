#[derive(Debug, PartialEq)]
pub struct IRI {
    str: String
}


impl IRI {
    pub fn from(txt: String) -> IRI {
        IRI { str: txt }
    }
}