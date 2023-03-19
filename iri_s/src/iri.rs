use std::fmt;
use std::str::FromStr;
use oxiri::{IriRef, IriParseError};

pub trait IRI {
//    fn to_string(&self) -> String ;
}

#[derive(Debug, Clone, PartialEq)]
pub struct IriS {
    s: String,
    iri: IriRef<String>
}
impl IriS {

    pub fn as_str(&self) -> &str {
        self.s.as_str()
    }

    pub fn extend(&self, str: &str) -> Result<Self, IriError> {
        let s = self.s.clone() + str;
        let iri = IriRef::parse(s)?;
        Ok(IriS { s: iri.to_string(), iri: iri })
    }

    pub fn is_absolute(&self) -> bool {
       self.iri.is_absolute()
    }

}

impl fmt::Display for IriS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"<{}>", self.s)
    }
}

#[derive(Debug)]
pub struct IriError {
    msg: String
}


impl FromStr for IriS {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }

}

impl From<IriParseError> for IriError {
    fn from(e: IriParseError) -> Self {
        IriError { msg: format!("IriParserError: {:?}",e.to_string())}
    }
}

fn parse_iri(s:&str) -> Result<IriS, IriError> {
    let iri = IriRef::parse(s.to_owned())?;
    Ok(IriS { s: iri.to_string(), iri: iri })
}


impl IRI for IriS {
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/").unwrap()  ;
        assert_eq!(iri.to_string(), "<http://example.org/>");
    }

}

