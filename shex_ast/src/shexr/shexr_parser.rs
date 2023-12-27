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
        // let schema_node = self.rdf_parser.instance_of(&Self::sx_schema())?;
        // self.parse_schema(&schema_node)
        Self::schema_parser().parse_impl(&mut self.rdf_parser.rdf).map_err(|e| {
            ShExRError::RDFParseError { err: e }
        })
    }

    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        instance_of(&ShExRVocab::sx_schema()).then(|node| {
            println!("Node for schema {node}");
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


    /*fn parse_schema(&mut self, node: &RDF::Subject) -> Result<Schema> {
        let mut shapes = Vec::new();
        self.rdf_parser.set_focus(&RDF::subject_as_term(node));
        let shape_nodes = self
            .rdf_parser
            .parse_list_for_predicate(&Self::sx_shapes())?;
        for shape_decl_node in shape_nodes {
            let (label, shape_expr, is_abstract) = self.parse_shape_decl(&shape_decl_node)?;
            shapes.push(ShapeDecl::new(label, shape_expr, is_abstract))
        }
        let maybe_shapes = if shapes.is_empty() {
            None
        } else {
            Some(shapes)
        };
        let prefixmap = self.rdf_parser.prefixmap();
        Ok(Schema::new().with_shapes(maybe_shapes).with_prefixmap(prefixmap))
    }*/

    fn term_as_subject(term: &RDF::Term) -> Result<RDF::Subject> {
        let subj = RDFParser::<RDF>::term_as_subject(term)?;
        Ok(subj)
    }

    fn set_focus(&mut self, focus: &RDF::Term) {
        self.rdf_parser.set_focus(focus)
    }

    /*fn parse_shape_decl(&mut self, node: &RDF::Term) -> Result<(ShapeExprLabel, ShapeExpr, bool)> {
        let label = Self::term_to_shape_label(node)?;
        self.set_focus(node);
        let shape_expr_node = self
            .rdf_parser
            .predicate_value(&Self::sx_shape_expr())?;
        self.set_focus(&shape_expr_node);
        let shape_expr = self.parse_shape_expr()?;
        let is_abstract = false;
        Ok((label, shape_expr, is_abstract))
    }

    fn parse_shape_expr(&mut self) -> Result<ShapeExpr> {
        let shape_expr_type = self.rdf_parser.get_rdf_type()?;
        let iri_type = RDFParser::<RDF>::term_as_iri(&shape_expr_type)?;
        match iri_type.as_str() {
            SX_SHAPE_AND => {
                let mut shape_exprs = Vec::new();
                let ls_nodes = self
                    .rdf_parser
                    .parse_list_for_predicate(&Self::sx_shape_exprs())?;
                for shape_expr_node in ls_nodes.iter() {
                    self.set_focus(&shape_expr_node);
                    let shape_expr = self.parse_shape_expr()?;
                    shape_exprs.push(shape_expr)
                }
                Ok(ShapeExpr::and(shape_exprs))
            }
            SX_NODECONSTRAINT => {
                let nc = self.parse_node_constraint()?;
                Ok(ShapeExpr::NodeConstraint(nc))
            }
            SX_SHAPE => {
                let shape = self.parse_shape()?;
                Ok(ShapeExpr::Shape(shape))
            }
            _ => todo!(),
        }
    }*/

    fn parse_node_constraint(&mut self) -> Result<NodeConstraint> {
        let mut nc = NodeConstraint::new();
        if let Some(node_kind) = Self::parse_nodekind().parse_impl(&mut self.rdf_parser.rdf)? {
            nc = nc.with_node_kind(node_kind)
        };
        if let Some(datatype) = self.parse_datatype()? {
            nc = nc.with_datatype(datatype)
        };
        if let Some(value_set) = Self::parse_value_set().parse_impl(&mut self.rdf_parser.rdf)? {
            nc = nc.with_values(value_set)
        };
        if let Some(xs_facets) = self.parse_xs_facet()? {
            nc = nc.with_xsfacets(xs_facets)
        }
        Ok(nc)
    }

    fn parse_nodekind() -> impl RDFNodeParse<RDF, Output = Option<NodeKind>> {
        optional(
            property_value(&sx_node_kind()).then(|ref node| {
                set_focus(node).and(Self::nodekind()
                ).map(|(_,vs)| { vs })
            })
        )
    }

    fn nodekind() -> impl RDFNodeParse<RDF, Output = NodeKind> {
        Self::iri_kind().or(
            Self::literal_kind()).or(
                Self::bnode_kind()).or(
                    Self::nonliteral_kind())
    }

    fn iri_kind() -> impl RDFNodeParse<RDF, Output = NodeKind> {
        is_iri(ShExRVocab::sx_iri()).map(|_| NodeKind::Iri)
    }

    fn literal_kind() -> impl RDFNodeParse<RDF, Output = NodeKind> {
        is_iri(ShExRVocab::sx_literal()).map(|_| NodeKind::Literal)
    }

    fn bnode_kind() -> impl RDFNodeParse<RDF, Output = NodeKind> {
        is_iri(ShExRVocab::sx_bnode()).map(|_| NodeKind::BNode)
    }

    fn nonliteral_kind() -> impl RDFNodeParse<RDF, Output = NodeKind> {
        is_iri(ShExRVocab::sx_nonliteral()).map(|_| NodeKind::NonLiteral)
    }


    fn parse_datatype(&self) -> Result<Option<IriRef>> {
        // TODO
        Ok(None)
    }

    fn parse_value_set() -> impl RDFNodeParse<RDF, Output = Option<Vec<ValueSetValue>>> {
        optional(
            property_value(&sx_values()).then(|ref node| {
                set_focus(node).and(
                    parse_rdf_list::<RDF, _>(Self::parse_value())).map(|(_,vs)| { vs })
            })
        )
    }

    fn parse_value() -> impl RDFNodeParse<RDF, Output = ValueSetValue> {
        //firstOf(objectValue, )
        Self::object_value().map(|ov| ValueSetValue::ObjectValue(ov))
    }

    fn object_value() -> impl RDFNodeParse<RDF, Output = ObjectValue> {
        iri().map(|ref iri| { 
            ObjectValue::IriRef(IriRef::Iri(iri.clone()))
        })
    }

    fn parse_xs_facet(&self) -> Result<Option<Vec<XsFacet>>> {
        // TODO
        Ok(None)
    }

    fn parse_shape(&self) -> Result<Shape> {
        let closed = None; // TODO
        let extra = None; // TODO
        let expression = None; // TODO
        Ok(Shape::new(closed, extra, expression))
    }
}

fn shape_expr_<RDF>() -> impl RDFNodeParse<RDF, Output = ShapeExpr> 
where RDF: FocusRDF {
    rdf_type().then(|type_term: RDF::Term| {
      term_as_iri(type_term).then(|iri| { 
        let iri_c = iri.clone();
        parse_shape_expr_iri(&iri_c) 
      })
    })
}

rdf_parser!{
    fn parse_shape_expr_iri['a, RDF](iri: &'a IriS)(RDF) -> ShapeExpr where [] 
    {
        match iri.as_str() {
            SX_SHAPE_AND => shape_and(),
            SX_NODECONSTRAINT => todo!(), // node_constraint(),
            _ => todo!()
        }
    }
}

rdf_parser! {
    fn node_constraint[RDF]()(RDF) -> ShapeExpr where [] {
        ok(&ShapeExpr::node_constraint(NodeConstraint::new()))
    }
}

rdf_parser! {
    pub fn shape_and[RDF]()(RDF) -> ShapeExpr where [] {
        property_value(&sx_shape_exprs()).then(|ref node| {
            set_focus(node).and(
                   parse_rdf_list::<RDF, _>(shape_expr())
                 ).map(|(_,vs)| { ShapeExpr::and(vs) }) 
           })
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

#[inline]
fn sx_shape_expr() -> IriS {
    IriS::new_unchecked(SX_SHAPE_EXPR)
}

#[inline]
fn sx_values() -> IriS {
    IriS::new_unchecked(SX_VALUES)
}

#[inline]
fn sx_shape_exprs() -> IriS {
    IriS::new_unchecked(SX_SHAPE_EXPRS)
}

#[inline]
fn sx_node_kind() -> IriS {
    IriS::new_unchecked(SX_NODEKIND)
}
