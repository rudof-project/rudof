use super::shexr_error::ShExRError;
use super::*;
use crate::{
    BNode, NodeConstraint, NodeKind, Schema, Shape, ShapeDecl, ShapeExpr, ShapeExprLabel,
    ValueSetValue, XsFacet, ObjectValue,
};
use iri_s::IriS;
use prefixmap::IriRef;
use srdf::FocusRDF;
use srdf::{Object, RDFParser};
use srdf::srdf_parser::*;
use srdf::RDFParseError;
use srdf::rdf_parser;

type Result<A> = std::result::Result<A, ShExRError>;

pub struct ShExRParser<RDF>
where
    RDF: FocusRDF,
{
    rdf_parser: RDFParser<RDF>,
}

impl<RDF> ShExRParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShExRParser<RDF> {
        ShExRParser {
            rdf_parser: RDFParser::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<Schema> {
        let schema = Self::schema_parser().parse_impl(&mut self.rdf_parser.rdf).map_err(|e| {
            ShExRError::RDFParseError { err: e }
        })?;
        let prefixmap = self.rdf_parser.prefixmap();
        Ok(schema.with_prefixmap(prefixmap))

    }

    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        instance_of(&ShExRVocab::sx_schema()).then(|node| {
            set_focus_subject(node).then(|_| Self::schema())
        })
    }

    fn schema() -> impl RDFNodeParse<RDF, Output = Schema> {
        property_value(&sx_shapes()).then(|ref node| {
            set_focus(node).and(
                parse_rdf_list::<RDF, _>(Self::shape_decl())
            ).map(|(_,vs)| { Schema::new().with_shapes(Some(vs)) })
        })
    }

    fn term_to_shape_label(term: &RDF::Term) -> Result<ShapeExprLabel> {
        let object = RDF::term_as_object(term);
        match object {
            Object::Iri { iri } => Ok(ShapeExprLabel::iri(iri)),
            Object::BlankNode(bnode) => Ok(ShapeExprLabel::bnode(BNode::new(bnode.as_str()))),
            Object::Literal(lit) => Err(ShExRError::ShapeExprLabelLiteral { lit }),
        }
    }

    fn shape_decl() -> impl RDFNodeParse<RDF, Output = ShapeDecl> {
        (term().flat_map(move |ref t| { 
            let label = Self::term_to_shape_label(t).map_err(|e| {
              RDFParseError::Custom { msg: format!("Expected term to be a label: {t}: {e}")}  
            })?;
            Ok(label)
        })).and(Self::parse_shape_expr()).map(|(label, se)| {
            ShapeDecl::new(label,  se, false )
        })
    }

    fn parse_shape_expr() -> impl RDFNodeParse<RDF, Output = ShapeExpr> {
        property_value(&sx_shape_expr()).then(|ref node| {
            set_focus(node).then(|_| shape_expr())
        })
    }

}



fn shape_expr_<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr> 
where RDF: FocusRDF {
   // I would like the following code to work...but it doesn't yet...
   /*parse_by_type(vec![
    (sx_shape_and(), shape_and()),
    . . .
    (sx_node_constraint(), node_constraint())
   ], default) */

   shape_and().or(shape_or()).or(shape()).or(node_constraint())
}

fn shape<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr> 
where RDF: FocusRDF {
    has_type(sx_shape()).with({
        closed().then(|maybe_closed| {
            println!("Value of closed: {maybe_closed:?}");
            let extra = None; // TODO
            let expression = None; // TODO
            ok(&ShapeExpr::shape(Shape::new(maybe_closed, extra, expression)))
        }
        )
    })
}

fn closed<RDF>() -> impl RDFNodeParse<RDF, Output = Option<bool>> 
where RDF: FocusRDF {
    optional(property_bool(&sx_closed()))
}

rdf_parser! {
    pub fn shape_and[RDF]()(RDF) -> ShapeExpr where [] {
        has_type(sx_shape_and()).with(
        property_value(&sx_shape_exprs()).then(|ref node| {
            set_focus(node).and(
                   parse_rdf_list::<RDF, _>(shape_expr())
                 ).map(|(_,vs)| { ShapeExpr::and(vs) }) 
           }))
    }
}

rdf_parser! {
    pub fn shape_or[RDF]()(RDF) -> ShapeExpr where [] {
        has_type(sx_shape_or()).with(
        property_value(&sx_shape_exprs()).then(|ref node| {
            set_focus(node).and(
                   parse_rdf_list::<RDF, _>(shape_expr())
                 ).map(|(_,vs)| { ShapeExpr::or(vs) }) 
           }))
    }
}


rdf_parser!{
    pub fn shape_expr[RDF]()(RDF) -> ShapeExpr where [] {
       shape_expr_()
    }
}

#[inline]
fn sx_schema<RDF>() -> RDF::Term 
where RDF: FocusRDF {
    RDFParser::<RDF>::term_iri_unchecked(SX_SCHEMA)
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


fn sx_node_constraint() -> IriS {
    IriS::new_unchecked(SX_NODECONSTRAINT)
}

rdf_parser!{
 fn node_constraint[RDF]()(RDF) -> ShapeExpr 
 where [] {
    parse_nodekind().then(|maybe_nodekind|
        parse_value_set().then(move |maybe_valueset| {
           let mut nc = NodeConstraint::new();
           if let Some(nk) = &maybe_nodekind {
              nc = nc.with_node_kind(nk.clone());
           }
           if let Some(vs) = &maybe_valueset {
              nc = nc.with_values(vs.clone())
           }
           ok(&ShapeExpr::node_constraint(nc))
         }
        )
    )
  }
}

fn parse_nodekind<RDF>() -> impl RDFNodeParse<RDF, Output = Option<NodeKind>> 
where RDF: FocusRDF {
    optional(
        property_value(&sx_node_kind()).then(|ref node| {
            set_focus(node).and(nodekind()
            ).map(|(_,vs)| { vs })
        })
    )
}

fn nodekind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind> 
where RDF: FocusRDF {
    iri_kind().or(
        literal_kind()).or(
            bnode_kind()).or(
                nonliteral_kind())
}

fn iri_kind<RDF> () -> impl RDFNodeParse<RDF, Output = NodeKind> 
where RDF: FocusRDF {
    is_iri(ShExRVocab::sx_iri()).map(|_| NodeKind::Iri)
}

fn literal_kind<RDF> () -> impl RDFNodeParse<RDF, Output = NodeKind> 
where RDF: FocusRDF {
    is_iri(ShExRVocab::sx_literal()).map(|_| NodeKind::Literal)
}

fn bnode_kind<RDF> () -> impl RDFNodeParse<RDF, Output = NodeKind> 
where RDF: FocusRDF {
    is_iri(ShExRVocab::sx_bnode()).map(|_| NodeKind::BNode)
}

fn nonliteral_kind<RDF>() -> impl RDFNodeParse<RDF, Output = NodeKind> 
where RDF: FocusRDF {
    is_iri(ShExRVocab::sx_nonliteral()).map(|_| NodeKind::NonLiteral)
}


fn parse_datatype<RDF>() -> Result<Option<IriRef>> 
where RDF: FocusRDF {
    // TODO
    Ok(None)
}

fn parse_value_set<RDF>() -> impl RDFNodeParse<RDF, Output = Option<Vec<ValueSetValue>>> 
where RDF: FocusRDF {
    optional(
        property_value(&sx_values()).then(|ref node| {
            set_focus(node).and(
                parse_rdf_list::<RDF, _>(parse_value())).map(|(_,vs)| { vs })
        })
    )
}

fn parse_value<RDF>() -> impl RDFNodeParse<RDF, Output = ValueSetValue> 
where RDF: FocusRDF {
    //firstOf(objectValue, )
    object_value().map(|ov| ValueSetValue::ObjectValue(ov))
}

fn object_value<RDF>() -> impl RDFNodeParse<RDF, Output = ObjectValue> 
where RDF: FocusRDF {
    iri().map(|ref iri| { 
        ObjectValue::IriRef(IriRef::Iri(iri.clone()))
    })
}

fn parse_xs_facet<RDF>() -> Result<Option<Vec<XsFacet>>> 
where RDF: FocusRDF {
    // TODO
    Ok(None)
}

