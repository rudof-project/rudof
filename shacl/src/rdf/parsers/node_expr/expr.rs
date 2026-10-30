use crate::ast::ASTNodeExpr;
use crate::rdf::parsers::node_expr::concat::concat;
use crate::rdf::parsers::node_expr::count::count;
use crate::rdf::parsers::node_expr::distinct::distinct;
use crate::rdf::parsers::node_expr::empty::empty;
use crate::rdf::parsers::node_expr::exists::exists;
use crate::rdf::parsers::node_expr::filter_shape::filter_shape;
use crate::rdf::parsers::node_expr::find_first::find_first;
use crate::rdf::parsers::node_expr::flat_map::flat_map;
use crate::rdf::parsers::node_expr::if_expression::if_expresion;
use crate::rdf::parsers::node_expr::instances_of::instances_of;
use crate::rdf::parsers::node_expr::intersection::intersection;
use crate::rdf::parsers::node_expr::limit::limit;
use crate::rdf::parsers::node_expr::list::list;
use crate::rdf::parsers::node_expr::match_all::match_all;
use crate::rdf::parsers::node_expr::max::max;
use crate::rdf::parsers::node_expr::min::min;
use crate::rdf::parsers::node_expr::nodes_matching::nodes_matching;
use crate::rdf::parsers::node_expr::offset::offset;
use crate::rdf::parsers::node_expr::order_by::order_by;
use crate::rdf::parsers::node_expr::path_values::path_values;
use crate::rdf::parsers::node_expr::remove::remove;
use crate::rdf::parsers::node_expr::sum::sum;
use crate::rdf::parsers::node_expr::var::var;
use crate::rdf::parsers::node_expr::{Neighs, to_object};
use prefixmap::IriRef;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{
    MAX_PATH_DEPTH, MIN_PARSE_STEPS, NeighborhoodParser, PARSE_STEPS_PER_TRIPLE, ParseBudget, ValuesPropertyParser,
};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{FocusRDF, RDFError};
use std::marker::PhantomData;
use std::sync::Arc;

/// Parses every value of `property` on the focus node as a node expression.
#[derive(Debug, Clone)]
pub(crate) struct NodeExprsPropertyParser<RDF> {
    property: IriS,
    budget: Arc<ParseBudget>,
    _marker: PhantomData<RDF>,
}

impl<RDF> NodeExprsPropertyParser<RDF> {
    pub fn new(property: IriS, budget: Arc<ParseBudget>) -> Self {
        Self {
            property,
            budget,
            _marker: PhantomData,
        }
    }
}

impl<RDF> RDFNodeParse<RDF> for NodeExprsPropertyParser<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<ASTNodeExpr>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        let focus = rdf.get_focus().cloned();
        let values = ValuesPropertyParser::new(self.property.clone()).parse_focused(rdf)?;

        let exprs = values
            .iter()
            .map(|value| {
                self.budget
                    .spend(rdf, |rdf, steps| node_expr_from_term(value, rdf, 0, steps))
            })
            .collect();

        if let Some(focus) = &focus {
            rdf.set_focus(focus);
        }

        exprs
    }
}

/// Parses `term` as a node expression nested `depth` levels deep, taking one of the `steps` left
/// for each expression and path.
pub(crate) fn node_expr_from_term<RDF: FocusRDF>(
    term: &RDF::Term,
    rdf: &mut RDF,
    depth: usize,
    steps: &mut usize,
) -> Result<ASTNodeExpr, RDFError> {
    // Bounded as SHACL paths are: a cyclic expression would nest forever, and a blank node
    // used twice is parsed twice, so N nested `shnex:concat ( _:x _:x )` would make 2^N
    if depth > MAX_PATH_DEPTH {
        return Err(RDFError::DefaultError {
            msg: format!("{term} is a cyclic node expression, or nests more than {MAX_PATH_DEPTH} levels"),
        });
    }
    let Some(left) = steps.checked_sub(1) else {
        return Err(RDFError::DefaultError {
            msg: format!(
                "Parsing {term} takes more than {MIN_PARSE_STEPS} steps plus {PARSE_STEPS_PER_TRIPLE} per triple of the graph"
            ),
        });
    };
    *steps = left;

    match to_object::<RDF>(term)? {
        Object::Iri(iri) => Ok(ASTNodeExpr::Iri(IriRef::iri(iri))),
        Object::Literal(lit) => Ok(ASTNodeExpr::Literal(lit)),
        Object::BlankNode(_) => blank_node_expr(term, rdf, depth, steps),
        Object::Triple { .. } => Err(RDFError::DefaultError {
            msg: "Triple term expressions are not supported yet".to_string(),
        }),
    }
}

/// Identifies which kind of node expression a blank node carries, and parses it.
fn blank_node_expr<RDF: FocusRDF>(
    term: &RDF::Term,
    rdf: &mut RDF,
    depth: usize,
    steps: &mut usize,
) -> Result<ASTNodeExpr, RDFError> {
    rdf.set_focus(term);

    let neighs: Neighs<RDF> = NeighborhoodParser::new().parse_focused(rdf)?;
    let expr_fns = [
        empty,
        var,
        path_values,
        exists,
        if_expresion,
        distinct,
        intersection,
        concat,
        remove,
        limit,
        offset,
        order_by,
        flat_map,
        count,
        min,
        max,
        sum,
        instances_of,
        nodes_matching,
        list,
        // TODO - These three take a shape, and parsing an inline shape from here would
        // TODO - make the shape parser and this parser mutually recursive.
        filter_shape,
        find_first,
        match_all,
    ];

    for fun in expr_fns {
        if let Some(expr) = fun(&neighs, rdf, term, depth + 1, steps)? {
            return Ok(expr);
        }
    }

    Err(RDFError::DefaultError {
        msg: format!("{term} is not a well-formed node expression"),
    })
}
