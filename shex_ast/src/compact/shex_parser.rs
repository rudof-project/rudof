use crate::ParseError;
use crate::Span;
use crate::ast::Schema;
use crate::compact::grammar_structs::ShExStatement;
use crate::shex_statement;
use crate::tws0;
use crate::{NodeConstraint, NumericFacet, ShapeExpr, StringFacet, TripleExpr, TripleExprLabel, XsFacet};
use nom::Err;
use prefixmap::DerefIri;
use rudof_iri::IriS;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use tracing::debug;

// This code is inspired from:
// https://github.com/vandenoever/rome/blob/master/src/io/turtle/parser.rs

type Result<A> = std::result::Result<A, ParseError>;

/// Resolves the IRI of a `BASE` or `PREFIX` declaration against the current
/// base: with `BASE <http://ex.example/a/>`, `BASE <../>` is `http://ex.example/`.
/// An IRI that can't be resolved is kept as it is.
fn resolve(base: Option<&IriS>, iri: IriS) -> IriS {
    match base {
        Some(base) => base.resolve_str(iri.as_str()).unwrap_or(iri),
        None => iri,
    }
}

pub struct ShExParser<'a> {
    shex_statement_iterator: StatementIterator<'a>,
    // state: ParserState,
    // done: bool,
}

impl ShExParser<'_> {
    /// Parse a ShEx schema that uses [ShEx compact syntax](https://shex.io/shex-semantics/index.html#shexc)
    ///
    /// `base` is an optional IRI that acts as the base for relative IRIs
    pub fn parse(src: &str, base: Option<IriS>, source_iri: &IriS) -> Result<Schema> {
        let mut schema = Schema::new(source_iri).with_base(base);
        let mut parser = ShExParser {
            shex_statement_iterator: StatementIterator::new(Span::new(src))?,
        };
        // let mut shapes_counter = 0;
        for s in parser.shex_statement_iterator.by_ref() {
            match s? {
                ShExStatement::BaseDecl { iri } => {
                    let iri = resolve(schema.base().as_ref(), iri);
                    schema = schema.with_base(Some(iri));
                },
                ShExStatement::PrefixDecl { alias, iri } => {
                    let iri = resolve(schema.base().as_ref(), iri);
                    schema.add_prefix(alias, &iri);
                },
                ShExStatement::StartDecl { shape_expr } => {
                    let shape_expr = shape_expr.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref())?;
                    schema = schema.with_start(Some(shape_expr))
                },
                ShExStatement::ImportDecl { iri } => {
                    schema = schema.with_import(iri);
                },
                ShExStatement::ShapeDecl {
                    is_abstract,
                    shape_label,
                    shape_expr,
                } => {
                    let shape_label = shape_label.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref())?;
                    let shape_expr = shape_expr.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref())?;
                    schema.add_shape(shape_label, shape_expr, is_abstract);
                },
                ShExStatement::StartActions { actions } => {
                    let actions = actions
                        .into_iter()
                        .map(|action| action.deref_iri(schema.base().as_ref(), schema.prefixmap().as_ref()))
                        .collect::<std::result::Result<Vec<_>, _>>()?;
                    schema = schema.with_start_actions(Some(actions));
                },
            }
        }
        check_schema(&schema)?;
        Ok(schema)
    }

    pub fn parse_buf(path: &Path, base: Option<IriS>) -> Result<Schema> {
        let source_iri = path.try_into().map_err(|e| ParseError::Custom {
            msg: format!("Cannot convert path to IRI: {e}"),
        })?;
        let data = fs::read_to_string(path)?;
        let schema = ShExParser::parse(&data, base, &source_iri)?;
        Ok(schema)
    }

    pub fn from_reader<R: io::Read>(mut reader: R, base: Option<IriS>, source_iri: &IriS) -> Result<Schema> {
        let mut v = Vec::new();
        reader.read_to_end(&mut v)?;
        let s = String::from_utf8(v).map_err(|e| ParseError::Utf8Error { error: format!("{e}") })?;
        Self::parse(s.as_str(), base, source_iri)
    }
}

struct StatementIterator<'a> {
    src: Span<'a>,
    done: bool,
}

impl StatementIterator<'_> {
    pub fn new(src: Span) -> Result<StatementIterator> {
        match tws0(src) {
            Ok((left, _)) => Ok(StatementIterator { src: left, done: false }),
            Err(Err::Incomplete(_)) => Ok(StatementIterator { src, done: false }),
            Err(e) => Err(ParseError::Custom {
                msg: format!("cannot start parsing. Error: {e}"),
            }),
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<ShExStatement<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        if self.src.is_empty() {
            self.done = true;
            return None;
        }
        match shex_statement()(self.src) {
            Ok((left, s)) => {
                r = Some(Ok(s));
                self.src = left;
            },
            Err(Err::Incomplete(needed)) => {
                debug!("Incomplete! shex_statement. Needed: {needed:?}");
                self.done = true;
                r = None;
            },
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(ParseError::NomError { err: Box::new(e) }));
                self.done = true;
            },
        }

        // Skip extra whitespace
        match tws0(self.src) {
            Ok((left, _)) => {
                self.src = left;
            },
            Err(Err::Incomplete(needed)) => {
                debug!("Incomplete on tws: needed {needed:?}");
                self.done = true;
            },
            Err(e) => {
                r = Some(Err(ParseError::Custom {
                    msg: format!("error parsing whitespace. Error: {e}"),
                }));
                self.done = true;
            },
        }
        r
    }
}

/// Checks the rules of ShExC that the grammar doesn't express: the labels of
/// shapes and of triple expressions are unique (a label can't be used for a
/// shape and for a triple expression), a node constraint has each facet only
/// once, and numeric facets are only used with numeric datatypes.
fn check_schema(schema: &Schema) -> Result<()> {
    let mut shape_labels = HashSet::new();
    let mut triple_expr_labels = HashSet::new();
    let shapes = schema.shapes().unwrap_or_default();
    for shape in &shapes {
        let label = shape.id.to_string();
        if !shape_labels.insert(label.clone()) {
            return Err(ParseError::DuplicateShapeLabel { label });
        }
    }
    let mut result = Ok(());
    let mut check = |item: Item| {
        if result.is_err() {
            return;
        }
        result = match item {
            Item::TripleExprLabel(label) => {
                let label = label.to_string();
                if shape_labels.contains(&label) {
                    Err(ParseError::ShapeAndTripleExprLabel { label })
                } else if !triple_expr_labels.insert(label.clone()) {
                    Err(ParseError::DuplicateTripleExprLabel { label })
                } else {
                    Ok(())
                }
            },
            Item::NodeConstraint(nc) => check_node_constraint(nc),
        }
    };
    if let Some(start) = schema.start() {
        visit_shape_expr(&start, &mut check);
    }
    for shape in &shapes {
        visit_shape_expr(&shape.shape_expr, &mut check);
    }
    result
}

const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
const NUMERIC_DATATYPES: [&str; 16] = [
    "integer",
    "decimal",
    "float",
    "double",
    "long",
    "int",
    "short",
    "byte",
    "nonNegativeInteger",
    "nonPositiveInteger",
    "negativeInteger",
    "positiveInteger",
    "unsignedLong",
    "unsignedInt",
    "unsignedShort",
    "unsignedByte",
];

fn check_node_constraint(nc: &NodeConstraint) -> Result<()> {
    let facets = nc.xs_facet().unwrap_or_default();
    let mut kinds = HashSet::new();
    for facet in &facets {
        let name = facet_name(facet);
        if !kinds.insert(name) {
            return Err(ParseError::DuplicateFacet {
                facet: name.to_string(),
            });
        }
    }
    if let Some(datatype) = nc.datatype() {
        let datatype = datatype.to_string();
        let numeric = datatype
            .strip_prefix(XSD)
            .is_some_and(|local| NUMERIC_DATATYPES.contains(&local));
        if !numeric && let Some(facet) = facets.iter().find(|f| is_numeric_range(f)) {
            return Err(ParseError::NumericFacetWithoutNumericDatatype {
                facet: facet_name(facet).to_string(),
                datatype,
            });
        }
    }
    Ok(())
}

fn facet_name(facet: &XsFacet) -> &'static str {
    match facet {
        XsFacet::StringFacet(StringFacet::Length(_)) => "LENGTH",
        XsFacet::StringFacet(StringFacet::MinLength(_)) => "MINLENGTH",
        XsFacet::StringFacet(StringFacet::MaxLength(_)) => "MAXLENGTH",
        XsFacet::StringFacet(StringFacet::Pattern(_)) => "PATTERN",
        XsFacet::NumericFacet(NumericFacet::MinInclusive(_)) => "MININCLUSIVE",
        XsFacet::NumericFacet(NumericFacet::MinExclusive(_)) => "MINEXCLUSIVE",
        XsFacet::NumericFacet(NumericFacet::MaxInclusive(_)) => "MAXINCLUSIVE",
        XsFacet::NumericFacet(NumericFacet::MaxExclusive(_)) => "MAXEXCLUSIVE",
        XsFacet::NumericFacet(NumericFacet::TotalDigits(_)) => "TOTALDIGITS",
        XsFacet::NumericFacet(NumericFacet::FractionDigits(_)) => "FRACTIONDIGITS",
    }
}

/// The range facets compare values as numbers. `TOTALDIGITS` and
/// `FRACTIONDIGITS` count the digits of the lexical form.
fn is_numeric_range(facet: &XsFacet) -> bool {
    matches!(
        facet,
        XsFacet::NumericFacet(
            NumericFacet::MinInclusive(_)
                | NumericFacet::MinExclusive(_)
                | NumericFacet::MaxInclusive(_)
                | NumericFacet::MaxExclusive(_)
        )
    )
}

enum Item<'a> {
    TripleExprLabel(&'a TripleExprLabel),
    NodeConstraint(&'a NodeConstraint),
}

fn visit_shape_expr(se: &ShapeExpr, f: &mut impl FnMut(Item)) {
    match se {
        ShapeExpr::ShapeOr { shape_exprs } | ShapeExpr::ShapeAnd { shape_exprs } => {
            for se in shape_exprs {
                visit_shape_expr(&se.se, f);
            }
        },
        ShapeExpr::ShapeNot { shape_expr } => visit_shape_expr(&shape_expr.se, f),
        ShapeExpr::NodeConstraint(nc) => f(Item::NodeConstraint(nc)),
        ShapeExpr::Shape(shape) => {
            if let Some(expression) = &shape.expression {
                visit_triple_expr(&expression.te, f);
            }
        },
        ShapeExpr::External | ShapeExpr::Ref(_) => {},
    }
}

fn visit_triple_expr(te: &TripleExpr, f: &mut impl FnMut(Item)) {
    match te {
        TripleExpr::EachOf { id, expressions, .. } | TripleExpr::OneOf { id, expressions, .. } => {
            if let Some(id) = id {
                f(Item::TripleExprLabel(id));
            }
            for expression in expressions {
                visit_triple_expr(&expression.te, f);
            }
        },
        TripleExpr::TripleConstraint { id, value_expr, .. } => {
            if let Some(id) = id {
                f(Item::TripleExprLabel(id));
            }
            if let Some(value_expr) = value_expr {
                visit_shape_expr(value_expr, f);
            }
        },
        TripleExpr::Ref(_) => {},
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Shape, ShapeExpr, ShapeExprLabel, XsFacet};
    use rudof_rdf::rdf_core::term::literal::NumericLiteral;

    use super::*;
    use rudof_iri::iri;

    #[test]
    fn test_prefix() {
        let str = r#"
 prefix e: <http://example.org/>
 e:S {}
 "#;
        let schema = ShExParser::parse(str, None, &iri!("http://default/")).unwrap();
        let mut expected = Schema::new(&iri!("http://default/"));
        expected.add_prefix("e", &IriS::new_unchecked("http://example.org/"));
        expected.add_shape(
            ShapeExprLabel::iri_unchecked("http://example.org/S"),
            ShapeExpr::Shape(Shape::new(None, None, None)),
            false,
        );
        assert_eq!(schema, expected)
    }

    #[test]
    fn test_node_constraint_annotations_and_sem_acts() {
        let str = r#"
 prefix e: <http://example.org/>
 e:NC [1 2] // e:a 7 %e:Test{ print(n) %}
 "#;
        let schema = ShExParser::parse(str, None, &iri!("http://default/")).unwrap();
        // The trailing code declaration belongs to the node constraint, not
        // to the schema's start actions.
        assert!(schema.start_actions().is_none());
        let nc = match schema.shapes().unwrap()[0].shape_expr.clone() {
            ShapeExpr::NodeConstraint(nc) => nc,
            other => panic!("expected a node constraint, found {other:?}"),
        };
        assert_eq!(nc.annotations().map(|anns| anns.len()), Some(1));
        assert_eq!(nc.sem_acts().map(|sas| sas.len()), Some(1));
    }

    fn node_constraint_facets(body: &str) -> Vec<XsFacet> {
        let str =
            format!("prefix : <http://example.org/>\nprefix xsd: <http://www.w3.org/2001/XMLSchema#>\n:S {body}\n");
        let schema = ShExParser::parse(&str, None, &iri!("http://default/"))
            .unwrap_or_else(|e| panic!("failed to parse `{body}`: {e}"));
        match schema.shapes().unwrap()[0].shape_expr.clone() {
            ShapeExpr::NodeConstraint(nc) => nc.xs_facet().unwrap_or_default(),
            other => panic!("expected a node constraint, found {other:?}"),
        }
    }

    // Regression test for https://github.com/rudof-project/rudof/issues/857
    #[test]
    fn test_chained_numeric_facets_case_insensitive() {
        let expected = vec![
            XsFacet::min_inclusive(NumericLiteral::integer(13)),
            XsFacet::max_inclusive(NumericLiteral::integer(20)),
        ];
        assert_eq!(
            node_constraint_facets("xsd:integer MININCLUSIVE 13 MAXINCLUSIVE 20"),
            expected
        );
        assert_eq!(
            node_constraint_facets("xsd:integer MinInclusive 13 MaxInclusive 20"),
            expected
        );
        assert_eq!(
            node_constraint_facets("xsd:integer minexclusive 1 maxexclusive 5"),
            vec![
                XsFacet::min_exclusive(NumericLiteral::integer(1)),
                XsFacet::max_exclusive(NumericLiteral::integer(5)),
            ]
        );
        assert_eq!(
            node_constraint_facets("xsd:decimal TotalDigits 3 FractionDigits 2"),
            vec![XsFacet::totaldigits(3), XsFacet::fractiondigits(2)]
        );
    }

    #[test]
    fn test_chained_string_facets() {
        let expected = vec![XsFacet::min_length(3), XsFacet::max_length(5)];
        assert_eq!(node_constraint_facets("xsd:string MINLENGTH 3 MAXLENGTH 5"), expected);
        assert_eq!(node_constraint_facets("xsd:string MinLength 3 MaxLength 5"), expected);
        assert_eq!(node_constraint_facets("LITERAL MINLENGTH 3 MAXLENGTH 5"), expected);
        assert_eq!(node_constraint_facets("IRI MINLENGTH 3 MAXLENGTH 5"), expected);
        assert_eq!(node_constraint_facets("MINLENGTH 3 MAXLENGTH 5"), expected);
        assert_eq!(
            node_constraint_facets("xsd:string /a/i LENGTH 2"),
            vec![XsFacet::pattern_flags("a", "i"), XsFacet::length(2)]
        );
    }

    #[test]
    fn test_shape_qualifiers_case_insensitive() {
        let str = r#"
 prefix : <http://example.org/>
 :S Closed Extra :p { :p . }
 "#;
        let schema = ShExParser::parse(str, None, &iri!("http://default/")).unwrap();
        let shape = match schema.shapes().unwrap()[0].shape_expr.clone() {
            ShapeExpr::Shape(shape) => shape,
            other => panic!("expected a shape, found {other:?}"),
        };
        assert_eq!(shape.closed, Some(true));
        assert_eq!(shape.extra.map(|e| e.len()), Some(1));
    }
}
