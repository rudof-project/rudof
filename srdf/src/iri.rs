
pub trait IRI {
    fn from(str: String) -> Self
     where Self: Sized ;
}

#[derive(Debug, PartialEq)]
pub struct MyIRI {
    txt: String 
}

impl IRI for MyIRI {
    fn from(txt: String) -> MyIRI {
        MyIRI { txt: txt }
    }
}

