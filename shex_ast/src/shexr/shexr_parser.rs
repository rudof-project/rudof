use super::shexr_error::ShExRError;
use super::*;
use crate::{
    BNode, NodeConstraint, NodeKind, ObjectValue, Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel, ValueSetValue,
};
use iri_s::IriS;
use iri_s::iri;
use prefixmap::IriRef;
use rdf::rdf_core::parser::rdf_node_parser::ParserExt;
use rdf::{
    rdf_core::{
        FocusRDF, RDFError, Rdf,
        parser::{
            RDFParse,
            rdf_node_parser::{
                RDFNodeParse,
                constructors::{
                    HasTypeParser, IriParser, IsIriParser, SetFocusParser,
                    SingleBoolPropertyParser, SingleInstanceParser, SingleValuePropertyParser,
                    SuccessParser, TermParser,
                },
            },
        },
        term::Object,
    },
    rdf_parser,
};

pub struct ShExRParser<RDF>
where
    RDF: FocusRDF,
{
    rdf_parser: RDFParse<RDF>,
}

impl<RDF> ShExRParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShExRParser<RDF> {
        ShExRParser {
            rdf_parser: RDFParse::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<Schema, ShExRError> {
        let schema = Self::schema_parser()
            .parse_focused(self.rdf_parser.rdf_mut())
            .map_err(|e| ShExRError::RDFParseError { err: e })?;
        let prefixmap = self.rdf_parser.prefixmap();
        Ok(schema.with_prefixmap(prefixmap))
    }

    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        let iri = ShExRVocab::sx_schema();
        SingleInstanceParser::new(iri).then(|node: <RDF as Rdf>::Subject| {
            let term: <RDF as Rdf>::Term = node.clone().into();
            SetFocusParser::new(term).then(|_| Self::schema())
        })
    }

    fn schema() -> impl RDFNodeParse<RDF, Output = Schema> {
        SingleValuePropertyParser::new(sx_shapes()).then(move |node| {
            SetFocusParser::new(node)
                .and(Self::shape_decl().list())
                .map(|(_, vs)| Schema::new(&iri!("http://default/")).with_shapes(Some(vs)))
        })
    }

    fn term_to_shape_label(term: &RDF::Term) -> Result<ShapeExprLabel, ShExRError> {
        let object = term
            .clone()
            .try_into()
            .map_err(|_| ShExRError::TermToRDFNodeFailed { term: term.to_string() })?;
        match object {
            Object::Iri(iri) => Ok(ShapeExprLabel::iri(iri)),
            Object::BlankNode(bnode) => Ok(ShapeExprLabel::bnode(BNode::new(bnode.as_str()))),
            Object::Literal(lit) => Err(ShExRError::ShapeExprLabelLiteral { term: lit.to_string() }),
            Object::Triple { .. } => todo!(),
        }
    }

    fn shape_decl() -> impl RDFNodeParse<RDF, Output = ShapeDecl> {
        (TermParser::new().flat_map(move |ref t| {
            let label = Self::term_to_shape_label(t).map_err(|e| RDFError::DefaultError {
                msg: format!("Expected term to be a label: {t}: {e}"),
            })?;
            Ok(label)
        }))
        .and(Self::parse_shape_expr())
        .map(|(label, se)| ShapeDecl::new(label, se, false))
    }

    fn parse_shape_expr() -> impl RDFNodeParse<RDF, Output = ShapeExpr> {
        SingleValuePropertyParser::new(sx_shape_expr())
            .then(|node| SetFocusParser::new(node).then(|_| shape_expr()))
    }
}

fn shape_expr_<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr>
where
    RDF: FocusRDF,
{
    shape_and().or(shape_or()).or(shape()).or(node_constraint())
}

fn shape<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr>
where
    RDF: FocusRDF,
{
    HasTypeParser::new(sx_shape()).with({
        closed().then(|maybe_closed| {
            let extra = None; // TODO
            let expression = None; // TODO
            SuccessParser::new(ShapeExpr::shape(Shape::new(
                maybe_closed,
                extra,
                expression,
            )))
        })
    })
}

fn closed<RDF>() -> impl RDFNodeParse<RDF, Output = Option<bool>>
where
    RDF: FocusRDF,
{
    SingleBoolPropertyParser::new(sx_closed()).optional()
}

rdf_parser! {
    pub fn shape_and[RDF]()(RDF) -> ShapeExpr where [] {
        HasTypeParser::new(sx_shape_and()).with(
        SingleValuePropertyParser::new(sx_shape_exprs()).then(|node| {
            SetFocusParser::new(node).and(shape_expr().list()).map(|(_,vs)| { ShapeExpr::and(vs) })
           }))
    }
}

rdf_parser! {
    pub fn shape_or[RDF]()(RDF) -> ShapeExpr where [] {
        HasTypeParser::new(sx_shape_or()).with(
        SingleValuePropertyParser::new(sx_shape_exprs().clone()).then(|node| {
            SetFocusParser::new(node).and(shape_expr().list()).map(|(_,vs)| { ShapeExpr::or(vs) })
        }))
    }
}

rdf_parser! {
    pub fn shape_expr[RDF]()(RDF) -> ShapeExpr where [] {
       shape_expr_()
    }
}

#[inline]
fn sx_shapes() -> IriS {
    IriS::new_unchecked(SX_SHAPES)
}

fn sx_shape_expr() -> IriS {
    IriS::new_unchecked(SX_SHAPE_EXPR)
}

fn sx_closed() -> IriS {
    IriS::new_unchecked(SX_CLOSED)
}

#[inline]
fn sx_values() -> IriS {
    IriS::new_unchecked(SX_VALUES)
}

#[inline]
fn sx_shape() -> IriS {
    IriS::new_unchecked(SX_SHAPE)
}

#[inline]
fn sx_shape_exprs() -> IriS {
    IriS::new_unchecked(SX_SHAPE_EXPRS)
}

#[inline]
fn sx_node_kind() -> IriS {
    IriS::new_unchecked(SX_NODEKIND)
}

fn sx_shape_and() -> IriS {
    IriS::new_unchecked(SX_SHAPE_AND)
}

fn sx_shape_or() -> IriS {
    IriS::new_unchecked(SX_SHAPE_OR)
}

fn node_constraint<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr>
where
    RDF: FocusRDF + 'static,
{
    parse_nodekind().then(|maybe_nodekind| {
        parse_value_set().then(move |maybe_valueset| {
            let mut nc = NodeConstraint::new();
            if let Some(nk) = &maybe_nodekind {
                nc = nc.with_node_kind(nk.clone());
            }
            if let Some(vs) = &maybe_valueset {
                nc = nc.with_values(vs.clone())
            }
            SuccessParser::new(ShapeExpr::node_constraint(nc))
        })
    })
}

fn parse_nodekind<RDF>() -> impl RDFNodeParse<RDF, Output = Option<NodeKind>>
where
    RDF: FocusRDF,
{
    SingleValuePropertyParser::new(sx_node_kind())
        .then(|node| SetFocusParser::new(node).and(nodekind()).map(|(_, vs)| vs))
        .optional()
}

fn nodekind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind>
where
    RDF: FocusRDF,
{
    iri_kind().or(literal_kind()).or(bnode_kind()).or(nonliteral_kind())
}

fn iri_kind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind>
where
    RDF: FocusRDF,
{
    IsIriParser::new(ShExRVocab::sx_iri()).map(|_| NodeKind::Iri)
}

fn literal_kind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind>
where
    RDF: FocusRDF,
{
    IsIriParser::new(ShExRVocab::sx_literal()).map(|_| NodeKind::Literal)
}

fn bnode_kind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind>
where
    RDF: FocusRDF,
{
    IsIriParser::new(ShExRVocab::sx_bnode()).map(|_| NodeKind::BNode)
}

fn nonliteral_kind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind>
where
    RDF: FocusRDF,
{
    IsIriParser::new(ShExRVocab::sx_nonliteral()).map(|_| NodeKind::NonLiteral)
}

fn parse_value_set<RDF>() -> impl RDFNodeParse<RDF, Output = Option<Vec<ValueSetValue>>>
where
    RDF: FocusRDF,
{
    SingleValuePropertyParser::new(sx_values())
        .then(|node| {
            SetFocusParser::new(node)
                .and(parse_value().list())
                .map(|(_, vs)| vs)
        })
        .optional()
}

fn parse_value<RDF>() -> impl RDFNodeParse<RDF, Output = ValueSetValue>
where
    RDF: FocusRDF,
{
    //firstOf(objectValue, )
    object_value().map(ValueSetValue::ObjectValue)
}

fn object_value<RDF>() -> impl RDFNodeParse<RDF, Output = ObjectValue>
where
    RDF: FocusRDF,
{
    IriParser::new().map(|iri: RDF::IRI| ObjectValue::IriRef(IriRef::Iri(iri.into())))
}
