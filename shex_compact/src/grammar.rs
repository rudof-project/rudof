use iri_s::IriS;
use std::{fmt::Debug, num::ParseIntError};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1, is_not},
    character::complete::{char, digit1, one_of, satisfy, multispace1},
    combinator::{fail, map, map_res, opt, recognize, value, cut},
    error::{ErrorKind, ParseError, FromExternalError},
    error_position,
    multi::{fold_many0, many0, many1},
    sequence::{delimited, tuple, pair},
    Err, IResult, InputTake, 
};
use shex_ast::{
    object_value::ObjectValue, value_set_value::ValueSetValue, Annotation, IriRef, NodeConstraint,
    Ref, SemAct, Shape, ShapeExpr, TripleExpr, XsFacet,
};
use log;
use thiserror::Error;

use crate::{Cardinality, Qualifier, ShExStatement, ParseError as ShExParseError};
use nom_locate::LocatedSpan;


// Some definitions borrowed from [Nemo](https://github.com/knowsys/nemo/blob/main/nemo/src/io/parser/types.rs)

pub type IntermediateResult<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

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
    mut parser: impl FnMut(Span<'a>) -> IntermediateResult<'a, T> + 'a,
    mut error: impl FnMut() -> ShExParseError + 'a,
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

/*fn not_eol(c: char) -> bool {
    c != '\n' && c != '\r'
}

fn comment(i: &str) -> IResult<&str, &str> {
    let (i, _) = char('#')(i)?;
    let (i, comment) = take_while(not_eol)(i)?;
    if i.is_empty() {
        Ok((i, comment))
    } else {
        // remove one \n or \r
        Ok((&i[1..], comment))
    }
}*/

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
    map_error(tag(token), || ShExParseError::ExpectedToken(token.to_string()))
}

/// A combinator that creates a parser for a specific token,
/// surrounded by trailing whitespace or comments.
pub fn token_tws<'a>(
    token: &'a str,
) -> impl FnMut(Span<'a>) -> IntermediateResult<Span<'a>> {
    map_error(
        delimited(tws0, tag(token), tws0),
        || ShExParseError::ExpectedToken(token.to_string()),
    )
}


/// whitespace that may contain comments
/*pub fn tws(i: &str) -> IResult<&str, ()> {
    fold_many0(
        alt((map(one_of(" \t\n\r"), |_| ()), map(comment, |_| ()))),
        || (),
        |_, _| (),
    )(i)
} */

/// `[1] shexDoc	   ::=   	directive* ((notStartAction | startActions) statement*)?`
pub fn shex_statement<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, Vec<ShExStatement>> {
    traced("shex_statement", 
    move |i| {
    let (i, (ds, _, maybe_sts)) = tuple((directives, tws1, opt(rest_shex_statements)))(i)?;
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
pub fn rest_shex_statements(i: Span) -> IntermediateResult<Vec<ShExStatement>> {
    let (i, (s, _, ss, _)) = tuple((alt((not_start_action, start_actions)), tws1, statements, tws0))(i)?;
    let mut rs = vec![s];
    rs.extend(ss);
    Ok((i, rs))
}

pub fn directives(i: Span) -> IntermediateResult<Vec<ShExStatement>> {
    let (i, vs) = many0(tuple((directive, tws1)))(i)?;
    let mut rs = Vec::new();
    for v in vs {
        let (d, _) = v;
        rs.push(d);
    }
    Ok((i, rs))
}

pub fn statements(i: Span) -> IntermediateResult<Vec<ShExStatement>> {
    many0(statement)(i)
}

/// [2] `directive	   ::=   	baseDecl | prefixDecl | importDecl`
pub fn directive(i: Span) -> IntermediateResult<ShExStatement> {
    alt((
        // base_decl,
        prefix_decl(),
        // import_decl
    ))(i)
}

/// [4] `prefixDecl	   ::=   	"PREFIX" PNAME_NS IRIREF`
fn prefix_decl<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, ShExStatement> {
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

/// `[5]   	notStartAction	   ::=   	start | shapeExprDecl`
fn not_start_action(i: Span) -> IntermediateResult<ShExStatement> {
    alt((start, shape_expr_decl()))(i)
}

/// `[6]   	start	   ::=   	"start" '=' inlineShapeExpression`
fn start(i: Span) -> IntermediateResult<ShExStatement> {
    let (i, (_, _, _, _, se)) = tuple((
        tag_no_case("START"),
        tws0,
        char('='),
        tws0,
        inline_shape_expression,
    ))(i)?;
    Ok((i, ShExStatement::StartDecl { shape_expr: se }))
}

/// `[7]   	startActions	   ::=   	codeDecl+`
fn start_actions(i: Span) -> IntermediateResult<ShExStatement> {
    let (i, cs) = many1(code_decl)(i)?;
    Ok((i, ShExStatement::StartActions { actions: cs }))
}

/// `[8]   	statement	   ::=   	directive | notStartAction`
fn statement(i: Span) -> IntermediateResult<ShExStatement> {
    alt((directive, not_start_action))(i)
}

/// `[9]   	shapeExprDecl	   ::=   	shapeExprLabel (shapeExpression | "EXTERNAL")`
fn shape_expr_decl<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, ShExStatement> {
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

fn shape_expr_or_external(i: Span) -> IntermediateResult<ShapeExpr> {
    alt((shape_expression, external))(i)
}

fn external(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, _) = tag_no_case("EXTERNAL")(i)?;
    Ok((i, ShapeExpr::external()))
}

/// `[10]   	shapeExpression	   ::=   	shapeOr`
fn shape_expression(i: Span) -> IntermediateResult<ShapeExpr> {
    shape_or(i)
}

/// `[11]   	inlineShapeExpression	   ::=   	inlineShapeOr`
fn inline_shape_expression(i: Span) -> IntermediateResult<ShapeExpr> {
    inline_shape_or(i)
}

/// `[12]   	shapeOr	   ::=   	shapeAnd ("OR" shapeAnd)*`
fn shape_or<'a>(i: Span<'a>) -> IntermediateResult<'a, ShapeExpr> {
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
fn inline_shape_or(i: Span) -> IntermediateResult<ShapeExpr> {
    many1_sep(inline_shape_and, symbol("OR"), make_shape_or, i)
}

/// `[14]   	shapeAnd	   ::=   	shapeNot ("AND" shapeNot)*``
fn shape_and(i: Span) -> IntermediateResult<ShapeExpr> {
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
fn inline_shape_and(i: Span) -> IntermediateResult<ShapeExpr> {
    many1_sep(inline_shape_not, symbol("AND"), make_shape_and, i)
}

/// `[16]   	shapeNot	   ::=   	"NOT"? shapeAtom`
fn shape_not(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, maybe) = opt(symbol("NOT"))(i)?;
    let (i, se) = shape_atom(i)?;
    match maybe {
        None => Ok((i, se)),
        Some(_) => Ok((i, ShapeExpr::not(se))),
    }
}

/// `[17]   	inlineShapeNot	   ::=   	"NOT"? inlineShapeAtom`
fn inline_shape_not(i: Span) -> IntermediateResult<ShapeExpr> {
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
fn shape_atom(i: Span) -> IntermediateResult<ShapeExpr> {
    alt((
        // Pending
        // non_lit_shape,
        lit_node_constraint_shape_expr,
        shape_opt_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

/// `From [18] `shape_opt_non_lit ::= shapeOrRef nonLitNodeConstraint?`
fn shape_opt_non_lit(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, se) = shape_or_ref()(i)?;
    let (i, maybe_non_lit) = opt(non_lit_node_constraint)(i)?;
    match maybe_non_lit {
        None => Ok((i, se)),
        Some(nl) => Ok((i, ShapeExpr::and(vec![se, ShapeExpr::node_constraint(nl)]))),
    }
}

/// `[20]   	inlineShapeAtom	   ::= nonLitNodeConstraint inlineShapeOrRef?`
/// `                                | litNodeConstraint`
/// `                                | inlineShapeOrRef nonLitNodeConstraint?`
/// `                                | '(' shapeExpression ')'`
/// `                                | '.'`
fn inline_shape_atom(i: Span) -> IntermediateResult<ShapeExpr> {
    alt((
        // Pending
        // nonlit_inline_shape,
        lit_node_constraint_shape_expr,
        inline_shape_or_ref_opt_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

/// `from [20] `inline_shape_or_ref_opt_non_lit ::= inlineShapeOrRef nonLitNodeConstraint?`
fn inline_shape_or_ref_opt_non_lit(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, se) = inline_shape_or_ref(i)?;
    let (i, maybe_non_lit) = opt(non_lit_node_constraint)(i)?;
    match maybe_non_lit {
        None => Ok((i, se)),
        Some(nl) => Ok((i, ShapeExpr::and(vec![se, ShapeExpr::node_constraint(nl)]))),
    }
}

fn lit_node_constraint_shape_expr(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, nc) = lit_node_constraint(i)?;
    Ok((i, ShapeExpr::NodeConstraint(nc)))
}

fn paren_shape_expr(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (_, _, se, _, _)) = tuple((char('('), tws0, shape_expression, tws0, char(')')))(i)?;
    Ok((i, se))
}

fn dot(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (_, _)) = tuple((tws0, char('.')))(i)?;
    Ok((i, ShapeExpr::any()))
}

/// `[21]   	shapeOrRef	   ::=   	   shapeDefinition | shapeRef`
fn shape_or_ref<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, ShapeExpr> {
  traced("shape_or_ref", 
  map_error(
    move |i| { alt((shape_definition, shape_ref))(i) },
    || ShExParseError::ExpectedShapeOrRef
  ))  
}

/// `[22]   	inlineShapeOrRef	   ::=   	   inlineShapeDefinition | shapeRef`
fn inline_shape_or_ref(i: Span) -> IntermediateResult<ShapeExpr> {
    alt((inline_shape_definition, shape_ref))(i)
}

/// `[23]   	shapeRef	   ::=   	   ATPNAME_LN | ATPNAME_NS | '@' shapeExprLabel`
fn shape_ref(i: Span) -> IntermediateResult<ShapeExpr> {
    alt((at_pname_ln, at_pname_ns, at_shape_expr_label))(i)
}

fn at_shape_expr_label(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (_, label)) = tuple((char('@'), shape_expr_label))(i)?;
    Ok((i, ShapeExpr::shape_ref(label)))
}

/// `[24]   	litNodeConstraint	   ::=   	   "LITERAL" xsFacet*
/// | datatype xsFacet*
/// | valueSet xsFacet*
/// | numericFacet+`
fn lit_node_constraint(i: Span) -> IntermediateResult<NodeConstraint> {
    alt((
        literal_facets,
        datatype_facets,
        value_set_facets,
        // numeric_facets,
    ))(i)
}

fn literal_facets(i: Span) -> IntermediateResult<NodeConstraint> {
    let (i, (_, _, facets)) = tuple((tag_no_case("LITERAL"), tws1, facets))(i)?;
    Ok((i, NodeConstraint::new().with_xsfacets(facets)))
}

fn datatype_facets(i: Span) -> IntermediateResult<NodeConstraint> {
    let (i, (dt, _, facets)) = tuple((datatype, tws1, facets))(i)?;
    Ok((i, dt.with_xsfacets(facets)))
}

fn value_set_facets(i: Span) -> IntermediateResult<NodeConstraint> {
    let (i, (vs, _, facets)) = tuple((value_set, tws1, facets))(i)?;
    Ok((i, vs.with_xsfacets(facets)))
}

fn facets(i: Span) -> IntermediateResult<Vec<XsFacet>> {
    many0(xs_facet)(i)
}

/// `[25]   	nonLitNodeConstraint	   ::=   	   nonLiteralKind stringFacet*`
/// `| stringFacet+`
fn non_lit_node_constraint(i: Span) -> IntermediateResult<NodeConstraint> {
    // Pending
    fail(i)
}

/// `[27]   	xsFacet	   ::=   	stringFacet | numericFacet`
fn xs_facet(i: Span) -> IntermediateResult<XsFacet> {
    alt((
        string_facet,
        // numeric_facet
    ))(i)
}

/// `[28]   	stringFacet	   ::=   	   stringLength INTEGER`
/// `| REGEXP`
fn string_facet(i: Span) -> IntermediateResult<XsFacet> {
    alt((
        string_length,
        // regexp
    ))(i)
}

// `[29]   	stringLength	   ::=   	"LENGTH" | "MINLENGTH" | "MAXLENGTH"`
fn string_length(i: Span) -> IntermediateResult<XsFacet> {
    alt((min_length, max_length, length))(i)
}

fn min_length(i: Span) -> IntermediateResult<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("MIN_LENGTH"), tws1, pos_integer))(i)?;
    Ok((i, XsFacet::min_length(n)))
}

fn max_length(i: Span) -> IntermediateResult<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("MAX_LENGTH"), tws1, pos_integer))(i)?;
    Ok((i, XsFacet::max_length(n)))
}

fn length(i: Span) -> IntermediateResult<XsFacet> {
    let (i, (_, _, n)) = tuple((tag_no_case("LENGTH"), tws0, pos_integer))(i)?;
    Ok((i, XsFacet::length(n)))
}

fn pos_integer(i: Span) -> IntermediateResult<usize> {
    let (i, n) = integer(i)?;
    let u: usize;
    if n < 0 {
        Err(Err::Error(error_position!(i, ErrorKind::Digit)))
    } else {
        u = n as usize;
        Ok((i, u))
    }
}

/// `[33]   	shapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}' annotation* semanticActions`
fn shape_definition(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (qualifiers, _, _, _, maybe_triple_expr, _, _, _, annotations, _, sem_actions)) =
        tuple((
            qualifiers,
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
}

/// `[34]   	inlineShapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}'`
fn inline_shape_definition(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (qualifiers, _, _, _, maybe_triple_expr, _, _)) = tuple((
        qualifiers,
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

fn annotations(i: Span) -> IntermediateResult<Vec<Annotation>> {
    many0(annotation)(i)
}

fn qualifiers(i: Span) -> IntermediateResult<Vec<Qualifier>> {
    many0(qualifier)(i)
}

fn qualifier(i: Span) -> IntermediateResult<Qualifier> {
    alt((closed, extra_property_set))(i)
}

fn closed(i: Span) -> IntermediateResult<Qualifier> {
    let (i, _) = tag_no_case("CLOSED")(i)?;
    Ok((i, Qualifier::Closed))
}

/// `[35]   	extraPropertySet	   ::=   	"EXTRA" predicate+`
fn extra_property_set(i: Span) -> IntermediateResult<Qualifier> {
    let (i, (_, ps)) = tuple((tag_no_case("EXTRA"), many1(predicate)))(i)?;
    Ok((i, Qualifier::Extra(ps)))
}

/// `[36]   	tripleExpression	   ::=   	oneOfTripleExpr`
fn triple_expression(i: Span) -> IntermediateResult<TripleExpr> {
    one_of_triple_expr(i)
}

/// `[37]   	oneOfTripleExpr	   ::=   	groupTripleExpr | multiElementOneOf`
fn one_of_triple_expr(i: Span) -> IntermediateResult<TripleExpr> {
    alt((group_triple_expr, multi_element_one_of))(i)
}

/// `[38]   	multiElementOneOf	   ::=   	groupTripleExpr ('|' groupTripleExpr)+`
fn multi_element_one_of(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (te1, _, tes)) = tuple((group_triple_expr, tws0, rest_group_triple_expr))(i)?;
    let mut rs = vec![te1];
    for te in tes {
        rs.push(te);
    }
    let te = TripleExpr::one_of(rs);
    Ok((i, te))
}

/// From [38] rest_group_triple_expr = ('|' groupTripleExpr)+
fn rest_group_triple_expr(i: Span) -> IntermediateResult<Vec<TripleExpr>> {
    let (i, vs) = many1(tuple((char('|'), tws0, group_triple_expr)))(i)?;
    let mut tes = Vec::new();
    for v in vs {
        let (_, _, te) = v;
        tes.push(te);
    }
    Ok((i, tes))
}

/// `[40]   	groupTripleExpr	   ::=   	singleElementGroup | multiElementGroup`
fn group_triple_expr(i: Span) -> IntermediateResult<TripleExpr> {
    alt((multi_element_group, single_element_group))(i)
}

/// `[41]   	singleElementGroup	   ::=   	unaryTripleExpr ';'?`
fn single_element_group(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (te, _, _)) = tuple((unary_triple_expr, tws0, opt(char(';'))))(i)?;
    Ok((i, te))
}

/// `[42]   	multiElementGroup	   ::=   	unaryTripleExpr (';' unaryTripleExpr)+ ';'?`
fn multi_element_group(i: Span) -> IntermediateResult<TripleExpr> {
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
fn rest_unary_triple_expr(i: Span) -> IntermediateResult<Vec<TripleExpr>> {
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
fn unary_triple_expr(i: Span) -> IntermediateResult<TripleExpr> {
    alt((unary_triple_expr_opt1, include_))(i)
}

/// From [41] unary_triple_expr_opt1 = ('$' tripleExprLabel)? (tripleConstraint | bracketedTripleExpr)
fn unary_triple_expr_opt1(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (maybe_label, _, te)) = tuple((
        triple_expr_label_opt,
        tws0,
        alt((triple_constraint, bracketed_triple_expr)),
    ))(i)?;
    // Pending: Process maybe_label
    Ok((i, te))
}

// From unary_triple_expr_opt1
fn triple_expr_label_opt(i: Span) -> IntermediateResult<Option<Ref>> {
    let (i, maybe_ts) = opt(tuple((char('$'), tws0, triple_expr_label)))(i)?;
    let maybe_label = match maybe_ts {
        Some((_, _, r)) => Some(r),
        None => None,
    };
    Ok((i, maybe_label))
}

/// `[44]   	bracketedTripleExpr	   ::=   	'(' tripleExpression ')' cardinality? annotation* semanticActions`
fn bracketed_triple_expr(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (_, _, te, _, _, _, maybe_card, _, annotations, _, sem_acts)) = tuple((
        char('('),
        tws0,
        triple_expression,
        tws0,
        char(')'),
        tws0,
        opt(cardinality),
        tws0,
        annotations,
        tws0,
        semantic_actions,
    ))(i)?;
    // Pending
    // let te = te.with_card(maybe_card).with_annotations(annotations).with_sem_acts(sem_acts);
    Ok((i, te))
}

/// `[45]   	tripleConstraint	   ::=   	senseFlags? predicate inlineShapeExpression cardinality? annotation* semanticActions`
fn triple_constraint(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (predicate, _, se, _, maybe_card, _)) = tuple((
        predicate,
        tws0,
        inline_shape_expression,
        tws0,
        opt(cardinality),
        tws0
    ))(i)?;
    let (min, max) = match maybe_card {
        None => (None, None),
        Some(card) => (card.min(), card.max()),
    };
    Ok((
        i,
        TripleExpr::triple_constraint(predicate, Some(se), min, max),
    ))
}

/// `[46]   	cardinality	   ::=   	'*' | '+' | '?' | REPEAT_RANGE`
fn cardinality(i: Span) -> IntermediateResult<Cardinality> {
    alt((
        plus, star, optional,
        // Pending
        // repeat_range
    ))(i)
}

fn plus(i: Span) -> IntermediateResult<Cardinality> {
    let (i, _) = char('+')(i)?;
    Ok((i, Cardinality::plus()))
}

fn star(i: Span) -> IntermediateResult<Cardinality> {
    let (i, _) = char('*')(i)?;
    Ok((i, Cardinality::star()))
}

fn optional(i: Span) -> IntermediateResult<Cardinality> {
    let (i, _) = char('?')(i)?;
    Ok((i, Cardinality::optional()))
}

/// `[48]   	valueSet	   ::=   	'[' valueSetValue* ']'`
fn value_set(i: Span) -> IntermediateResult<NodeConstraint> {
    let (i, (_, _, vs, _, _)) = tuple((char('['), tws0, many0(value_set_value), tws0, char(']')))(i)?;
    Ok((i, NodeConstraint::new().with_values(vs)))
}

/// `[49]   	valueSetValue	   ::=   	iriRange | literalRange | languageRange`
/// `                               | exclusion+`
fn value_set_value(i: Span) -> IntermediateResult<ValueSetValue> {
    alt((
        // Pending
        iri_range,
        // literal_range,
        // language_range,
        // exclusion_plus
    ))(i)
}

type Exclusion = ();

/// `[51]   	iriRange	   ::=   	   iri ('~' exclusion*)?`
fn iri_range(i: Span) -> IntermediateResult<ValueSetValue> {
    let (i, (iri, _, maybe_exc)) = tuple((iri, tws0, opt(char_exclusion)))(i)?;
    // Pending char_exclusion
    let vs = ValueSetValue::iri(iri);
    Ok((i, vs))
}

fn char_exclusion(i: Span) -> IntermediateResult<Vec<Exclusion>> {
    let (i, (_, _, es)) = tuple((char('~'), tws0, many0(exclusion)))(i)?;
    Ok((i, es))
}

/// `[50]   	exclusion	   ::=   	'.' '-' (iri | literal | LANGTAG) '~'?`
fn exclusion(i: Span) -> IntermediateResult<Exclusion> {
    let (i, (_, _, _, _, e, _, maybe_tilde)) =
        tuple((char('.'), tws0, char('-'), tws0, exc, tws0, opt(char('~'))))(i)?;
    Ok((i, ()))
}

/// `from [50] exc = iri | literal | LANGTAG`
fn exc(i: Span) -> IntermediateResult<Exclusion> {
    let (i, e) = alt((
        iri,
        // literal,
        // lang_tag
    ))(i)?;
    Ok((i, ()))
}

/// `[57]   	include	   ::=   	'&' tripleExprLabel`
fn include_(i: Span) -> IntermediateResult<TripleExpr> {
    let (i, (_, _, tel)) = tuple((char('&'), tws0, triple_expr_label))(i)?;
    // Pending: We should add a temporary reference to a triple_expr_label which should be dereferenced in a second step
    todo!()
    // Ok((i, tel))
}

/// `[58]   	annotation	   ::=   	"//" predicate (iri | literal)`
fn annotation(i: Span) -> IntermediateResult<Annotation> {
    let (i, (_, p, o)) = tuple((tag("//"), predicate, iri_or_literal))(i)?;
    Ok((i, Annotation::new(p.into(), o)))
}

fn iri_or_literal(i: Span) -> IntermediateResult<ObjectValue> {
    // Pending literal
    let (i, iri) = iri(i)?;
    Ok((i, ObjectValue::IriRef(iri.into())))
}

/// `[59]   	semanticActions	   ::=   	codeDecl*`
fn semantic_actions(i: Span) -> IntermediateResult<Option<Vec<SemAct>>> {
    let (i, sas) = many0(code_decl)(i)?;
    if sas.is_empty() {
        Ok((i, None))
    } else {
        Ok((i, Some(sas)))
    }
}

/// `[60]   	codeDecl	   ::=   	'%' iri (CODE | '%')`
fn code_decl(i: Span) -> IntermediateResult<SemAct> {
    let (i, (_, iri, code)) = tuple((char('%'), iri, code_or_percent))(i)?;
    Ok((i, SemAct::new(IriRef::from(iri), code)))
}

fn code_or_percent(i: Span) -> IntermediateResult<Option<String>> {
    let (i, maybe_code) = alt((code, percent_code))(i)?;
    Ok((i, maybe_code))
}

fn percent_code(i: Span) -> IntermediateResult<Option<String>> {
    let (i, _) = char('%')(i)?;
    Ok((i, None))
}

/// `[61]   	predicate	   ::=   	iri | RDF_TYPE`
fn predicate(i: Span) -> IntermediateResult<IriRef> {
    alt((iri, rdf_type))(i)
}

/// `[62]   	datatype	   ::=   	iri`
fn datatype(i: Span) -> IntermediateResult<NodeConstraint> {
    let (i, iri_ref) = iri(i)?;
    Ok((i, NodeConstraint::new().with_datatype(iri_ref)))
}

/// `[63]   	shapeExprLabel	   ::=   	iri | blankNode`
fn shape_expr_label(i: Span) -> IntermediateResult<Ref> {
    let (i, ref_) = alt((iri_as_ref, blank_node_ref))(i)?;
    Ok((i, ref_))
}
fn iri_as_ref(i: Span) -> IntermediateResult<Ref> {
    let (i, iri_ref) = iri(i)?;
    Ok((i, Ref::iri_ref(iri_ref)))
}

fn blank_node_ref(i: Span) -> IntermediateResult<Ref> {
    let (i, bn) = blank_node(i)?;
    Ok((i, Ref::bnode_unchecked(bn)))
}

/// `[64]   	tripleExprLabel	   ::=   	iri | blankNode`
fn triple_expr_label(i: Span) -> IntermediateResult<Ref> {
    let (i, iri_ref) = iri(i)?; // alt((iri, blank_node))(i)?;
    let iri_s: IriS = iri_ref.into();
    Ok((i, Ref::from(iri_s)))
}

/// `[67]   	<CODE>	   ::=   	"{" ([^%\\] | "\\" [%\\] | UCHAR)* "%" "}"`
fn code(i: Span) -> IntermediateResult<Option<String>> {
    let (i, str) = delimited(char('{'), code_str, char('}'))(i)?;
    Ok((i, Some(str.to_string())))
}

fn code_str(i: Span) -> IntermediateResult<&str> {
    // Pending
    fail(i)
}
/// `[69]   	<RDF_TYPE>	   ::=   	"a"`
fn rdf_type(i: Span) -> IntermediateResult<IriRef> {
    let (i, _) = tag_no_case("a")(i)?;
    Ok((i, IriS::rdf_type().into()))
}

/// `[70]   	<ATPNAME_NS>	   ::=   	"@" PNAME_NS`
fn at_pname_ns(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (_, _, pname)) = tuple((char('@'), tws0, pname_ns))(i)?;
    todo!()
    // Ok((i, ShapeExpr::shape_ref())
}

/// `[71]   	<ATPNAME_LN>	   ::=   	"@" PNAME_LN`
fn at_pname_ln(i: Span) -> IntermediateResult<ShapeExpr> {
    let (i, (_, _, pname_ln)) = tuple((char('@'), tws0, pname_ln))(i)?;
    Ok((i, ShapeExpr::iri_ref(pname_ln)))
}

/// `[136s]   	iri	   ::=   	IRIREF | prefixedName`
fn iri(i: Span) -> IntermediateResult<IriRef> {
    alt((iri_ref_s, prefixed_name()))(i)
}

fn iri_ref_s(i: Span) -> IntermediateResult<IriRef> {
    let (i, iri) = iri_ref(i)?;
    Ok((i, iri.into()))
}

/// `[137s]   	prefixedName	   ::=   	PNAME_LN | PNAME_NS`
fn prefixed_name<'a>() -> impl FnMut(Span<'a>) -> IntermediateResult<'a, IriRef> {
   traced("prefixed_name", 
   map_error(move |i| {
     let (i, iri_ref) = alt((pname_ln, pname_ns_iri_ref))(i)?;
     Ok((i, iri_ref))
   }, || ShExParseError::ExpectedPrefixedName
  ))
}

fn pname_ns_iri_ref(i: Span) -> IntermediateResult<IriRef> {
    let (i, pname_ns) = pname_ns(i)?;
    Ok((i, IriRef::prefixed(pname_ns.fragment(), "")))
}

/// `[138s]   	blankNode	   ::=   	BLANK_NODE_LABEL`
fn blank_node(i: Span) -> IntermediateResult<&str> {
    blank_node_label(i)
}

/// `[142s]   	<BLANK_NODE_LABEL>	   ::=   	"_:" (PN_CHARS_U | [0-9]) ((PN_CHARS | ".")* PN_CHARS)?`
fn blank_node_label(i: Span) -> IntermediateResult<&str> {
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

fn blank_node_label2(src: Span) -> IntermediateResult<()> {
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

fn blank_node_label3(i: Span) -> IntermediateResult<Span> {
    take_while(is_pn_chars_or_dot)(i)
}

/// `[19t]   	<INTEGER>	   ::=   	[+-]? [0-9]+`
fn integer(i: Span) -> IntermediateResult<isize> {
    let (i, (maybe_sign, digits)) = tuple((opt(one_of("+-")), digits))(i)?;
    let n = match maybe_sign {
        None => digits,
        Some('+') => digits,
        Some('-') => -digits,
        _ => panic!("Internal parser error, Strange maybe_sign: {maybe_sign:?}"),
    };
    Ok((i, n))
}

fn digits(i: Span) -> IntermediateResult<isize> {
    map_res(digit1, |number: Span| number.parse::<isize>())(i)
}

impl FromExternalError<Span<'_>, ParseIntError> for LocatedParseError {
    fn from_external_error(input: Span, _kind: ErrorKind, e: ParseIntError) -> Self {
        ShExParseError::ParseIntError{ str: input.fragment().to_string(), err: e }.at(input)
    }
}

/// `[141s]   	<PNAME_LN>	   ::=   	PNAME_NS PN_LOCAL`
fn pname_ln(i: Span) -> IntermediateResult<IriRef> {
    // This code is different here: https://github.com/vandenoever/rome/blob/047cf54def2aaac75ac4b9adbef08a9d010689bd/src/io/turtle/grammar.rs#L293
    let (i, (prefix, local)) = tuple((pname_ns, pn_local))(i)?;
    Ok((i, IriRef::prefixed(prefix.fragment(), local)))
}

/// `[77]   	<PN_LOCAL>	   ::=   	(PN_CHARS_U | ":" | [0-9] | PLX) (PN_CHARS | "." | ":" | PLX)`
fn pn_local(i: Span) -> IntermediateResult<&str> {
    let (i, cs) = recognize(tuple((alt((one_if(is_pn_local_start), plx)), pn_local2)))(i)?;
    Ok((i, cs.fragment()))
}

fn is_pn_local_start(c: char) -> bool {
    c == ':' || is_digit(c) || is_pn_chars_u(c)
}

fn pn_local2(src: Span) -> IntermediateResult<()> {
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

fn pn_local3(i: Span) -> IntermediateResult<Span> {
    recognize(many0(alt((pn_chars_colon, plx, char_dot))))(i)
}

fn pn_chars_colon(i: Span) -> IntermediateResult<Span> {
    take_while1(is_pn_chars_colon)(i)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

fn plx(i: Span) -> IntermediateResult<Span> {
    alt((percent, pn_local_esc))(i)
}

/// ShEx rule
/// `[173s]   	<PN_LOCAL_ESC>	   ::=   	"\\" ( "_" | "~" | "." | "-" | "!" | "$" | "&" | "'" |
///                "(" | ")" | "*" | "+" | "," | ";" | "=" | "/" | "?" | "#" | "@" | "%" )``
fn pn_local_esc(i: Span) -> IntermediateResult<Span> {
    recognize(tuple((
        char('\\'),
        one_if(|c| "_~.-!$&'()*+,;=/?#@%".contains(c)),
    )))(i)
}

fn percent(i: Span) -> IntermediateResult<Span> {
    recognize(tuple((char('%'), one_if(is_hex), one_if(is_hex))))(i)
}

fn is_hex(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

/// `[18t]   	<IRIREF>	   ::=   	"<" ([^#0000- <>\"{}|^`\\] | UCHAR)* ">"`
fn iri_ref(i: Span) -> IntermediateResult<IriS> {
    let (i, str) = delimited(char('<'), take_while(is_iri_ref), char('>'))(i)?;
    Ok((i, IriS::new_unchecked(str.fragment())))
}

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

/// [140s] `<PNAME_NS>	   ::=   	PN_PREFIX? ":"`
fn pname_ns(i: Span) -> IntermediateResult<Span> {
    let (i, (maybe_pn_prefix, _)) = tuple((opt(pn_prefix), char(':')))(i)?;
    Ok((i, maybe_pn_prefix.unwrap_or(Span::from(""))))
}

/// [168s] `<PN_PREFIX>	::= PN_CHARS_BASE ( (PN_CHARS | ".")* PN_CHARS )?`
fn pn_prefix(i: Span) -> IntermediateResult<Span> {
    /*let (i, (pn_chars_base, maybe_rest)) = tuple((pn_chars_base, opt(rest_pn_prefix)))(i)?;
    let mut s: String = pn_chars_base.to_string();
    Ok((i, s.as_str()))*/
    recognize(tuple((
        satisfy(is_pn_chars_base),
        take_while(is_pn_chars),
        rest_pn_chars, // fold_many0(tuple((char('.'), take_while1(is_pn_chars))), || (), |_, _| ()),
    )))(i)
}

fn rest_pn_chars(i: Span) -> IntermediateResult<Vec<Span>> {
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

fn pn_chars_base(i: Span) -> IntermediateResult<char> {
    satisfy(is_pn_chars_base)(i)
}

/// From [168s] rest_pn_prefix = (PN_CHARS | ".")* PN_CHARS )
fn rest_pn_prefix(i: Span) -> IntermediateResult<&str> {
    let (i, (vs, cs)) = tuple((many0(alt((pn_chars, char_dot))), pn_chars))(i)?;
    // TODO...collect vs
    Ok((i, cs.fragment()))
}

fn char_dot(i: Span) -> IntermediateResult<Span> {
    recognize(char('.'))(i)
}

fn pn_chars(i: Span) -> IntermediateResult<Span> {
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
) -> impl Fn(Span<'a>) -> IntermediateResult<Span<'a>> {
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

fn symbol<'a>(value: &'a str) -> impl FnMut(Span<'a>) -> IntermediateResult<'a, ()> {
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
) -> IntermediateResult<'a, O2>
where
    F: FnMut(Span<'a>) -> IntermediateResult<'a, O>,
    G: FnMut(Span<'a>) -> IntermediateResult<'a, ()>,
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
    }

    #[test]
    fn test_empty_shex_statement() {
        assert_eq!(shex_statement(""), Ok(((""), Vec::new())))
    }

    #[test]
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
