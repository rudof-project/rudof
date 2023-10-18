use iri_s::IriS;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1},
    character::complete::{char, one_of},
    combinator::{fail, map, opt, recognize, map_res},
    error::{ErrorKind, ParseError},
    error_position,
    multi::{fold_many0, many0, many1},
    sequence::{delimited, tuple},
    Err, IResult, InputLength, InputTake, Needed, Parser,
};
use shex_ast::{
    object_value::ObjectValue, Annotation, IriRef, NodeConstraint, Ref, SemAct, ShapeExpr,
    ShapeLabel, TripleExpr,
};

use crate::{Qualifier, ShExStatement, Cardinality};

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
    inline_shape_or(i)
}

/// `[12]   	shapeOr	   ::=   	shapeAnd ("OR" shapeAnd)*`
fn shape_or(i: &str) -> IResult<&str, ShapeExpr> {
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
fn inline_shape_or(i: &str) -> IResult<&str, ShapeExpr> {
    many1_sep(inline_shape_and, symbol("OR"), make_shape_or, i)
}

/// `[14]   	shapeAnd	   ::=   	shapeNot ("AND" shapeNot)*``
fn shape_and(i: &str) -> IResult<&str, ShapeExpr> {
    many1_sep(shape_not, symbol("AND"), make_shape_and, i)
}

fn make_shape_and(ses: Vec<ShapeExpr>) -> ShapeExpr {
    if ses.len() == 1 {
        ses[0].clone()
    } else {
        ShapeExpr::or(ses)
    }
}

/// `[15]   	inlineShapeAnd	   ::=   	inlineShapeNot ("AND" inlineShapeNot)*`
fn inline_shape_and(i: &str) -> IResult<&str, ShapeExpr> {
    many1_sep(inline_shape_not, symbol("AND"), make_shape_and, i)
}

/// `[16]   	shapeNot	   ::=   	"NOT"? shapeAtom`
fn shape_not(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, maybe) = opt(symbol("NOT"))(i)?;
    let (i, se) = shape_atom(i)?;
    match maybe {
        None => Ok((i, se)),
        Some(_) => Ok((i, ShapeExpr::not(se))),
    }
}

/// `[17]   	inlineShapeNot	   ::=   	"NOT"? inlineShapeAtom`
fn inline_shape_not(i: &str) -> IResult<&str, ShapeExpr> {
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
fn shape_atom(i: &str) -> IResult<&str, ShapeExpr> {
    alt((
        // Pending
        // non_lit_shape,
        // lit_node_constraint,
        shape_opt_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

fn shape_opt_non_lit(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, se) = shape_or_ref(i)?;
    let (i, maybe_non_lit) = opt(non_lit_node_constraint)(i)?;
    match maybe_non_lit {
        None => Ok((i, se)),
        Some(nl) => Ok((i, ShapeExpr::and(vec![se, ShapeExpr::node_constraint(nl)]))),
    }
}

/// `[20]   	inlineShapeAtom	   ::=   	   nonLitNodeConstraint inlineShapeOrRef?
/// `| litNodeConstraint`
/// `| inlineShapeOrRef nonLitNodeConstraint?`
/// `| '(' shapeExpression ')'`
/// `| '.'`
fn inline_shape_atom(i: &str) -> IResult<&str, ShapeExpr> {
    alt((
        // Pending
        // nonlit_inline_shape,
        // lit_node_constraint,
        // inline_shape_non_lit,
        paren_shape_expr,
        dot,
    ))(i)
}

fn paren_shape_expr(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (_, _, se, _, _)) = tuple((char('('), tws, shape_expression, tws, char(')')))(i)?;
    Ok((i, se))
}

fn dot(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (_, _)) = tuple((tws, char('.')))(i)?;
    Ok((i, ShapeExpr::any()))
}

/// `[21]   	shapeOrRef	   ::=   	   shapeDefinition | shapeRef`
fn shape_or_ref(i: &str) -> IResult<&str, ShapeExpr> {
    alt((shape_definition, shape_ref))(i)
}

/// `[23]   	shapeRef	   ::=   	   ATPNAME_LN | ATPNAME_NS | '@' shapeExprLabel`
fn shape_ref(i: &str) -> IResult<&str, ShapeExpr> {
    alt((at_pname_ln, at_pname_ns, at_shape_expr_label))(i)
}

fn at_shape_expr_label(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (_, label)) = tuple((char('@'), shape_expr_label))(i)?;
    Ok((i, ShapeExpr::shape_ref(label)))
}

/// `[25]   	nonLitNodeConstraint	   ::=   	   nonLiteralKind stringFacet*`
/// `| stringFacet+`
fn non_lit_node_constraint(i: &str) -> IResult<&str, NodeConstraint> {
    // Pending
    fail(i)
}

/// `[33]   	shapeDefinition	   ::=   	(extraPropertySet | "CLOSED")* '{' tripleExpression? '}' annotation* semanticActions`
fn shape_definition(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (qualifiers, _, _, maybe_triple_expr, _, _, annotations, sem_actions)) = tuple((
        qualifiers,
        char('{'),
        tws,
        opt(triple_expression),
        tws,
        char('}'),
        annotations,
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
            Qualifier::Extra(ps) => extra.append(&mut ps),
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
            closed,
            maybe_extra,
            maybe_triple_expr,
            annotations,
            sem_actions,
        ),
    ))
}

fn annotations(i: &str) -> IResult<&str, Vec<Annotation>> {
    many0(annotation)(i)
}

fn qualifiers(i: &str) -> IResult<&str, Vec<Qualifier>> {
    many0(qualifier)(i)
}

fn qualifier(i: &str) -> IResult<&str, Qualifier> {
    alt((closed, extra_property_set))(i)
}

fn closed(i: &str) -> IResult<&str, Qualifier> {
    let (i, _) = tag_no_case("CLOSED")(i)?;
    Ok((i, Qualifier::Closed))
}

/// `[35]   	extraPropertySet	   ::=   	"EXTRA" predicate+`
fn extra_property_set(i: &str) -> IResult<&str, Qualifier> {
    let (i, (_, ps)) = tuple((tag_no_case("EXTRA"), many1(predicate)))(i)?;
    Ok((i, Qualifier::Extra(ps)))
}

/// `[36]   	tripleExpression	   ::=   	oneOfTripleExpr`
fn triple_expression(i: &str) -> IResult<&str, TripleExpr> {
    // Pending
    triple_constraint(i)
}

/// `[45]   	tripleConstraint	   ::=   	senseFlags? predicate inlineShapeExpression cardinality? annotation* semanticActions`
fn triple_constraint(i: &str) -> IResult<&str, TripleExpr> {
    let (i, (predicate, se, maybe_card)) =
        tuple((predicate, inline_shape_expression, opt(cardinality)))(i)?;
    let min = maybe_card.and_then(|m| m.min());
    let max = maybe_card.and_then(|m| m.max());
    Ok((i, TripleExpr::triple_constraint(predicate, Some(se), min, max)))
}

/// `46]   	cardinality	   ::=   	'*' | '+' | '?' | REPEAT_RANGE`
fn cardinality(i: &str) -> IResult<&str, Cardinality> {
    alt((plus, star, optional, 
        // Pending
        // repeat_range
    ))(i)
}

fn plus(i: &str) -> IResult<&str, Cardinality> {
    let (i, _) = char('+')(i)?;
    Ok((i, Cardinality::plus()))
}

fn star(i: &str) -> IResult<&str, Cardinality> {
    let (i, _) = char('*')(i)?;
    Ok((i, Cardinality::star()))
}

fn optional(i: &str) -> IResult<&str, Cardinality> {
    let (i, _) = char('?')(i)?;
    Ok((i, Cardinality::optional()))
}


/// `[58]   	annotation	   ::=   	"//" predicate (iri | literal)`
fn annotation(i: &str) -> IResult<&str, Annotation> {
    let (i, (_, p, o)) = tuple((tag("//"), predicate, iri_or_literal))(i)?;
    Ok((i, Annotation::new(p.into(), o)))
}

fn iri_or_literal(i: &str) -> IResult<&str, ObjectValue> {
    // Pending literal
    let (i, iri) = iri(i)?;
    Ok((i, ObjectValue::IriRef(iri.into())))
}

/// `[59]   	semanticActions	   ::=   	codeDecl*`
fn semantic_actions(i: &str) -> IResult<&str, Option<Vec<SemAct>>> {
    let (i, sas) = many0(code_decl)(i)?;
    if sas.is_empty() {
        Ok((i, None))
    } else {
        Ok((i, Some(sas)))
    }
}

/// `[60]   	codeDecl	   ::=   	'%' iri (CODE | '%')`
fn code_decl(i: &str) -> IResult<&str, SemAct> {
    let (i, (_, iri, code)) = tuple((char('%'), iri, code_or_percent))(i)?;
    Ok((i, SemAct::new(IriRef::from(iri), code)))
}

fn code_or_percent(i: &str) -> IResult<&str, Option<String>> {
    let (i, maybe_code) = alt((code, percent_code))(i)?;
    Ok((i, maybe_code))
}

fn percent_code(i: &str) -> IResult<&str, Option<String>> {
    let (i, _) = char('%')(i)?;
    Ok((i, None))
}

/// `[61]   	predicate	   ::=   	iri | RDF_TYPE`
fn predicate(i: &str) -> IResult<&str, IriRef> {
    alt((iri, rdf_type))(i)
}

/// `[63]   	shapeExprLabel	   ::=   	iri | blankNode`
fn shape_expr_label(i: &str) -> IResult<&str, Ref> {
    let (i, iri_s) = iri(i)?; // alt((iri, blank_node))(i)?;
    Ok((i, Ref::from(iri_s)))
}

/// `[67]   	<CODE>	   ::=   	"{" ([^%\\] | "\\" [%\\] | UCHAR)* "%" "}"`
fn code(i: &str) -> IResult<&str, Option<String>> {
    let (i, str) = delimited(char('{'), code_str, char('}'))(i)?;
    Ok((i, Some(str.to_string())))
}

fn code_str(i: &str) -> IResult<&str, &str> {
    // Pending
    fail(i)
}
/// `[69]   	<RDF_TYPE>	   ::=   	"a"`
fn rdf_type(i: &str) -> IResult<&str, IriS> {
    let (i, _) = tag_no_case("a")(i)?;
    Ok((i, IriS::rdf_type()))
}

/// `[70]   	<ATPNAME_NS>	   ::=   	"@" PNAME_NS`
fn at_pname_ns(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (_, _, pname)) = tuple((char('@'), tws, pname_ns))(i)?;
    todo!()
    // Ok((i, ShapeExpr::shape_ref())
}

/// `[71]   	<ATPNAME_LN>	   ::=   	"@" PNAME_LN`
fn at_pname_ln(i: &str) -> IResult<&str, ShapeExpr> {
    let (i, (_, _, pname_ln)) = tuple((char('@'), tws, pname_ln))(i)?;
    todo!();
}

/// `[136s]   	iri	   ::=   	IRIREF | prefixedName`
fn iri(i: &str) -> IResult<&str, IriRef> {
    alt((iri_ref, prefixed_name))(i)
}

/// `[137s]   	prefixedName	   ::=   	PNAME_LN | PNAME_NS`
fn prefixed_name(i: &str) -> IResult<&str, IriRef> {
    let (i, s) = alt((pname_ln, pname_ns))(i)?;
    todo!()
}

/// `[138s]   	blankNode	   ::=   	BLANK_NODE_LABEL`
fn blank_node(i: &str) -> IResult<&str, &str> {
    todo!()
}

/// `[141s]   	<PNAME_LN>	   ::=   	PNAME_NS PN_LOCAL`
fn pname_ln(i: &str) -> IResult<&str, &str> {
    // This code is different here: https://github.com/vandenoever/rome/blob/047cf54def2aaac75ac4b9adbef08a9d010689bd/src/io/turtle/grammar.rs#L293
    let (i, (str_pname_ns, str_pn_local)) = tuple((pname_ns, pn_local))(i)?;
    // concat
    println!("pname_ln with pname_ns = {str_pname_ns} and pn_local = {str_pn_local}");
    Ok((i, str_pname_ns))
}

/// `[77]   	<PN_LOCAL>	   ::=   	(PN_CHARS_U | ":" | [0-9] | PLX) (PN_CHARS | "." | ":" | PLX)`
fn pn_local(i: &str) -> IResult<&str, &str> {
    recognize(tuple((alt((one_if(is_pn_local_start), plx)), pn_local2)))(i)
}

fn is_pn_local_start(c: char) -> bool {
    c == ':' || is_digit(c) || is_pn_chars_u(c)
}

fn pn_local2(src: &str) -> IResult<&str, ()> {
    match pn_local3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                Ok(((&src[m.len() - 1..]), ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

fn pn_local3(i: &str) -> IResult<&str, &str> {
    recognize(many0(alt((pn_chars_colon, plx, tag(".")))))(i)
}

fn pn_chars_colon(i: &str) -> IResult<&str, &str> {
    take_while1(is_pn_chars_colon)(i)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

fn plx(i: &str) -> IResult<&str, &str> {
    alt((percent, pn_local_esc))(i)
}

/// ShEx rule
/// `[173s]   	<PN_LOCAL_ESC>	   ::=   	"\\" ( "_" | "~" | "." | "-" | "!" | "$" | "&" | "'" |
///                "(" | ")" | "*" | "+" | "," | ";" | "=" | "/" | "?" | "#" | "@" | "%" )``
fn pn_local_esc(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('\\'),
        one_if(|c| "_~.-!$&'()*+,;=/?#@%".contains(c)),
    )))(i)
}

fn percent(i: &str) -> IResult<&str, &str> {
    recognize(tuple((char('%'), one_if(is_hex), one_if(is_hex))))(i)
}

fn is_hex(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
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

fn symbol<'a>(value: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |i: &str| {
        let (i, _) = tag_no_case(value)(i)?;
        Ok((i, ()))
    }
}

fn many1_sep<'a, O, O2, F, G, H>(
    mut parser_many: F,
    mut sep: G,
    maker: H,
    mut i: &'a str,
) -> IResult<&'a str, O2>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
    G: FnMut(&'a str) -> IResult<&'a str, ()>,
    H: Fn(Vec<O>) -> O2,
{
    let mut vs = Vec::new();

    // skip tws
    if let Ok((left, _)) = tws(i) {
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
        match sep(i) {
            Ok((left, _)) => {
                i = left;
            }
            _ => return Ok((i, maker(vs))),
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
            Ok((
                "",
                ShapeLabel::Iri(IriS::new_unchecked("http://example.org/S"))
            ))
        );
    }

    #[test]
    fn test_shape_expr_dot() {
        assert_eq!(shape_expression("."), Ok(("", ShapeExpr::any())));
    }
}
