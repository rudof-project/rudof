use std::{ops::Add, fmt};

pub trait IRI {
//    fn to_string(&self) -> String ;
}

#[derive(Debug, Clone, PartialEq)]
pub struct IriS {
    s: String
}
impl IriS {

    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }

    pub fn from_str(str: &str) -> IriS {
        IriS { s: str.to_owned() }
    }

    pub fn extend(&self, str: &str) -> Self {
        let s = self.s.clone() + str;
        IriS { s: s }
    }

}
impl fmt::Display for IriS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"<{}>", self.s)
    }
}

impl Add for IriS {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        IriS {
            s: self.s + other.s.as_str()
        }
    }

}

impl IRI for IriS {
    
    /* fn to_string(&self) -> String { 
        self.s.clone()
    }*/
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/")  ;
        assert_eq!(iri.to_string(), "<http://example.org/>");
    }

}

