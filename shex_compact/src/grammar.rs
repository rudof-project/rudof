use iri_s::IriS;
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while, take_while1},
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    error::{ErrorKind, ParseError},
    error_position,
    multi::{fold_many0, many0},
    sequence::{delimited, tuple},
    Err, IResult, InputTake, Needed,
};
use shex_ast::{ShapeExpr, ShapeLabel};

use crate::ShExStatement;

fn not_eol(c: char) -> bool {
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
}

/// whitespace that may contain comments
pub fn tws(i: &str) -> IResult<&str, ()> {
    fold_many0(
        alt((map(one_of(" \t\n\r"), |_| ()), map(comment, |_| ()))),
        || (),
        |_, _| (),
    )(i)
}

/// `[1] shexDoc	   ::=   	directive* ((notStartAction | startActions) statement*)?`
pub fn shex_statement(i: &str) -> IResult<&str, Vec<ShExStatement>> {
    let (i, (ds, sts)) = tuple((directives, statements))(i)?;
    let mut result = Vec::new();
    result.extend(ds);
    result.extend(sts);
    Ok((i, result))
}

pub fn directives(i: &str) -> IResult<&str, Vec<ShExStatement>> {
    many0(directive)(i)
}

pub fn statements(i: &str) -> IResult<&str, Vec<ShExStatement>> {
    many0(statement)(i)
}

/// [2] `directive	   ::=   	baseDecl | prefixDecl | importDecl`
pub fn directive(i: &str) -> IResult<&str, ShExStatement> {
    alt((
        // base_decl,
        prefix_decl,
        // import_decl
    ))(i)
}

/// [4] `prefixDecl	   ::=   	"PREFIX" PNAME_NS IRIREF`
fn prefix_decl(i: &str) -> IResult<&str, ShExStatement> {
    let (i, (_, _, pname_ns, _, iri_ref)) =
        tuple((tag_no_case("PREFIX"), tws, pname_ns, tws, iri_ref))(i)?;
    Ok((
        i,
        ShExStatement::PrefixDecl {
            alias: pname_ns,
            iri: iri_ref,
        },
    ))
}

/// `[5]   	notStartAction	   ::=   	start | shapeExprDecl`
fn not_start_action(i: &str) -> IResult<&str, ShExStatement> {
    alt((start, shape_expr_decl))(i)
}

/// `[6]   	start	   ::=   	"start" '=' inlineShapeExpression`
fn start(i: &str) -> IResult<&str, ShExStatement> {
    let (i, (_, _, _, _, se)) = tuple((
        tag_no_case("START"),
        tws,
        char('='),
        tws,
        inline_shape_expression,
    ))(i)?;
    Ok((i, ShExStatement::StartDecl { shape_expr: se }))
}

/// `[8]   	statement	   ::=   	directive | notStartAction`
fn statement(i: &str) -> IResult<&str, ShExStatement> {
    alt((directive, not_start_action))(i)
}

/// `[9]   	shapeExprDecl	   ::=   	shapeExprLabel (shapeExpression | "EXTERNAL")`
fn shape_expr_decl(i: &str) -> IResult<&str, ShExStatement> {
    let (i, (shape_label, _, shape_expr)) =
        tuple((shape_expr_label, tws, shape_expr_or_external))(i)?;
    Ok((
        i,
        ShExStatement::ShapeDecl {
            shape_label,
            shape_expr,
        },
    ))
}

fn shape_expr_or_external(i: &str) -> IResult<&str, ShapeExpr> {
    alt((shape_expression, external))(i)
}

fn external(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, _) = tag_no_case("EXTERNAL")(i)?;
    Ok((i, ShapeExpr::external()))
}

/// `[10]   	shapeExpression	   ::=   	shapeOr`
fn shape_expression(i: &str) -> IResult<&str, ShapeExpr> {
    shape_or(i)
}

/// `[11]   	inlineShapeExpression	   ::=   	inlineShapeOr`
fn inline_shape_expression(i: &str) -> IResult<&str, ShapeExpr> {
    todo!()
}

/// `[12]   	shapeOr	   ::=   	shapeAnd ("OR" shapeAnd)*`
fn shape_or(i: &str) -> IResult<&str, ShapeExpr> {
    todo!()
}

/// `[63]   	shapeExprLabel	   ::=   	iri | blankNode`
fn shape_expr_label(i: &str) -> IResult<&str, ShapeLabel> {
    let (i, iri_s) = iri(i)?; // alt((iri, blank_node))(i)?;
    Ok((i, ShapeLabel::Iri(iri_s)))
}

/// `[136s]   	iri	   ::=   	IRIREF | prefixedName`
fn iri(i: &str) -> IResult<&str, IriS> {
    alt((iri_ref, prefixed_name))(i)
}

/// `[137s]   	prefixedName	   ::=   	PNAME_LN | PNAME_NS`
fn prefixed_name(i: &str) -> IResult<&str, IriS> {
    let (i, s) = alt((pname_ln, pname_ns))(i)?;
    todo!()
}

/// `[138s]   	blankNode	   ::=   	BLANK_NODE_LABEL`
fn blank_node(i: &str) -> IResult<&str, &str> {
    todo!()
}

/// `[141s]   	<PNAME_LN>	   ::=   	PNAME_NS PN_LOCAL`
fn pname_ln(i: &str) -> IResult<&str, &str> {
    todo!()
}

/// `[18t]   	<IRIREF>	   ::=   	"<" ([^#0000- <>\"{}|^`\\] | UCHAR)* ">"`
fn iri_ref(i: &str) -> IResult<&str, IriS> {
    let (i, str) = delimited(char('<'), take_while(is_iri_ref), char('>'))(i)?;
    Ok((i, IriS::new_unchecked(str)))
}

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

/// [140s] `<PNAME_NS>	   ::=   	PN_PREFIX? ":"`
fn pname_ns(i: &str) -> IResult<&str, &str> {
    let (i, pn_prefix) = opt(pn_prefix)(i)?;
    let (i, _) = char(':')(i)?;
    Ok((i, pn_prefix.unwrap_or("")))
}

/// [168s] `<PN_PREFIX>	::= PN_CHARS_BASE ( (PN_CHARS | ".")* PN_CHARS )?`
fn pn_prefix(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        one_if(is_pn_chars_base),
        take_while(is_pn_chars),
        fold_many0(
            tuple((char('.'), take_while1(is_pn_chars))),
            || (),
            |_, _| (),
        ),
    )))(i)
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
fn one_if<'a, E: ParseError<&'a str>, F: Fn(char) -> bool>(
    f: F,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    move |i: &str| {
        if let Some(c) = i.chars().next() {
            if f(c) {
                Ok(i.take_split(1))
            } else {
                Err(Err::Error(error_position!(i, ErrorKind::OneOf)))
            }
        } else {
            Err(Err::Incomplete(Needed::new(1)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ShExStatement;

    #[test]
    fn test_comment() {
        assert_eq!(comment("#\r\na"), Ok((("\na"), (""))));
        assert_eq!(comment("#\n\ra"), Ok((("\ra"), (""))));
        // assert_eq!(comment(""), Err(Err::Error(("".as_ref(), ErrorKind::Char))));
        assert_eq!(comment("#"), Ok(("", "")));
        assert_eq!(comment("#abc"), Ok(("", "abc")));
        assert_eq!(comment("#\n\n"), Ok(("\n", "")));
    }

    #[test]
    fn test_prefix_id() {
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
}
