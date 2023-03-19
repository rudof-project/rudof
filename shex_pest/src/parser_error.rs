use std::fmt::{Display, Formatter};
use iri_s::IriError;
use pest::{error::Error, RuleType};
use pest::iterators::Pair;
use crate::parser::Rule;
use crate::shexc_error::ShExCError;


#[derive(Debug, Clone)]
pub(crate) struct ParserErrorFactory {
    pub(crate) repr: &'static str,
}

impl ParserErrorFactory {
    pub(crate) fn new(name: &'static str) -> Self { 
        ParserErrorFactory { repr: name } 
    }
    
    pub(crate) fn error(&self, fn_name: &str) -> ParserError {
        ParserError {
            repr: self.repr.to_string(),
            fn_name: fn_name.to_string(),
            rule: None,
            expecting: None,
            unreachable: false,
            context: None,
        }
    }

}

#[derive(Debug, Clone)]
pub struct ParserError {
    repr: String,
    fn_name: String,
    rule: Option<String>,
    expecting: Option<String>,
    unreachable: bool,
    context: Option<String>,
}


impl ParserError {
     pub(crate) fn unexpected<T: RuleType>(&mut self, pair: &Pair<'_, T>) -> &mut Self {
        self.context = Some(format!("{:?}: {:?}", pair.as_rule(), pair.as_str()));
        self
    }

    pub(crate) fn absoluteIRIExpected(&mut self, str: String) -> &mut Self {
        self.context = Some(format!("Absolute IRI expected. Found {:?}", str));
        self
    }
}

impl From<Error<Rule>> for ShExCError {
    fn from(e: Error<Rule>) -> Self {
        ShExCError::ParseError { msg: format!("Error<Rule>: {:?}",e)}
    }
}

impl From<ParserError> for ShExCError {
    fn from(e: ParserError) -> Self {
        ShExCError::ParseError { msg: format!("ParserError: {:?}",e)}
    }
}

impl From<IriError> for ShExCError {
    fn from(e: IriError) -> Self {
        ShExCError::IRIError { msg: format!("IriError: {:?}",e)}
    }
}

impl std::error::Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            format!(
                "{}{}{}{}",
                &self.fn_name,
                match &self.rule {
                    None => String::new(),
                    Some(s) => format!(", rule: {s}"),
                },
                match &self.expecting {
                    None => String::new(),
                    Some(s) => format!(", expecting: {s}"),
                },
                if self.unreachable {
                    ", should have been unreachable".to_string()
                } else {
                    String::new()
                },
            ),
            match &self.context {
                None => String::new(),
                Some(s) => format!(", context: '{s}'"),
            }
        )
    }
}
