use iri_s::IriS;
use colored::*;
use std::{fmt::Debug, num::{ParseIntError, ParseFloatError}, result, collections::VecDeque};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1, is_not},
    character::complete::{char, digit1, one_of, satisfy, multispace1, alpha1, alphanumeric1, none_of, digit0},
    combinator::{fail, map_res, opt, recognize, value, cut, map},
    error::{ErrorKind, FromExternalError},
    error_position,
    multi::{fold_many0, many0, many1, count},
    sequence::{delimited, tuple, pair, preceded},
    Err, IResult, InputTake, 
};
use shex_ast::{
    object_value::ObjectValue, value_set_value::ValueSetValue, Annotation, IriRef, NodeConstraint,
    Ref, SemAct, Shape, ShapeExpr, TripleExpr, XsFacet, NodeKind, NumericFacet, NumericLiteral, Pattern, StringFacet,
};
use rust_decimal::Decimal;
use log;
use thiserror::Error;

use crate::{Cardinality, Qualifier, ShExStatement, ParseError as ShExParseError, NumericLength, NumericRange};
use nom_locate::LocatedSpan;
use srdf::{lang::Lang, literal::Literal};

// Some definitions borrowed from [Nemo](https://github.com/knowsys/nemo/blob/main/nemo/src/io/parser/types.rs)

pub type IRes<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

/// A [`LocatedSpan`] over the input.
pub type Span<'a> = LocatedSpan<&'a str>;

/// Create a [`Span`][nom_locate::LocatedSpan] over the input.
pub fn span_from_str(input: &str) -> Span<'_> {
    Span::new(input)
}

/// The result of a parse
pub type ParseResult<'a, T> = Result<T, LocatedParseError>;


/// A [`ParseError`] at a certain location
#[derive(Debug, Error)]
#[error("Parse error on line {}, column {}: {}\nat {}{}", 
  .line, .column, 
  .source, 
  .fragment, 
  format_parse_error_context(.context))]
pub struct LocatedParseError {
    #[source]
    pub source: ShExParseError,
    pub line: u32,
    pub column: usize,
    pub fragment: String,
    pub context: Vec<LocatedParseError>,
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
        ShExParseError::SyntaxError(kind.description().to_string()).at(input)
    }

    fn append(input: Span, kind: ErrorKind, other: Self) -> Self {
        let mut error = ShExParseError::SyntaxError(kind.description().to_string()).at(input);
        error.append(other);
        error
    }
}

/// A combinator that modifies the associated error.
pub fn map_error<'a, T: 'a>(
    mut parser: impl FnMut(Span<'a>) -> IRes<'a, T> + 'a,
    mut error: impl FnMut() -> ShExParseError + 'a,
) -> impl FnMut(Span<'a>) -> IRes<'a, T> + 'a {
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
) -> impl FnMut(Span<'a>) -> IRes<'a, T>
where
    T: Debug,
    P: FnMut(Span<'a>) -> IRes<'a, T>,
{
    move |input| {
        log::trace!(target: "parser", "{fun}({input:?})");
        let result = parser(input);
        match &result {
            Ok(res) => { 
                let s = format!("{fun}({input:?}) -> {res:?}");
                log::trace!(target: "parser", "{}", s.green()); 
            }
            Err(e) => { 
                let s = format!("{fun}({input:?}) -> {e:?}");
                log::trace!(target: "parser", "{}", s.red()); 
            }
        }
        result
    }
}

/// A combinator that recognises a comment, starting at a `#`
/// character and ending at the end of the line.
pub fn comment(input: Span) -> IRes<()> {
    alt((
        value((), pair(tag("#"), is_not("\n\r"))),
        // a comment that immediately precedes the end of the line –
        // this must come after the normal line comment above
        value((), tag("#")),
        value((), multi_comment)
    ))(input)
}

pub fn multi_comment(i: Span) -> IRes<()> {
    value((), delimited(tag("/*"), is_not("*/"), tag("*/")))(i)
}

/// A combinator that recognises an arbitrary amount of whitespace and
/// comments.
pub fn tws0(input: Span) -> IRes<()> {
    value((), many0(alt((value((), multispace1), comment))))(input)
}

/// A combinator that recognises any non-empty amount of whitespace
/// and comments.
pub fn tws1(input: Span) -> IRes<()> {
    value((), many1(alt((value((), multispace1), comment))))(input)
}

/// A combinator that creates a parser for a specific token.
pub fn token<'a>(token: &'a str) -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    map_error(tag(token), || ShExParseError::ExpectedToken(token.to_string()))
}

/// A combinator that creates a parser for a specific token,
/// surrounded by trailing whitespace or comments.
pub fn token_tws<'a>(
    token: &'a str,
) -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    map_error(
        delimited(tws0, tag(token), tws0),
        || ShExParseError::ExpectedToken(token.to_string()),
    )
}

/// `[1] shexDoc	   ::=   	directive* ((notStartAction | startActions) statement*)?`
pub fn shex_statement<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Vec<ShExStatement>> {
    traced("shex_statement", 
    move |i| {
    let (i, (ds, _, maybe_sts)) = tuple((directives, tws0, opt(rest_shex_statements)))(i)?;
    let mut result = Vec::new();
     result.extend(ds);
     match maybe_sts {
       None => {}
       Some(sts) => {
         result.extend(sts);
       }
     }
     Ok((i, result))
    })
}

/// From [1] rest_shex_statements = ((notStartAction | startActions) statement*)
pub fn rest_shex_statements(i: Span) -> IRes<Vec<ShExStatement>> {
    let (i, (s, _, ss, _)) = tuple((alt((not_start_action, start_actions)), tws0, statements, tws0))(i)?;
    let mut rs = vec![s];
    rs.extend(ss);
    Ok((i, rs))
}

pub fn directives(i: Span) -> IRes<Vec<ShExStatement>> {
    let (i, vs) = many0(tuple((directive, tws1)))(i)?;
    let mut rs = Vec::new();
    for v in vs {
        let (d, _) = v;
        rs.push(d);
    }
    Ok((i, rs))
}

pub fn statements(i: Span) -> IRes<Vec<ShExStatement>> {
    many0(statement)(i)
}

/// `[2] directive	   ::=   	baseDecl | prefixDecl | importDecl`
pub fn directive(i: Span) -> IRes<ShExStatement> {
    alt((
        base_decl(),
        prefix_decl(),
        import_decl()
    ))(i)
}

/// `[3]   	baseDecl	   ::=   	"BASE" IRIREF`
fn base_decl<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShExStatement> {
    traced("base_decl",
    map_error(move |i| {
         let (i, (_, _, iri_ref)) =
         tuple(
            (tag_no_case("BASE"), 
              tws0, 
              cut(iri_ref)))(i)?;
     Ok((
         i,
         ShExStatement::BaseDecl {
             iri: iri_ref,
         },
     ))
    }, || ShExParseError::ExpectedBaseDecl
   ))
 }
 

/// [4] `prefixDecl	   ::=   	"PREFIX" PNAME_NS IRIREF`
fn prefix_decl<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShExStatement> {
   traced("prefix_decl",
   map_error(move |i| {
        let (i, (_, _, pname_ns, _, iri_ref)) =
        tuple((tag_no_case("PREFIX"), tws0, cut(pname_ns), tws0, cut(iri_ref)))(i)?;
    Ok((
        i,
        ShExStatement::PrefixDecl {
            alias: pname_ns.fragment(),
            iri: iri_ref,
        },
    ))
   }, || ShExParseError::ExpectedPrefixDecl
  ))
}

/// `[4½]   	importDecl	   ::=   	"IMPORT" IRIREF`
fn import_decl<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShExStatement> {
    traced("import_decl",
    map_error(move |i| {
         let (i, (_, _, iri_ref)) =
         tuple(
            (tag_no_case("IMPORT"), 
              tws0, 
              cut(iri_ref)))(i)?;
     log::debug!("grammar: Import {iri_ref:?}");         
     Ok((
         i,
         ShExStatement::ImportDecl {
             iri: iri_ref,
         },
     ))
    }, || ShExParseError::ExpectedImportDecl
   ))
 }

/// `[5]   	notStartAction	   ::=   	start | shapeExprDecl`
fn not_start_action(i: Span) -> IRes<ShExStatement> {
    alt((start(), shape_expr_decl()))(i)
}

/// `[6]   	start	   ::=   	"start" '=' inlineShapeExpression`
fn start<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShExStatement> {
    map_error(move |i| {
        let (i, (_, _, _, _, se)) = tuple((
        tag_no_case("START"),
        tws0,
        cut(char('=')),
        tws0,
        cut(inline_shape_expression),
    ))(i)?;
    Ok((i, ShExStatement::StartDecl { shape_expr: se }))
    },
    || ShExParseError::ExpectedStart
  )
}

/// `[7]   	startActions	   ::=   	codeDecl+`
fn start_actions(i: Span) -> IRes<ShExStatement> {
    let (i, cs) = many1(code_decl())(i)?;
    Ok((i, ShExStatement::StartActions { actions: cs }))
}

/// `[8]   	statement	   ::=   	directive | notStartAction`
fn statement(i: Span) -> IRes<ShExStatement> {
    alt((directive, not_start_action))(i)
}

/// `[9]   	shapeExprDecl	   ::=   	shapeExprLabel (shapeExpression | "EXTERNAL")`
fn shape_expr_decl<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShExStatement> {
    traced("shape_expr_decl", 
    map_error(
        move |i| {
      let (i, (shape_label, _, shape_expr)) =
        tuple((shape_expr_label, tws0, cut(shape_expr_or_external)))(i)?;
    Ok((
        i,
        ShExStatement::ShapeDecl {
            shape_label,
            shape_expr,
        },
    ))
   },
   || ShExParseError::ExpectedShapeExprDecl
  ))
}

fn shape_expr_or_external(i: Span) -> IRes<ShapeExpr> {
    alt((shape_expression, external))(i)
}

fn external(i: Span) -> IRes<ShapeExpr> {
    let (i, _) = tag_no_case("EXTERNAL")(i)?;
    Ok((i, ShapeExpr::external()))
}

/// `[10]   	shapeExpression	   ::=   	shapeOr`
fn shape_expression(i: Span) -> IRes<ShapeExpr> {
    shape_or(i)
}

/// `[11]   	inlineShapeExpression	   ::=   	inlineShapeOr`
fn inline_shape_expression(i: Span) -> IRes<ShapeExpr> {
    inline_shape_or(i)
}

/// `[12]   	shapeOr	   ::=   	shapeAnd ("OR" shapeAnd)*`
fn shape_or<'a>(i: Span<'a>) -> IRes<'a, ShapeExpr> {
    many1_sep(shape_and, symbol("OR"), make_shape_or, i)
}

fn make_shape_or(ses: Vec<ShapeExpr>) -> ShapeExpr {
    if ses.len() == 1 {
        ses[0].clone()
    } else {
        ShapeExpr::or(ses)
    }
}

/// `[13]   	inlineShapeOr	   ::=   	inlineShapeAnd ("OR" inlineShapeAnd)*`
fn inline_shape_or(i: Span) -> IRes<ShapeExpr> {
    many1_sep(inline_shape_and, symbol("OR"), make_shape_or, i)
}

/// `[14]   	shapeAnd	   ::=   	shapeNot ("AND" shapeNot)*``
fn shape_and(i: Span) -> IRes<ShapeExpr> {
    many1_sep(shape_not, symbol("AND"), make_shape_and, i)
}

fn make_shape_and(ses: Vec<ShapeExpr>) -> ShapeExpr {
    if ses.len() == 1 {
        ses[0].clone()
    } else {
        ShapeExpr::and(ses)
    }
}

/// `[15]   	inlineShapeAnd	   ::=   	inlineShapeNot ("AND" inlineShapeNot)*`
fn inline_shape_and(i: Span) -> IRes<ShapeExpr> {
    many1_sep(inline_shape_not, symbol("AND"), make_shape_and, i)
}

/// `[16]   	shapeNot	   ::=   	"NOT"? shapeAtom`
fn shape_not(i: Span) -> IRes<ShapeExpr> {
    let (i, maybe) = opt(symbol("NOT"))(i)?;
    let (i, se) = shape_atom(i)?;
    match maybe {
        None => Ok((i, se)),
        Some(_) => Ok((i, ShapeExpr::not(se))),
    }
}

/// `[17]   	inlineShapeNot	   ::=   	"NOT"? inlineShapeAtom`
fn inline_shape_not(i: Span) -> IRes<ShapeExpr> {
    let (i, maybe) = opt(symbol("NOT"))(i)?;
    let (i, se) = inline_shape_atom(i)?;
    match maybe {
        None => Ok((i, se)),
        Some(_) => Ok((i, ShapeExpr::not(se))),
    }
}

/// `[18]   	shapeAtom	   ::=   	   nonLitNodeConstraint shapeOrRef?
/// `| litNodeConstraint`
/// `| shapeOrRef nonLitNodeConstraint?`
/// `| '(' shapeExpression ')'`
/// `| '.'`
fn shape_atom(i: Span) -> IRes<ShapeExpr> {
    alt((
        non_lit_opt_shape_or_ref,
        lit_node_constraint_shape_expr,
        shape_opt_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

fn non_lit_opt_shape_or_ref(i: Span) -> IRes<ShapeExpr> {
    let (i, (non_lit, _, maybe_se)) = tuple((non_lit_node_constraint, tws0, opt(shape_or_ref())))(i)?;
    let nc = ShapeExpr::node_constraint(non_lit);
    let se_result = match maybe_se {
        None => nc,
        Some(se) => make_shape_and(vec![nc, se])
    };
    Ok((i, se_result)) 
}

/// `From [18] `shape_opt_non_lit ::= shapeOrRef nonLitNodeConstraint?`
fn shape_opt_non_lit(i: Span) -> IRes<ShapeExpr> {
    let (i, se) = shape_or_ref()(i)?;
    let (i, maybe_non_lit) = opt(non_lit_node_constraint)(i)?;
    match maybe_non_lit {
        None => Ok((i, se)),
        Some(nl) => Ok((i, ShapeExpr::and(vec![se, ShapeExpr::node_constraint(nl)]))),
    }
}

/// `[20] inlineShapeAtom ::= nonLitNodeConstraint inlineShapeOrRef?`
/// `                      | litNodeConstraint`
/// `                      | inlineShapeOrRef nonLitNodeConstraint?`
/// `                      | '(' shapeExpression ')'`
/// `                      | '.'`
fn inline_shape_atom(i: Span) -> IRes<ShapeExpr> {
    alt((
        non_lit_inline_opt_shape_or_ref,
        lit_node_constraint_shape_expr,
        inline_shape_or_ref_opt_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

/// From [20] `non_lit_inline_opt_shape_or_ref = nonLitNodeConstraint inlineShapeOrRef?`
fn non_lit_inline_opt_shape_or_ref(i: Span) -> IRes<ShapeExpr> {
    let (i, (non_lit, _, maybe_se)) = tuple((non_lit_node_constraint, tws0, opt(inline_shape_or_ref)))(i)?;
    let nc = ShapeExpr::node_constraint(non_lit);
    let se_result = match maybe_se {
        None => nc,
        Some(se) => make_shape_and(vec![nc, se])
    };
    Ok((i, se_result)) 

}

/// `from [20] `inline_shape_or_ref_opt_non_lit ::= inlineShapeOrRef nonLitNodeConstraint?`
fn inline_shape_or_ref_opt_non_lit(i: Span) -> IRes<ShapeExpr> {
    let (i, se) = inline_shape_or_ref(i)?;
    let (i, maybe_non_lit) = opt(non_lit_node_constraint)(i)?;
    match maybe_non_lit {
        None => Ok((i, se)),
        Some(nl) => Ok((i, ShapeExpr::and(vec![se, ShapeExpr::node_constraint(nl)]))),
    }
}

fn lit_node_constraint_shape_expr(i: Span) -> IRes<ShapeExpr> {
    let (i, nc) = lit_node_constraint(i)?;
    Ok((i, ShapeExpr::NodeConstraint(nc)))
}

fn paren_shape_expr(i: Span) -> IRes<ShapeExpr> {
    let (i, (_, _, se, _, _)) = tuple((char('('), tws0, shape_expression, tws0, char(')')))(i)?;
    Ok((i, se))
}

fn dot(i: Span) -> IRes<ShapeExpr> {
    let (i, (_, _)) = tuple((tws0, char('.')))(i)?;
    Ok((i, ShapeExpr::any()))
}

/// `[21]   	shapeOrRef	   ::=   	   shapeDefinition | shapeRef`
fn shape_or_ref<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShapeExpr> {
  traced("shape_or_ref", 
  map_error(
    move |i| { alt((shape_definition(), shape_ref))(i) },
    || ShExParseError::ExpectedShapeOrRef
  ))  
}

/// `[22]   	inlineShapeOrRef	   ::=   	   inlineShapeDefinition | shapeRef`
fn inline_shape_or_ref(i: Span) -> IRes<ShapeExpr> {
    alt((inline_shape_definition, shape_ref))(i)
}

/// `[23]   	shapeRef	   ::=   	   ATPNAME_LN | ATPNAME_NS | '@' shapeExprLabel`
fn shape_ref(i: Span) -> IRes<ShapeExpr> {
    alt((at_pname_ln, at_pname_ns, at_shape_expr_label))(i)
}

fn at_shape_expr_label(i: Span) -> IRes<ShapeExpr> {
    let (i, (_, label)) = tuple((char('@'), shape_expr_label))(i)?;
    Ok((i, ShapeExpr::shape_ref(label)))
}

/// `[24]   	litNodeConstraint	   ::=   	   "LITERAL" xsFacet*
/// | datatype xsFacet*
/// | valueSet xsFacet*
/// | numericFacet+`
fn lit_node_constraint(i: Span) -> IRes<NodeConstraint> {
    alt((
        literal_facets(),
        datatype_facets,
        value_set_facets,
        numeric_facets,
    ))(i)
}

fn literal_facets<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, NodeConstraint> {
    traced("literal_facets", move |i| {
     let (i, (_, _, facets)) = tuple((tag_no_case("LITERAL"), tws1, facets()))(i)?;
     Ok((i, NodeConstraint::new()
            .with_node_kind(NodeKind::Literal)
            .with_xsfacets(facets))
    )
    })
}

fn datatype_facets(i: Span) -> IRes<NodeConstraint> {
    let (i, (dt, _, facets)) = tuple((datatype, tws1, facets()))(i)?;
    Ok((i, dt.with_xsfacets(facets)))
}

fn value_set_facets(i: Span) -> IRes<NodeConstraint> {
    let (i, (vs, _, facets)) = tuple((value_set, tws1, facets()))(i)?;
    Ok((i, vs.with_xsfacets(facets)))
}

/// `from [24] numeric_facets = numericFacet+`
fn numeric_facets(i: Span) -> IRes<NodeConstraint> {
    map(many1(numeric_facet()), |ns| NodeConstraint::new().with_xsfacets(ns))(i)
}

fn facets<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Vec<XsFacet>> {
    traced("facets", move |i| {
     many0(xs_facet())(i)
    })
}

/// `[25] nonLitNodeConstraint ::= nonLiteralKind stringFacet*`
/// `                            | stringFacet+`
fn non_lit_node_constraint(i: Span) -> IRes<NodeConstraint> {
    alt((non_literal_kind_string_facets, string_facets))(i)
}

/// `from [25] non_literal_kind_string-facets = nonLiteralKind stringFacet* `
fn non_literal_kind_string_facets(i: Span) -> IRes<NodeConstraint> {
    let (i, (kind, facets)) = tuple((non_literal_kind, many0(string_facet)))(i)?;
    let mut nc = NodeConstraint::new().with_node_kind(kind);
    if !facets.is_empty() {
       nc = nc.with_xsfacets(facets);
    } 
    Ok((i, nc))
}

/// `from [25] string_facets = string_facet+`
fn string_facets(i: Span) -> IRes<NodeConstraint> {
   let (i, facets) = many1(string_facet)(i)?;
   Ok((i, NodeConstraint::new().with_xsfacets(facets)))
}

/// `[26]   	nonLiteralKind	   ::=   	"IRI" | "BNODE" | "NONLITERAL"` 
fn  non_literal_kind(i: Span) -> IRes<NodeKind> {
    alt((
        map(token_tws("IRI"), |_| NodeKind::Iri),
        map(token_tws("BNODE"), |_| NodeKind::BNode),
        map(token_tws("NONLITERAL"), |_| NodeKind::NonLiteral)
    ))(i)
}

/// `[27]   	xsFacet	   ::=   	stringFacet | numericFacet`
fn xs_facet<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, XsFacet> {
  traced("xs_facet", move |i| {
     alt((
        string_facet,
        numeric_facet()
    ))(i)
  })
}

/// `[28]   	stringFacet	   ::=   	   stringLength INTEGER`
/// `| REGEXP`
fn string_facet(i: Span) -> IRes<XsFacet> {
    alt((
        string_length,
        map(regexp, |p| XsFacet::StringFacet(StringFacet::Pattern(p)))
    ))(i)
}

// `[29]   	stringLength	   ::=   	"LENGTH" | "MINLENGTH" | "MAXLENGTH"`
fn string_length(i: Span) -> IRes<XsFacet> {
    alt((min_length, max_length, length))(i)
}

fn min_length(i: Span) -> IRes<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("MINLENGTH"), tws1, pos_integer))(i)?;
    Ok((i, XsFacet::min_length(n)))
}

fn max_length(i: Span) -> IRes<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("MAXLENGTH"), tws1, pos_integer))(i)?;
    Ok((i, XsFacet::max_length(n)))
}

fn length(i: Span) -> IRes<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("LENGTH"), tws0, pos_integer))(i)?;
    Ok((i, XsFacet::length(n)))
}

fn pos_integer(i: Span) -> IRes<usize> {
    let (i, n) = integer(i)?;
    let u: usize;
    if n < 0 {
        Err(Err::Error(error_position!(i, ErrorKind::Digit)))
    } else {
        u = n as usize;
        Ok((i, u))
    }
}

/// `[30]   	numericFacet	   ::=   	   numericRange numericLiteral
/// `| numericLength INTEGER`
fn numeric_facet<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, XsFacet> {
    traced("numeric_facet", move |i| {
        alt((
            numeric_range_lit(), 
            numeric_length_int()
        ))(i)
    })
}

/// `From [30] numeric_range_lit = numericRange numericLiteral``
fn numeric_range_lit<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, XsFacet> {
  traced("numeric_range", move |i| {  
   let (i, (n_range, v)) = tuple((numeric_range, cut(numeric_literal)))(i)?;
   let v = match n_range {
     NumericRange::MinInclusive => XsFacet::NumericFacet(NumericFacet::MinInclusive(v)),
     NumericRange::MinExclusive => XsFacet::NumericFacet(NumericFacet::MinExclusive(v)),
     NumericRange::MaxInclusive => XsFacet::NumericFacet(NumericFacet::MaxInclusive(v)),
     NumericRange::MaxExclusive => XsFacet::NumericFacet(NumericFacet::MaxExclusive(v)),
   };
   Ok((i, v))
  })
}

/// `From [30] numericLength INTEGER`
fn numeric_length_int<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, XsFacet> {
    traced("numeric_length_int", move |i| {
    let (i, (numeric_length,n)) = tuple((numeric_length, integer))(i)?;
    let nm = match numeric_length {
        NumericLength::FractionDigits => XsFacet::NumericFacet(NumericFacet::FractionDigits(n as usize)),
        NumericLength::TotalDigits => XsFacet::NumericFacet(NumericFacet::TotalDigits(n as usize))
    };
    Ok((i, nm))
    })
}

fn numeric_length(i: Span) -> IRes<NumericLength> {
    alt((
        map(token_tws("TOTALDIGITS"), |_| NumericLength::TotalDigits),
        map(token_tws("FRACTIONDIGITS"), |_| NumericLength::FractionDigits)
    ))(i)
}


/// `[31]    	numericRange 	   ::=    	"MININCLUSIVE" | "MINEXCLUSIVE" | "MAXINCLUSIVE" | "MAXEXCLUSIVE"`
fn numeric_range(i: Span) -> IRes<NumericRange> {
        alt((
          map(token_tws("MININCLUSIVE"), |_| NumericRange::MinInclusive),
          map(token_tws("MAXINCLUSIVE"), |_| NumericRange::MaxInclusive),
          map(token_tws("MINEXCLUSIVE"), |_| NumericRange::MinExclusive),
          map(token_tws("MAXEXCLUSIVE"), |_| NumericRange::MaxExclusive),
    ))(i) 
}

/// `[33]   	shapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}' annotation* semanticActions`
fn shape_definition<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, ShapeExpr> {
    traced("shape_definition",
    map_error(move |i| {
    let (i, (qualifiers, _, _, _, maybe_triple_expr, _, _, _, annotations, _, sem_actions)) =
        tuple((
            qualifiers(),
            tws0,
            char('{'),
            tws0,
            opt(triple_expression),
            tws0,
            char('}'),
            tws0,
            annotations,
            tws0,
            semantic_actions,
        ))(i)?;
    let closed = if qualifiers.contains(&Qualifier::Closed) {
        Some(true)
    } else {
        None
    };
    let mut extra = Vec::new();
    for q in qualifiers {
        match q {
            Qualifier::Closed => {}
            Qualifier::Extra(ps) => {
                for p in ps {
                    extra.push(p)
                }
            }
        }
    }
    let maybe_extra = if extra.is_empty() { None } else { Some(extra) };
    let annotations = if annotations.is_empty() {
        None
    } else {
        Some(annotations)
    };
    Ok((
        i,
        ShapeExpr::shape(
            Shape::new(closed, maybe_extra, maybe_triple_expr)
                .with_annotations(annotations)
                .with_sem_acts(sem_actions),
        ),
    ))
  }, || ShExParseError::ExpectedShapeDefinition))
}

/// `[34]   	inlineShapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}'`
fn inline_shape_definition(i: Span) -> IRes<ShapeExpr> {
    let (i, (qualifiers, _, _, _, maybe_triple_expr, _, _)) = tuple((
        qualifiers(),
        tws0,
        char('{'),
        tws0,
        opt(triple_expression),
        tws0,
        char('}'),
    ))(i)?;
    let closed = if qualifiers.contains(&Qualifier::Closed) {
        Some(true)
    } else {
        None
    };
    let mut extra = Vec::new();
    for q in qualifiers {
        match q {
            Qualifier::Closed => {}
            Qualifier::Extra(ps) => {
                for p in ps {
                    extra.push(p)
                }
            }
        }
    }
    let maybe_extra = if extra.is_empty() { None } else { Some(extra) };
    Ok((
        i,
        ShapeExpr::shape(Shape::new(closed, maybe_extra, maybe_triple_expr)),
    ))
}

fn annotations(i: Span) -> IRes<Vec<Annotation>> {
    many0(annotation())(i)
}

fn qualifiers<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Vec<Qualifier>> {
    traced("qualifiers", map_error(move |i| many0(qualifier())(i), || ShExParseError::ExpectedQualifiers))
}

fn qualifier<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Qualifier> {
    traced("qualifier", map_error(move |i| {
        alt((closed(), extra_property_set()))(i)
    }, || ShExParseError::ExpectedQualifier)
    )
}

fn closed<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Qualifier> {
    traced("Closed", map_error(move |i| {
    let (i, _) = token_tws("CLOSED")(i)?;
    Ok((i, Qualifier::Closed))
    }, || ShExParseError::ExpectedClosed))
}

/// `[35]   	extraPropertySet	   ::=   	"EXTRA" predicate+`
fn extra_property_set<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Qualifier> {
    traced("extra_property_set", 
    map_error(move |i| {
    let (i, (_, ps)) = tuple((token_tws("EXTRA"), cut(many1(predicate))))(i)?;
    Ok((i, Qualifier::Extra(ps)))
    }, || ShExParseError::ExpectedEXTRAPropertySet))
}

/// `[36]   	tripleExpression	   ::=   	oneOfTripleExpr`
fn triple_expression(i: Span) -> IRes<TripleExpr> {
    one_of_triple_expr(i)
}

/// `[37]   	oneOfTripleExpr	   ::=   	groupTripleExpr | multiElementOneOf`
fn one_of_triple_expr(i: Span) -> IRes<TripleExpr> {
    alt((multi_element_one_of(), group_triple_expr()))(i)
}

/// `[38]   	multiElementOneOf	   ::=   	groupTripleExpr ('|' groupTripleExpr)+`
fn multi_element_one_of<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, TripleExpr> {
  traced("multi_element_one_of", move |i| {
    let (i, (te1, _, tes)) = tuple((group_triple_expr(), tws0, rest_group_triple_expr))(i)?;
    let mut rs = vec![te1];
    for te in tes {
        rs.push(te);
    }
    let te = TripleExpr::one_of(rs);
    Ok((i, te))
  })
}

/// From [38] rest_group_triple_expr = ('|' groupTripleExpr)+
fn rest_group_triple_expr(i: Span) -> IRes<Vec<TripleExpr>> {
    let (i, vs) = many1(tuple((token_tws("|"), group_triple_expr())))(i)?;
    let mut tes = Vec::new();
    for v in vs {
        let (_, te) = v;
        tes.push(te);
    }
    Ok((i, tes))
}

/// `[40]   	groupTripleExpr	   ::=   	singleElementGroup | multiElementGroup`
fn group_triple_expr<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, TripleExpr> {
    traced("group_triple_expr", move |i| { alt((multi_element_group, single_element_group))(i) })
}

/// `[41]   	singleElementGroup	   ::=   	unaryTripleExpr ';'?`
fn single_element_group(i: Span) -> IRes<TripleExpr> {
    let (i, (te, _, _)) = tuple((unary_triple_expr, tws0, opt(char(';'))))(i)?;
    Ok((i, te))
}

/// `[42]   	multiElementGroup	   ::=   	unaryTripleExpr (';' unaryTripleExpr)+ ';'?`
fn multi_element_group(i: Span) -> IRes<TripleExpr> {
    let (i, (te1, _, tes, _, _)) = tuple((
        unary_triple_expr,
        tws0,
        rest_unary_triple_expr,
        tws0,
        opt(char(';')),
    ))(i)?;
    let mut rs = vec![te1];
    for t in tes {
        rs.push(t);
    }
    let te = TripleExpr::each_of(rs);
    Ok((i, te))
}

/// From [42] rest_unary_triple_expr = (';' unaryTripleExpr)+
fn rest_unary_triple_expr(i: Span) -> IRes<Vec<TripleExpr>> {
    let (i, vs) = many1(tuple((char(';'), tws0, unary_triple_expr)))(i)?;
    let mut tes = Vec::new();
    for v in vs {
        let (_, _, te) = v;
        tes.push(te)
    }
    Ok((i, tes))
}

/// `[43] unaryTripleExpr ::= ('$' tripleExprLabel)? (tripleConstraint | bracketedTripleExpr)`
/// `                     |   include`
fn unary_triple_expr(i: Span) -> IRes<TripleExpr> {
    alt((unary_triple_expr_opt1, include_))(i)
}

/// From [41] unary_triple_expr_opt1 = ('$' tripleExprLabel)? (tripleConstraint | bracketedTripleExpr)
fn unary_triple_expr_opt1(i: Span) -> IRes<TripleExpr> {
    let (i, (maybe_label, _, te)) = tuple((
        triple_expr_label_opt,
        tws0,
        alt((triple_constraint(), bracketed_triple_expr)),
    ))(i)?;
    // Pending: Process maybe_label
    Ok((i, te))
}

// From unary_triple_expr_opt1
fn triple_expr_label_opt(i: Span) -> IRes<Option<Ref>> {
    let (i, maybe_ts) = opt(tuple((char('$'), tws0, triple_expr_label)))(i)?;
    let maybe_label = match maybe_ts {
        Some((_, _, r)) => Some(r),
        None => None,
    };
    Ok((i, maybe_label))
}

/// `[44]   	bracketedTripleExpr	   ::=   	'(' tripleExpression ')' cardinality? annotation* semanticActions`
fn bracketed_triple_expr(i: Span) -> IRes<TripleExpr> {
    let (i, (_, _, te, _, _, _, maybe_card, _, annotations, _, sem_acts)) = tuple((
        char('('),
        tws0,
        cut(triple_expression),
        tws0,
        cut(char(')')),
        tws0,
        cut(opt(cardinality())),
        tws0,
        annotations,
        tws0,
        semantic_actions,
    ))(i)?;
    let mut te = te;
    match maybe_card {
        Some(card) => {
           te = te.with_min(card.min());
           te = te.with_max(card.max());
        },
        None => {}
    };
    if !annotations.is_empty() {
      te = te.with_annotations(Some(annotations));
    }
    te = te.with_sem_acts(sem_acts);
    Ok((i, te))
}

/// `[45]   	tripleConstraint	   ::=   	senseFlags? predicate inlineShapeExpression cardinality? annotation* semanticActions`
fn triple_constraint<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, TripleExpr> {
    traced("triple_constraint",
    map_error(
        move |i| { 
        let (i, (predicate, _, se, _, maybe_card, _, annotations, _, sem_acts)) = tuple((
        predicate,
        tws0,
        cut(inline_shape_expression),
        tws0,
        cut(opt(cardinality())),
        tws0,
        annotations,
        tws0,
        semantic_actions
    ))(i)?;
    let (min, max) = match maybe_card {
        None => (None, None),
        Some(card) => (card.min(), card.max()),
    };
    let value_expr = if se == ShapeExpr::any() {
        None
    } else {
        Some(se)
    };
    let mut te = TripleExpr::triple_constraint(predicate, value_expr, min, max);
    te = te.with_sem_acts(sem_acts);
    if !annotations.is_empty() {
        te = te.with_annotations(Some(annotations))
    }
    Ok((i,te))}, 
    || ShExParseError::ExpectedTripleConstraint
  ))
}

/// `[46]   	cardinality	   ::=   	'*' | '+' | '?' | REPEAT_RANGE`
fn cardinality<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Cardinality> {
   traced("cardinality", map_error(move |i| {
     alt((
        plus, 
        star, 
        optional,
        repeat_range()
     ))(i)
    }, || ShExParseError::ExpectedCardinality 
  ))
}

fn plus(i: Span) -> IRes<Cardinality> {
    let (i, _) = char('+')(i)?;
    Ok((i, Cardinality::plus()))
}

fn star(i: Span) -> IRes<Cardinality> {
    let (i, _) = char('*')(i)?;
    Ok((i, Cardinality::star()))
}

fn optional(i: Span) -> IRes<Cardinality> {
    let (i, _) = char('?')(i)?;
    Ok((i, Cardinality::optional()))
}

/// `[48]   	valueSet	   ::=   	'[' valueSetValue* ']'`
fn value_set(i: Span) -> IRes<NodeConstraint> {
    let (i, (_, _, vs, _, _)) = tuple((char('['), tws0, many0(value_set_value), tws0, char(']')))(i)?;
    Ok((i, NodeConstraint::new().with_values(vs)))
}

/// `[49]   	valueSetValue	   ::=   	iriRange | literalRange | languageRange`
/// `                               | exclusion+`
fn value_set_value(i: Span) -> IRes<ValueSetValue> {
    alt((
        // Pending
        iri_range,
        literal_range,
        // language_range,
        // exclusion_plus
    ))(i)
}

type Exclusion = ();

/// `[51]   	iriRange	   ::=   	   iri ('~' exclusion*)?`
fn iri_range(i: Span) -> IRes<ValueSetValue> {
    let (i, (iri, _, maybe_exc)) = tuple((iri, tws0, opt(tilde_exclusion)))(i)?;
    // Pending char_exclusion
    let vs = ValueSetValue::iri(iri);
    Ok((i, vs))
}

fn tilde_exclusion(i: Span) -> IRes<Vec<Exclusion>> {
    let (i, (_, _, es)) = tuple((char('~'), tws0, many0(exclusion)))(i)?;
    Ok((i, es))
}

/// `[50]   	exclusion	   ::=   	'.' '-' (iri | literal | LANGTAG) '~'?`
fn exclusion(i: Span) -> IRes<Exclusion> {
    let (i, (_, _, _, _, e, _, maybe_tilde)) =
        tuple((char('.'), tws0, char('-'), tws0, exc, tws0, opt(char('~'))))(i)?;
    Ok((i, ()))
}

/// `from [50] exc = iri | literal | LANGTAG`
fn exc(i: Span) -> IRes<Exclusion> {
    let (i, e) = alt((
        iri,
        // literal,
        // lang_tag
    ))(i)?;
    Ok((i, ()))
}

/// `[53]  literalRange	::= literal ('~' literalExclusion*)?`
fn literal_range(i: Span) -> IRes<ValueSetValue> {
   let (i, (literal, _, maybe_exc)) = tuple((literal, tws0, opt(tilde_literal_exclusion)))(i)?;

   // Pending literal_exclusion
   let vs = ValueSetValue::object_value(literal);
   Ok((i, vs))
}

fn tilde_literal_exclusion(i: Span) -> IRes<Vec<Exclusion>> {
    let (i, (_, _, es)) = tuple((tilde(), tws0, many0(literal_exclusion)))(i)?;
    Ok((i, es))
}

fn tilde<'a>() -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    move |i| { token("~")(i) }
}

fn dash<'a>() -> impl FnMut(Span<'a>) -> IRes<Span<'a>> {
    move |i| { token("-")(i) }
}

/// `[54]   	literalExclusion	   ::=   	'-' literal '~'?`
fn literal_exclusion(i: Span) -> IRes<Exclusion> {
    let (i, (_, literal, maybe_tilde)) = tuple((dash(), literal, opt(tilde())))(i)?;
    todo!()
    // Ok((i, Exclusion::))
}

/// `[57]   	include	   ::=   	'&' tripleExprLabel`
fn include_(i: Span) -> IRes<TripleExpr> {
    let (i, (_, _, tel)) = tuple((char('&'), tws0, triple_expr_label))(i)?;
    // Pending: We should add a temporary reference to a triple_expr_label which should be dereferenced in a second step
    todo!()
    // Ok((i, tel))
}

/// `[58]   	annotation	   ::=   	"//" predicate (iri | literal)`
fn annotation<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Annotation> {
    traced("annotation", map_error(move |i| {
    let (i, (_, p, _, o)) = tuple((token_tws("//"), cut(predicate), tws0, cut(iri_or_literal)))(i)?;
    Ok((i, Annotation::new(p.into(), o)))
    }, || ShExParseError::ExpectedAnnotation))
}

fn iri_or_literal(i: Span) -> IRes<ObjectValue> {
    // Pending literal
    let (i, iri) = iri(i)?;
    Ok((i, ObjectValue::IriRef(iri.into())))
}

/// `[59]   	semanticActions	   ::=   	codeDecl*`
fn semantic_actions(i: Span) -> IRes<Option<Vec<SemAct>>> {
    let (i, sas) = many0(code_decl())(i)?;
    if sas.is_empty() {
        Ok((i, None))
    } else {
        Ok((i, Some(sas)))
    }
}

/// `[60]   	codeDecl	   ::=   	'%' iri (CODE | '%')`
fn code_decl<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, SemAct> {
    traced("code_decl", map_error(move |i| {
    let (i, (_, _, iri, _, code)) = tuple((char('%'), tws0, cut(iri), tws0, cut(code_or_percent)))(i)?;
    Ok((i, SemAct::new(IriRef::from(iri), code)))
    }, || ShExParseError::CodeDeclaration))
}

fn code_or_percent(i: Span) -> IRes<Option<String>> {
    let (i, maybe_code) = alt((code(), percent_code))(i)?;
    Ok((i, maybe_code))
}

fn percent_code(i: Span) -> IRes<Option<String>> {
    let (i, _) = char('%')(i)?;
    Ok((i, None))
}

/// `[13t]   	literal	   ::=   	rdfLiteral | numericLiteral | booleanLiteral`
fn literal(i: Span) -> IRes<ObjectValue> {
   alt((
    rdf_literal, 
    map(numeric_literal, |n| ObjectValue::NumericLiteral(n)), 
    boolean_literal))(i)
}

/// `[16t]   	numericLiteral	   ::=   	INTEGER | DECIMAL | DOUBLE`
pub fn numeric_literal(i: Span) -> IRes<NumericLiteral> {
    alt((
        map(double, |n| NumericLiteral::decimal_from_f64(n)), 
        decimal,
        map(integer, |n| NumericLiteral::Integer(n))
    ))(i)
}

pub fn boolean_literal(i: Span) -> IRes<ObjectValue> {
    map(boolean_value, |b| ObjectValue::bool(b))(i)
}

pub fn boolean_value(i: Span) -> IRes<bool> {
    alt((map(token_tws("true"), |_| true), 
         map(token_tws("false"), |_| false)))(i)
}

/// `[65]   	rdfLiteral	   ::=   	langString | string ("^^" datatype)?`
/// Refactored according to rdfLiteral in Turtle
/// `rdfLiteral ::= string (LANGTAG | '^^' iri)?`
fn rdf_literal(i: Span) -> IRes<ObjectValue> {
    let (i, str) = string(i)?;
    let (i, maybe_value) = opt(alt((
        map(lang_string, |lang| ObjectValue::ObjectLiteral {
            value: str.fragment().to_string(),
            language: Some(lang),
            type_: None
        }),
        map(preceded(token("^^"), datatype_iri), |datatype| {
            ObjectValue::ObjectLiteral {
                value: str.fragment().to_string(),
                language: None,
                type_: Some(datatype)
            }
        }),
    )))(i)?;
    let value = match maybe_value {
        Some(v) => v,
        None => ObjectValue::ObjectLiteral {
            value: str.fragment().to_string(),
            language: None,
            type_: None
        }
    };
    Ok((i, value))
}

/// ``
fn datatype_iri(i: Span) -> IRes<IriRef> {
    iri(i)
}

/// `[135s]   	string	   ::= STRING_LITERAL1 | STRING_LITERAL_LONG1`
/// `                        | STRING_LITERAL2 | STRING_LITERAL_LONG2`
pub fn string(input: Span) -> IRes<Span> {
    map_error(
        alt((
            string_literal_long_quote,
            string_literal_long_single_quote,
            string_literal_quote,
            string_literal_single_quote,
        )),
        || ShExParseError::ExpectedStringLiteral,
    )(input)
}

pub fn string_literal_quote(input: Span) -> IRes<Span> {
    delimited(
        token(r#"""#),
        cut(recognize(many0(alt((
            recognize(none_of(REQUIRES_ESCAPE)),
            echar,
            uchar,
        ))))),
        token(r#"""#),
    )(input)
}

pub fn string_literal_single_quote(input: Span) -> IRes<Span> {
    delimited(
        token("'"),
        cut(recognize(many0(alt((
            recognize(none_of(REQUIRES_ESCAPE)),
            echar,
            uchar,
        ))))),
        token("'"),
    )(input)
}

pub fn string_literal_long_single_quote(input: Span) -> IRes<Span> {
    delimited(
        token("'''"),
        cut(recognize(many0(alt((
            recognize(none_of(r"'\")),
            echar,
            uchar,
        ))))),
        token("'''"),
    )(input)
}

pub fn string_literal_long_quote(input: Span) -> IRes<Span> {
    delimited(
        token(r#"""""#),
        cut(recognize(many0(alt((
            recognize(none_of(r#""\"#)),
            echar,
            uchar,
        ))))),
        token(r#"""""#),
    )(input)
}

pub fn hex(input: Span) -> IRes<Span> {
    recognize(one_of(HEXDIGIT))(input)
}

/// Valid hexadecimal digits.
const HEXDIGIT: &str = "0123456789ABCDEFabcdef";

/// Characters requiring escape sequences in single-line string literals.
/// 22 = ", 5C = \, 0A = \n, 0D = \r
const REQUIRES_ESCAPE: &str = "\u{22}\u{5C}\u{0A}\u{0D}";


pub fn uchar(input: Span) -> IRes<Span> {
    recognize(alt((
        preceded(token(r"\u"), count(hex, 4)),
        preceded(token(r"\U"), count(hex, 8)),
    )))(input)
}

fn echar(input: Span) -> IRes<Span> {
    recognize(preceded(token(r"\"), one_of(r#"tbnrf"'\"#)))(input)
}

/// 
fn lang_string(i: Span) -> IRes<Lang> {
    let (i, lang_str) = preceded(
        token("@"),
        recognize(tuple((alpha1, many0(preceded(token("-"), alphanumeric1))))),
    )(i)?;
    Ok((i, Lang::new(lang_str.fragment())))
}

/// `[61]   	predicate	   ::=   	iri | RDF_TYPE`
fn predicate(i: Span) -> IRes<IriRef> {
    alt((iri, rdf_type))(i)
}

/// `[62]   	datatype	   ::=   	iri`
fn datatype(i: Span) -> IRes<NodeConstraint> {
    let (i, iri_ref) = iri(i)?;
    Ok((i, NodeConstraint::new().with_datatype(iri_ref)))
}

/// `[63]   	shapeExprLabel	   ::=   	iri | blankNode`
fn shape_expr_label(i: Span) -> IRes<Ref> {
    let (i, ref_) = alt((iri_as_ref, blank_node_ref))(i)?;
    Ok((i, ref_))
}
fn iri_as_ref(i: Span) -> IRes<Ref> {
    let (i, iri_ref) = iri(i)?;
    Ok((i, Ref::iri_ref(iri_ref)))
}

fn blank_node_ref(i: Span) -> IRes<Ref> {
    let (i, bn) = blank_node(i)?;
    Ok((i, Ref::bnode_unchecked(bn)))
}

/// `[64]   	tripleExprLabel	   ::=   	iri | blankNode`
fn triple_expr_label(i: Span) -> IRes<Ref> {
    let (i, iri_ref) = iri(i)?; // alt((iri, blank_node))(i)?;
    let iri_s: IriS = iri_ref.into();
    Ok((i, Ref::from(iri_s)))
}

/// `[67]   	<CODE>	   ::=   	"{" ([^%\\] | "\\" [%\\] | UCHAR)* "%" "}"`
fn code<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Option<String>> {
    traced("code", map_error(move |i| {
    let (i, (_, str, _, _, _)) = tuple((char('{'), cut(code_str), cut(char('%')), tws0, cut(char('}'))))(i)?;
    let str = unescape_code(str.fragment());
    Ok((i, Some(str)))
    }, || ShExParseError::Code))
}

fn unescape_code(str: &str) -> String {
    let non_escaped = ['%', '\\'];
    let mut queue : VecDeque<_> = str.chars().collect();
    let mut r = String::new();
    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            r.push(c);
            continue;
        }
        match queue.pop_front() {
            Some(c) if non_escaped.contains(&c) => {
                r.push('\\');
                r.push(c)
            }
            Some('u') => {
                let mut s = String::new();
                for _ in 0..4 {
                    if let Some(c) = queue.pop_front() {
                       s.push(c)
                    } else {
                        panic!("unescape_pattern: \\u is not followed by 4 chars")
                    }
                } 

                let u = u32::from_str_radix(&s, 16).unwrap();
                let c = char::from_u32(u).unwrap(); 
                r.push(c)
            }
            Some('U') => {
                let mut s = String::new();
                for _ in 0..8 {
                    if let Some(c) = queue.pop_front() {
                       s.push(c)
                    } else {
                        panic!("unescape_pattern: \\u is not followed by 8 chars")
                    }
                } 

                let u = u32::from_str_radix(&s, 16).unwrap();
                let c = char::from_u32(u).unwrap(); 
                r.push(c)
            }
            Some(c) => r.push(c),
            None => panic!("unescape pattern. No more characters after \\")
        }
    }
    r
}


/// from [67] code_str = ([^%\\] | "\\" [%\\] | UCHAR)*
fn code_str(i: Span) -> IRes<Span> {
    recognize(many0(alt(
      (recognize(none_of(REQUIRES_ESCAPE_CODE)),
       escaped_code,
       uchar
      )
    )))(i)
}

/// Characters requiring escape sequences in patterns
/// %, 5C = \
const REQUIRES_ESCAPE_CODE: &str = "%\u{5C}";

/// from [67] escaped_code = "\\" [%\\]
fn escaped_code(i: Span) -> IRes<Span> {
    recognize(preceded(token(r"\"), one_of(r#"%\"#)))(i)
}
  
/// `[68]   	<REPEAT_RANGE>	   ::=   	"{" INTEGER ( "," (INTEGER | "*")? )? "}"`
fn repeat_range<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Cardinality> {
    traced("repeat_range", map_error(move |i| {
       let (i, (_, min, maybe_rest_range, _)) = tuple((token("{"), integer, opt(rest_range()), cut(token("}"))))(i)?;
       let cardinality = match maybe_rest_range {
         None => {
           Cardinality::exact(min as i32)
         },
         Some(maybe_max) => match maybe_max {
            None => {
                Cardinality::min_max(min as i32, -1)
            },
            Some(max) => {
                Cardinality::min_max(min as i32, max as i32)
            }
         }
       };
       Ok((i, cardinality))
    }, || ShExParseError::ExpectedRepeatRange))
}

/// From [68] rest_range = "," (INTEGER | "*")?
/// rest_range = "," integer_or_star ?
fn rest_range<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, Option<i32>> {
    traced("rest_range", map_error(move |i| {
      let (i, (_, maybe_max)) = tuple((token_tws(","), opt(integer_or_star)))(i)?;
      Ok((i, maybe_max))
    }, || ShExParseError::ExpectedRestRepeatRange))
}

/// From rest_range, integer_or_star = INTEGER | "*"
fn integer_or_star(i:Span) -> IRes<i32> {
    alt((map(integer, |n| n as i32), 
        (map(token_tws("*"), |_| (-1))
    )))(i)
}

/// `[69]   	<RDF_TYPE>	   ::=   	"a"`
fn rdf_type(i: Span) -> IRes<IriRef> {
    let (i, _) = tag_no_case("a")(i)?;
    Ok((i, IriS::rdf_type().into()))
}

/// `[70]   	<ATPNAME_NS>	   ::=   	"@" PNAME_NS`
fn at_pname_ns(i: Span) -> IRes<ShapeExpr> {
    let (i, (_, _, pname)) = tuple((char('@'), tws0, pname_ns))(i)?;
    let ref_ = Ref::iri_unchecked(pname.fragment());
    Ok((i, ShapeExpr::shape_ref(ref_)))
}

/// `[71]   	<ATPNAME_LN>	   ::=   	"@" PNAME_LN`
fn at_pname_ln(i: Span) -> IRes<ShapeExpr> {
    let (i, (_, _, pname_ln)) = tuple((char('@'), tws0, pname_ln))(i)?;
    Ok((i, ShapeExpr::iri_ref(pname_ln)))
}

/// `[72] <REGEXP>	::= '/' ([^/\\\n\r]
/// | '\\' [nrt\\|.?*+(){}$-\[\]^/]
/// | UCHAR
/// )+ '/' [smix]*`
fn regexp(i: Span) -> IRes<Pattern> {
    let(i, (_, str, _, flags)) = tuple((char('/'), pattern, cut(char('/')), flags))(i)?;
    let flags = flags.fragment();
    let flags = if flags.is_empty() {
        None 
    } else {
        Some(flags.to_string())
    };
    let str = unescape_pattern(str.fragment());
    Ok((i, Pattern { str, flags }))
}

/// unescape characters in pattern strings
fn unescape_pattern(str: &str) -> String {
    let non_escaped = ['t','n','r','-', '\\'];
    let mut queue : VecDeque<_> = str.chars().collect();
    let mut r = String::new();
    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            r.push(c);
            continue;
        }
        match queue.pop_front() {
            Some(c) if non_escaped.contains(&c) => {
                r.push('\\');
                r.push(c)
            }
            Some('u') => {
                let mut s = String::new();
                for _ in 0..4 {
                    if let Some(c) = queue.pop_front() {
                       s.push(c)
                    } else {
                        panic!("unescape_pattern: \\u is not followed by 4 chars")
                    }
                } 

                let u = u32::from_str_radix(&s, 16).unwrap();
                let c = char::from_u32(u).unwrap(); 
                r.push(c)
            }
            Some('U') => {
                let mut s = String::new();
                for _ in 0..8 {
                    if let Some(c) = queue.pop_front() {
                       s.push(c)
                    } else {
                        panic!("unescape_pattern: \\u is not followed by 8 chars")
                    }
                } 

                let u = u32::from_str_radix(&s, 16).unwrap();
                let c = char::from_u32(u).unwrap(); 
                r.push(c)
            }
            Some(c) => r.push(c),
            None => panic!("unescape pattern. No more characters after \\")
        }
    }
    r
}

/// [72b] from [72] pattern = ([^/\\\n\r] | '\\' [nrt\\|.?*+(){}$-\[\]^/] | UCHAR) +
fn pattern(i: Span) -> IRes<Span> {
    recognize(many1(alt(
        (recognize(none_of(REQUIRES_ESCAPE_PATTERN)),
         escaped_pattern,
         uchar)
    )))(i)
}

/// from [72b] escaped_pattern = '\\' [nrt\\|.?*+(){}$-\[\]^/]
fn escaped_pattern(i: Span) -> IRes<Span> {
  recognize(preceded(token(r"\"), one_of(r#"nrt\|.?*+(){}$-[]^/"#)))(i)
}

/// Characters requiring escape sequences in patterns
/// 2F = /, 5C = \, 0A = \n, 0D = \r
const REQUIRES_ESCAPE_PATTERN: &str = "\u{2F}\u{5C}\u{0A}\u{0D}";


/// `from [72] flags = [smix]*`
fn flags(i: Span) -> IRes<Span> {
    recognize(many0(alt((char('s'), char('m'), char('i'), char('x')))))(i)
}

/// `[136s]   	iri	   ::=   	IRIREF | prefixedName`
fn iri(i: Span) -> IRes<IriRef> {
    alt((iri_ref_s, prefixed_name()))(i)
}

fn iri_ref_s(i: Span) -> IRes<IriRef> {
    let (i, iri) = iri_ref(i)?;
    Ok((i, iri.into()))
}

/// `[137s]   	prefixedName	   ::=   	PNAME_LN | PNAME_NS`
fn prefixed_name<'a>() -> impl FnMut(Span<'a>) -> IRes<'a, IriRef> {
   traced("prefixed_name", 
   map_error(move |i| {
     let (i, iri_ref) = alt((pname_ln, pname_ns_iri_ref))(i)?;
     Ok((i, iri_ref))
   }, || ShExParseError::ExpectedPrefixedName
  ))
}

fn pname_ns_iri_ref(i: Span) -> IRes<IriRef> {
    let (i, pname_ns) = pname_ns(i)?;
    Ok((i, IriRef::prefixed(pname_ns.fragment(), "")))
}

/// `[138s]   	blankNode	   ::=   	BLANK_NODE_LABEL`
fn blank_node(i: Span) -> IRes<&str> {
    blank_node_label(i)
}

/// `[142s]   	<BLANK_NODE_LABEL>	   ::=   	"_:" (PN_CHARS_U | [0-9]) ((PN_CHARS | ".")* PN_CHARS)?`
fn blank_node_label(i: Span) -> IRes<&str> {
    let (i, _) = tag("_:")(i)?;
    let (i, label) = recognize(tuple((one_if(is_pn_chars_u_digit), blank_node_label2)))(i)?;
    Ok((i, label.fragment()))
}

fn is_pn_chars_u_digit(c: char) -> bool {
    is_digit(c) || is_pn_chars_u(c)
}

fn is_pn_chars_or_dot(c: char) -> bool {
    c == '.' || is_pn_chars(c)
}

fn blank_node_label2(src: Span) -> IRes<()> {
    match blank_node_label3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                // TODO!!: Original parser had this:
                // But I need to see how to remove the last character of left...
                // Ok(((&src[m.len() - 1..]), ()))
                log::error!("This code is pending review when the last is a '.' {left}");
                Ok((left, ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

fn blank_node_label3(i: Span) -> IRes<Span> {
    take_while(is_pn_chars_or_dot)(i)
}

/// `[19t]   	<INTEGER>	   ::=   	[+-]? [0-9]+`
fn integer(i: Span) -> IRes<isize> {
    let (i, (maybe_sign, digits)) = tuple((opt(one_of("+-")), digits))(i)?;
    let n = match maybe_sign {
        None => digits,
        Some('+') => digits,
        Some('-') => -digits,
        _ => panic!("Internal parser error, Strange maybe_sign: {maybe_sign:?}"),
    };
    Ok((i, n))
}

/// `[20t]   	<DECIMAL>	   ::=   	[+-]? [0-9]* "." [0-9]+`
fn decimal(i: Span) -> IRes<NumericLiteral> {
    map_res(
        pair(
            recognize(preceded(opt(sign), digit0)),
            preceded(token("."), digit1),
        ),
        |(whole, fraction)| {
            Ok::<_, ParseIntError>(NumericLiteral::decimal(whole.parse()?, fraction.parse()?))
        },
    )(i)
}


/// `[21t] <DOUBLE>	::= [+-]? ([0-9]+ "." [0-9]* EXPONENT | "."? [0-9]+ EXPONENT)`
fn double(i: Span) -> IRes<f64> {
    map_res(
        recognize(preceded(
         opt(sign),
         alt((
           recognize(tuple((digit1, token("."), digit0, exponent))),
           recognize(tuple((token("."), digit1, exponent))),
           recognize(pair(digit1, exponent)),
         )),
        )),
     |value: LocatedSpan<&str>| value.parse()
    )(i)
}

pub fn exponent(input: Span) -> IRes<Span> {
    recognize(tuple((one_of("eE"), opt(sign), digit1)))(input)
}

pub fn sign(input: Span) -> IRes<Span> {
    recognize(one_of("+-"))(input)
}

fn digits(i: Span) -> IRes<isize> {
    map_res(digit1, |number: Span| number.parse::<isize>())(i)
}

impl FromExternalError<Span<'_>, ParseIntError> for LocatedParseError {
    fn from_external_error(input: Span, _kind: ErrorKind, e: ParseIntError) -> Self {
        ShExParseError::ParseIntError{ str: input.fragment().to_string(), err: e }.at(input)
    }
}

impl FromExternalError<Span<'_>, ParseFloatError> for LocatedParseError {
    fn from_external_error(input: Span, _kind: ErrorKind, e: ParseFloatError) -> Self {
        ShExParseError::ParseFloatError{ str: input.fragment().to_string(), err: e }.at(input)
    }
}


/// `[141s]   	<PNAME_LN>	   ::=   	PNAME_NS PN_LOCAL`
fn pname_ln(i: Span) -> IRes<IriRef> {
    // This code is different here: https://github.com/vandenoever/rome/blob/047cf54def2aaac75ac4b9adbef08a9d010689bd/src/io/turtle/grammar.rs#L293
    let (i, (prefix, local)) = tuple((pname_ns, pn_local))(i)?;
    Ok((i, IriRef::prefixed(prefix.fragment(), local)))
}

/// `[77]   	<PN_LOCAL>	   ::=   	(PN_CHARS_U | ":" | [0-9] | PLX) (PN_CHARS | "." | ":" | PLX)`
fn pn_local(i: Span) -> IRes<&str> {
    let (i, cs) = recognize(tuple((alt((one_if(is_pn_local_start), plx)), pn_local2)))(i)?;
    Ok((i, cs.fragment()))
}

fn is_pn_local_start(c: char) -> bool {
    c == ':' || is_digit(c) || is_pn_chars_u(c)
}

fn pn_local2(src: Span) -> IRes<()> {
    match pn_local3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                // TODO!!: Original parser had this:
                // But I need to see how to remove the last character of left...
                // Ok(((&src[m.len() - 1..]), ()))
                log::error!("This code is pending review when the last is a '.' {left}");
                Ok((left, ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

fn pn_local3(i: Span) -> IRes<Span> {
    recognize(many0(alt((pn_chars_colon, plx, char_dot))))(i)
}

fn pn_chars_colon(i: Span) -> IRes<Span> {
    take_while1(is_pn_chars_colon)(i)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

fn plx(i: Span) -> IRes<Span> {
    alt((percent, pn_local_esc))(i)
}

/// ShEx rule
/// `[173s]   	<PN_LOCAL_ESC>	   ::=   	"\\" ( "_" | "~" | "." | "-" | "!" | "$" | "&" | "'" |
///                "(" | ")" | "*" | "+" | "," | ";" | "=" | "/" | "?" | "#" | "@" | "%" )``
fn pn_local_esc(i: Span) -> IRes<Span> {
    recognize(tuple((
        char('\\'),
        one_if(|c| "_~.-!$&'()*+,;=/?#@%".contains(c)),
    )))(i)
}

fn percent(i: Span) -> IRes<Span> {
    recognize(tuple((char('%'), one_if(is_hex), one_if(is_hex))))(i)
}

fn is_hex(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

/// `[18t]   	<IRIREF>	   ::=   	"<" ([^#0000- <>\"{}|^`\\] | UCHAR)* ">"`
fn iri_ref(i: Span) -> IRes<IriS> {
    let (i, str) = delimited(char('<'), take_while(is_iri_ref), char('>'))(i)?;
    log::debug!("Iri_ref {str}");
    Ok((i, IriS::new_unchecked(str.fragment())))
}

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

/// [140s] `<PNAME_NS>	   ::=   	PN_PREFIX? ":"`
fn pname_ns(i: Span) -> IRes<Span> {
    let (i, (maybe_pn_prefix, _)) = tuple((opt(pn_prefix), char(':')))(i)?;
    Ok((i, maybe_pn_prefix.unwrap_or(Span::from(""))))
}

/// [168s] `<PN_PREFIX>	::= PN_CHARS_BASE ( (PN_CHARS | ".")* PN_CHARS )?`
fn pn_prefix(i: Span) -> IRes<Span> {
    /*let (i, (pn_chars_base, maybe_rest)) = tuple((pn_chars_base, opt(rest_pn_prefix)))(i)?;
    let mut s: String = pn_chars_base.to_string();
    Ok((i, s.as_str()))*/
    recognize(tuple((
        satisfy(is_pn_chars_base),
        take_while(is_pn_chars),
        rest_pn_chars, // fold_many0(tuple((char('.'), take_while1(is_pn_chars))), || (), |_, _| ()),
    )))(i)
}

fn rest_pn_chars(i: Span) -> IRes<Vec<Span>> {
    let (i, vs) = fold_many0(
        tuple((char_dot, take_while1(is_pn_chars))),
        Vec::new,
        |mut cs: Vec<Span>, (c, rs)| {
            cs.push(c);
            cs.push(rs);
            cs
        },
    )(i)?;
    Ok((i, vs))
}

fn pn_chars_base(i: Span) -> IRes<char> {
    satisfy(is_pn_chars_base)(i)
}

/// From [168s] rest_pn_prefix = (PN_CHARS | ".")* PN_CHARS )
fn rest_pn_prefix(i: Span) -> IRes<&str> {
    let (i, (vs, cs)) = tuple((many0(alt((pn_chars, char_dot))), pn_chars))(i)?;
    // TODO...collect vs
    Ok((i, cs.fragment()))
}

fn char_dot(i: Span) -> IRes<Span> {
    recognize(char('.'))(i)
}

fn pn_chars(i: Span) -> IRes<Span> {
    one_if(is_pn_chars)(i)
}

/// [164s] `<PN_CHARS_BASE>	   ::=   	   [A-Z] | [a-z]`
///        `                   | [#00C0-#00D6] | [#00D8-#00F6] | [#00F8-#02FF]`
///        `                   | [#0370-#037D] | [#037F-#1FFF]`
///        `                   | [#200C-#200D] | [#2070-#218F] | [#2C00-#2FEF]`
///        `                   | [#3001-#D7FF] | [#F900-#FDCF] | [#FDF0-#FFFD]`
///        `                   | [#10000-#EFFFF]`
fn is_pn_chars_base(c: char) -> bool {
    is_alpha(c)
        || in_range(c, 0xC0, 0x00D6)
        || in_range(c, 0x00D8, 0x00F6)
        || in_range(c, 0x00F8, 0x02FF)
        || in_range(c, 0x0370, 0x037D)
        || in_range(c, 0x037F, 0x1FFF)
        || in_range(c, 0x200C, 0x200D)
        || in_range(c, 0x2070, 0x218F)
        || in_range(c, 0x2C00, 0x2FEF)
        || in_range(c, 0x3001, 0xD7FF)
        || in_range(c, 0xF900, 0xFDCF)
        || in_range(c, 0xFDF0, 0xFFFD)
        || in_range(c, 0x10000, 0xEFFFF)
}

/// `[165s]   	<PN_CHARS_U>	   ::=   	PN_CHARS_BASE | "_"`
fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

/// `[167s] <PN_CHARS>	::= PN_CHARS_U | "-" | [0-9]`
/// ` | [#00B7] | [#0300-#036F] | [#203F-#2040]`
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c)
        || c == '-'
        || is_digit(c)
        || c == 0xB7 as char
        || in_range(c, 0x0300, 0x036F)
        || in_range(c, 0x203F, 0x2040)
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn in_range(c: char, lower: u32, upper: u32) -> bool {
    c as u32 >= lower && c as u32 <= upper
}

/// Take one character if it fits the function
fn one_if<'a, F: Fn(char) -> bool>(
    f: F,
) -> impl Fn(Span<'a>) -> IRes<Span<'a>> {
    move |i| {
        if let Some(c) = i.chars().next() {
            if f(c) {
                Ok(i.take_split(1))
            } else {
                Err(Err::Error(error_position!(i, ErrorKind::OneOf)))
            }
        } else {
            // Err(Err::Incomplete(Needed::new(1)))
            Err(Err::Error(error_position!(i, ErrorKind::OneOf)))
        }
    }
}

fn symbol<'a>(value: &'a str) -> impl FnMut(Span<'a>) -> IRes<'a, ()> {
    move |i| {
        let (i, _) = tag_no_case(value)(i)?;
        Ok((i, ()))
    }
}

fn many1_sep<'a, O, O2, F, G, H>(
    mut parser_many: F,
    mut sep: G,
    maker: H,
    mut i: Span<'a>,
) -> IRes<'a, O2>
where
    F: FnMut(Span<'a>) -> IRes<'a, O>,
    G: FnMut(Span<'a>) -> IRes<'a, ()>,
    H: Fn(Vec<O>) -> O2,
{
    let mut vs = Vec::new();

    // skip tws
    if let Ok((left, _)) = tws0(i) {
        i = left;
    }
    match parser_many(i) {
        Ok((left, v)) => {
            vs.push(v);
            i = left;
        }
        Err(e) => return Err(e),
    }
    loop {
        if let Ok((left, _)) = tws0(i) {
            i = left;
        }

        match sep(i) {
            Ok((left, _)) => {
                i = left;
            }
            _ => return Ok((i, maker(vs))),
        }

        if let Ok((left, _)) = tws0(i) {
            i = left;
        }

        match parser_many(i) {
            Ok((left, v)) => {
                vs.push(v);
                i = left;
            }
            _ => return Ok((i, maker(vs))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ShExStatement;

    /*#[test]
    fn test_comment() {
        assert_eq!(comment(Span::new("#\r\na")), Ok((Span::new("\na"), ())));
        assert_eq!(comment("#\n\ra"), Ok((("\ra"), (""))));
        // assert_eq!(comment(""), Err(Err::Error(("".as_ref(), ErrorKind::Char))));
        assert_eq!(comment("#"), Ok(("", "")));
        assert_eq!(comment("#abc"), Ok(("", "abc")));
        assert_eq!(comment("#\n\n"), Ok(("\n", "")));
    }

    #[test]
    fn test_prefix_id_with_dots() {
        assert_eq!(
            prefix_decl("prefix a.b.c: <urn>"),
            Ok((
                "",
                ShExStatement::PrefixDecl {
                    alias: "a.b.c",
                    iri: IriS::new_unchecked("urn")
                }
            ))
        );
    }

    #[test]
    fn test_prefix_id() {
        assert_eq!(
            prefix_decl("prefix a: <urn>"),
            Ok((
                "",
                ShExStatement::PrefixDecl {
                    alias: "a",
                    iri: IriS::new_unchecked("urn")
                }
            ))
        );
    }

    #[test]
    fn test_prefix_basic() {
        assert_eq!(
            prefix_decl("prefix e: <http://example.org/>"),
            Ok((
                "",
                ShExStatement::PrefixDecl {
                    alias: "e",
                    iri: IriS::new_unchecked("http://example.org/")
                }
            ))
        );
    }

    #[test]
    fn test_directives_prefix_decl() {
        assert_eq!(
            directives("prefix e: <http://example.org/>"),
            Ok((
                "",
                vec![ShExStatement::PrefixDecl {
                    alias: "e",
                    iri: IriS::new_unchecked("http://example.org/")
                }]
            ))
        );
    }

    #[test]
    fn test_shex_statement_prefix_decl() {
        assert_eq!(
            shex_statement("prefix e: <http://example.org/>"),
            Ok((
                "",
                vec![ShExStatement::PrefixDecl {
                    alias: "e",
                    iri: IriS::new_unchecked("http://example.org/")
                }]
            ))
        );
    }

    #[test]
    fn test_shape_expr_label() {
        assert_eq!(
            shape_expr_label("<http://example.org/S>"),
            Ok(("", Ref::iri_unchecked("http://example.org/S")))
        );
    }

    #[test]
    fn test_shape_expr_dot() {
        assert_eq!(shape_expression("."), Ok(("", ShapeExpr::any())));
    }

    #[test]
    fn test_shape_expr_triple_constraint() {
        let p = IriRef::try_from("http://example.org/p").unwrap();

        assert_eq!(
            shape_expression("{ <http://example.org/p> . }"),
            Ok((
                "",
                ShapeExpr::shape(
                    Shape::default().with_expression(TripleExpr::triple_constraint(
                        p,
                        Some(ShapeExpr::any()),
                        None,
                        None
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_shape_def_triple_constraint() {
        let p = IriRef::try_from("http://example.org/p").unwrap();

        assert_eq!(
            shape_definition("{ <http://example.org/p> . }"),
            Ok((
                "",
                ShapeExpr::shape(
                    Shape::default().with_expression(TripleExpr::triple_constraint(
                        p,
                        Some(ShapeExpr::any()),
                        None,
                        None
                    ))
                )
            ))
        );
    }

    #[test]
    fn test_triple_expression() {
        let p = IriRef::try_from("http://example.org/p").unwrap();

        assert_eq!(
            triple_expression("<http://example.org/p> . ?"),
            Ok((
                "",
                TripleExpr::triple_constraint(p, Some(ShapeExpr::any()), Some(0), Some(1))
            ))
        );
    }

    #[test]
    fn test_triple_constraint() {
        let p = IriRef::try_from("http://example.org/p").unwrap();

        assert_eq!(
            unary_triple_expr_opt1("<http://example.org/p> . ?"),
            Ok((
                "",
                TripleExpr::triple_constraint(p, Some(ShapeExpr::any()), Some(0), Some(1))
            ))
        );
    }

    #[test]
    fn test_shape_expr_and() {
        let p = IriRef::try_from("http://example.org/p").unwrap();
        let q = IriRef::try_from("http://example.org/q").unwrap();
        let se1 = ShapeExpr::shape(Shape::default().with_expression(
            TripleExpr::triple_constraint(p, Some(ShapeExpr::any()), None, None),
        ));
        let se2 = ShapeExpr::shape(Shape::default().with_expression(
            TripleExpr::triple_constraint(q, Some(ShapeExpr::any()), None, None),
        ));
        assert_eq!(
            shape_expression("{ <http://example.org/p> . } AND { <http://example.org/q> . }"),
            Ok(("", ShapeExpr::and(vec![se1, se2])))
        );
    }*/

    #[test]
    fn test_empty_shex_statement() {
        let (_, result) = shex_statement()(Span::new("")).unwrap();
        let expected = Vec::new();
        assert_eq!(result, expected)
    }

    /*#[test]
    fn test_incomplete() {
        use super::*;

        fn m(i: &str) -> IResult<&str, ShapeExpr> {
            let (i, s) = shape_definition(i)?;
            Ok((i, s))
        }
        let te = TripleExpr::triple_constraint(
            IriRef::prefixed("","p"), 
            Some(ShapeExpr::iri_ref(IriRef::prefixed("", "User"))), 
            Some(0), 
            Some(-1));
        assert_eq!(m("{ :p @:User * ; }"), Ok(((""), 
          ShapeExpr::shape(Shape::new(None, None, Some(te)))
          )))
    }*/
}
