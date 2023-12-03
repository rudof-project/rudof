use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use iri_s::IriS;
use nom::branch::alt;
use nom::bytes::complete::{tag, is_not};

use nom::IResult;
use nom::Err;
use nom::character::complete::multispace1;
use nom::error::ErrorKind;
use nom::combinator::{all_consuming, value, cut};
use nom::multi::{many0, many1};
use nom::sequence::{tuple, pair, delimited};
use nom_locate::LocatedSpan;
use prefixmap::PrefixMap;
use thiserror::Error;


// This parser approach borrows lots of ideas and code from [Nemo](https://github.com/knowsys/nemo)


/// A [`LocatedSpan`] over the input.
type Span<'a> = LocatedSpan<&'a str>;

/// Create a [`Span`][nom_locate::LocatedSpan] over the input.
pub fn span_from_str(input: &str) -> Span<'_> {
    Span::new(input)
}

/// An intermediate parsing result
pub(super) type IntermediateResult<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

/// The result of a parse
pub type ParseResult<'a, T> = Result<T, LocatedParseError>;

/// A [`ParseError`] at a certain location
#[derive(Debug, Error)]
#[error("Parse error on line {}, column {}: {}\nat {}{}", 
  .line, 
  .column, 
  .source, 
  .fragment, 
  format_parse_error_context(.context))]
pub struct LocatedParseError {
    #[source]
    pub(super) source: ParseError,
    pub(super) line: u32,
    pub(super) column: usize,
    pub(super) fragment: String,
    pub(super) context: Vec<LocatedParseError>,
}

impl LocatedParseError {
    /// Append another [`LocatedParseError`] as context to this error.
    pub fn append(&mut self, other: LocatedParseError) {
        self.context.push(other)
    }
}


fn format_parse_error_context(context: &[LocatedParseError]) -> String {
    let mut fragments = Vec::new();

    for error in context {
        let error_string = format!("{error}");
        for line in error_string.split('\n') {
            fragments.push(format!("{}{line}", " ".repeat(2)));
        }
    }

    if fragments.is_empty() {
        String::new()
    } else {
        format!("\nContext:\n{}", fragments.join("\n"))
    }
}

impl nom::error::ParseError<Span<'_>> for LocatedParseError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        ParseError::SyntaxError(kind.description().to_string()).at(input)
    }

    fn append(input: Span, kind: ErrorKind, other: Self) -> Self {
        let mut error = ParseError::SyntaxError(kind.description().to_string()).at(input);
        error.append(other);
        error
    }
}

/// Errors that can occur during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
   
    #[error("Expected further input: {0}")]
    MissingInput(String),

    #[error(r#"Expected "{0}""#)]
    ExpectedToken(String),

    #[error("Expected Association which starts by @")]
    ExpectedShapeAssociation,

    #[error("Expected 1 or more shape map declarations")]
    ExpectedShapeMapDecls,

    #[error("Expected IRI")]
    ExpectedIri,

    #[error("Expected shape label")]
    ExpectedShapeLabel,

}

impl ParseError {
    /// Locate this error by adding a position.
    pub fn at(self, position: Span) -> LocatedParseError {
        // miri doesn't like nom_locate, cf. https://github.com/fflorent/nom_locate/issues/88
        let column = if cfg!(not(miri)) {
            position.naive_get_utf8_column()
        } else {
            0
        };
        let fragment = if position.is_empty() {
            String::new()
        } else {
            let line = if cfg!(not(miri)) {
                String::from_utf8(position.get_line_beginning().to_vec())
                    .expect("input is valid UTF-8")
            } else {
                String::new()
            };
            format!("\"{line}\"\n{}^", "-".repeat(3 + column))
        };

        LocatedParseError {
            source: self,
            line: position.location_line(),
            column,
            fragment,
            context: Vec::new(),
        }
    }
}

pub fn parse_shapemap(input: impl AsRef<str>) -> Result<InputShapeMap, LocatedParseError> {
    let shapemap = all_input_consumed(ShapeMapParser::new().parse_shapemap())(input.as_ref())?;
    Ok(shapemap)
}

#[derive(Debug, PartialEq, Clone)]

pub struct Association {
    node: NodeSelector, 
    shape: ShapeMapLabel
}

impl Association {
    fn new(node: NodeSelector, shape: ShapeMapLabel) -> Self {
        Association { node, shape }
    }
    
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeSelector {
    Iri(IriS)
}

impl NodeSelector {
    fn iri(iri: IriS) -> NodeSelector {
        NodeSelector::Iri(iri)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShapeMapLabel {
    Start,
    Iri(IriS)
}

impl ShapeMapLabel {
    fn start() -> Self {
        ShapeMapLabel::Start
    }

    fn iri(iri: IriS) -> Self {
        ShapeMapLabel::Iri(iri)
    }
}


/// The main parser. Holds a hash map for
/// prefixes, as well as the base IRI.
#[derive(Debug, Default)]
pub struct ShapeMapParser {
    base: Option<IriS>,
    nodes_prefixmap: Option<PrefixMap>,
    shapes_prefixmap: Option<PrefixMap>,
    associations: Vec<Association>,
}


impl ShapeMapParser {
    /// Construct a new [`RuleParser`].
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_base(mut self, base: Option<IriS>) -> Self {
        self.base = base;
        self
    }

    pub fn with_nodes_prefixmap(mut self, nodes_prefixmap: Option<PrefixMap>) -> Self {
        self.nodes_prefixmap = nodes_prefixmap;
        self
    }

    pub fn with_shapes_prefixmap(mut self, shapes_prefixmap: Option<PrefixMap>) -> Self {
        self.shapes_prefixmap = shapes_prefixmap;
        self
    }

    pub fn add_association(&mut self, a: Association) {
        self.associations.push(a);
    }

    pub fn shape_map(&self) -> InputShapeMap {
        // InputShapeMap::new().with_associations(self.associations.)
        todo!()
    }

    pub fn parse<'a>(str: &str) -> Result<Vec<Association>, LocatedParseError> {
       todo!()
    }

    pub fn parse_shapemap<'a>(&self) -> impl FnMut(Span<'a>) -> IntermediateResult<'a, InputShapeMap> {
        traced("parse_shapemap", 
            move |i| {
            let (i, _) = tws0(i)?;
            let (i, _) = tuple(
                (shape_association(), 
                 many0(comma_shape_association())
                ))(i)?;


            Ok((i, InputShapeMap::new()))
        }
      )
    }

}


pub fn shape_association<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, Association> {
    traced("shape_association",
        map_error(
            move |i| {
             let (i, (node_selector, _, shapemap_label, _)) = tuple((iri, token_tws("@"), cut(shapemap_label()), tws0))(i)?;
             let association = Association::new(NodeSelector::iri(node_selector), shapemap_label);
             Ok((i, association))
            },
            || ParseError::ExpectedShapeAssociation
        )
    )
}

pub fn shapemap_label<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, ShapeMapLabel> {
    traced("shapemap_label",
        map_error(
            move |i| {
             let (i, label) = token_tws("label")(i)?;
             Ok((i, ShapeMapLabel::Start))
            },
            || ParseError::ExpectedShapeLabel
        )
    )
}


pub fn comma_shape_association<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, Vec<Association>> {
    traced("comma_shape_association",
        map_error(
            move |i| {
             let (i, _) = tuple((token_tws(","), cut(shape_association())))(i)?;
             Ok((i, Vec::new()))
            },
            || ParseError::ExpectedShapeAssociation
        )
    )
}

/// A combinator that recognises a comment, starting at a `#`
/// character and ending at the end of the line.
pub fn comment(input: Span) -> IntermediateResult<()> {
    alt((
        value((), pair(tag("#"), is_not("\n\r"))),
        // a comment that immediately precedes the end of the line –
        // this must come after the normal line comment above
        value((), tag("#")),
    ))(input)
}

/// A combinator that recognises an arbitrary amount of whitespace and
/// comments.
pub fn tws0(input: Span) -> IntermediateResult<()> {
    value((), many0(alt((value((), multispace1), comment))))(input)
}

/// A combinator that recognises any non-empty amount of whitespace
/// and comments.
pub fn tws1(input: Span) -> IntermediateResult<()> {
    value((), many1(alt((value((), multispace1), comment))))(input)
}

/// A combinator that creates a parser for a specific token.
pub fn token<'a>(token: &'a str) -> impl FnMut(Span<'a>) -> IntermediateResult<Span<'a>> {
    map_error(tag(token), || ParseError::ExpectedToken(token.to_string()))
}

/// A combinator that creates a parser for a specific token,
/// surrounded by trailing whitespace or comments.
pub fn token_tws<'a>(
    token: &'a str,
) -> impl FnMut(Span<'a>) -> IntermediateResult<Span<'a>> {
    map_error(
        delimited(tws0, tag(token), tws0),
        || ParseError::ExpectedToken(token.to_string()),
    )
}

pub fn iri(i: Span) -> IntermediateResult<IriS> {
    let (i, _) = map_error(
        token_tws(":a"),
        || ParseError::ExpectedIri,
    )(i)?;
    Ok((i, IriS::new_unchecked("http://example.org/a")))
}

/// A combinator that modifies the associated error.
pub fn map_error<'a, T: 'a>(
    mut parser: impl FnMut(Span<'a>) -> IntermediateResult<'a, T> + 'a,
    mut error: impl FnMut() -> ParseError + 'a,
) -> impl FnMut(Span<'a>) -> IntermediateResult<'a, T> + 'a {
    move |input| {
        parser(input).map_err(|e| match e {
            Err::Incomplete(_) => e,
            Err::Error(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Error(err)
            }
            Err::Failure(context) => {
                let mut err = error().at(input);
                err.append(context);
                Err::Failure(err)
            }
        })
    }
}
  

/// A combinator to add tracing to the parser.
/// [fun] is an identifier for the parser and [parser] is the actual parser.
#[inline(always)]
fn traced<'a, T, P>(
    fun: &'static str,
    mut parser: P,
) -> impl FnMut(Span<'a>) -> IntermediateResult<'a, T>
where
    T: Debug,
    P: FnMut(Span<'a>) -> IntermediateResult<'a, T>,
{
    move |input| {
        log::trace!(target: "parser", "{fun}({input:?})");
        let result = parser(input);
        log::trace!(target: "parser", "{fun}({input:?}) -> {result:?}");
        result
    }
}

/// A combinator that makes sure all input has been consumed.
pub fn all_input_consumed<'a, T: 'a>(
    parser: impl FnMut(Span<'a>) -> IntermediateResult<'a, T> + 'a,
) -> impl FnMut(&'a str) -> Result<T, LocatedParseError> + 'a {
    let mut p = all_consuming(parser);
    move |input| {
        let input = Span::new(input);
        p(input).map(|(_, result)| result).map_err(|e| match e {
            Err::Incomplete(e) => ParseError::MissingInput(match e {
                nom::Needed::Unknown => "expected an unknown amount of further input".to_string(),
                nom::Needed::Size(size) => format!("expected at least {size} more bytes"),
            })
            .at(input),
            Err::Error(e) | Err::Failure(e) => e,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputShapeMap {
    associations: Vec<Association>
}

impl InputShapeMap {
    pub fn new() -> Self {
        InputShapeMap { associations: Vec::new() }
    }

    pub fn with_associations(mut self, associations: Vec<Association>) -> Self {
        self.associations = associations;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test_log::test]
    fn example_shapemap () {
        let input = Span::new(":a@label");
        let shape_map = parse_shapemap(input).unwrap();
        let expected = InputShapeMap::new();
        assert_eq!(shape_map, expected);
    }

/*    #[test_log::test]
    fn example_shapemap_failed () {
        let input = Span::new("\n @START \n # Comment \n@STRT\n");
        let shape_map = parse_shapemap(input).unwrap();
        let expected = InputShapeMap::new();
        assert_eq!(shape_map, expected);
    } */

}
