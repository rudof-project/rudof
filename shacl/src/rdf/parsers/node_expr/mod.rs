mod expr;
mod var;
mod path_values;
mod exists;
mod empty;
mod distinct;
mod intersection;
mod concat;
mod remove;
mod limit;
mod offset;
mod order_by;
mod flat_map;
mod count;
mod min;
mod max;
mod sum;
mod instances_of;
mod nodes_matching;
mod if_expression;
mod find_first;
mod filter_shape;
mod match_all;
mod list;

use std::collections::{HashMap, HashSet};
pub(crate) use expr::{node_expr_from_term, NodeExprsPropertyParser};


use crate::ast::ASTNodeExpr;
use rudof_rdf::rdf_core::parser::rdf_node_parser::RDFNodeParse;
use rudof_rdf::rdf_core::parser::rdf_node_parser::constructors::{ListParser, ValuesPropertyParser};
use rudof_rdf::rdf_core::{FocusRDF, RDFError, Rdf};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::parser::rdf_node_parser::utils::term_to_int;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclNodeExprVocab;

/// The outgoing arcs of the blank node a node expression is written on.
type Neighs<RDF> = HashMap<<RDF as Rdf>::IRI, HashSet<<RDF as Rdf>::Term>>;

/// The single value of `property`, if present.
fn single<RDF: FocusRDF>(neighs: &Neighs<RDF>, property: &IriS) -> Result<Option<RDF::Term>, RDFError> {
    match values::<RDF>(neighs, property) {
        None => Ok(None),
        Some(terms) if terms.len() == 1 => Ok(terms.iter().next().cloned()),
        Some(terms) => Err(RDFError::DefaultError {
            msg: format!("{property} must have exactly one value, found {}", terms.len()),
        }),
    }
}

/// Parses the value of `property` as a nested node expression, if present.
fn optional_expr<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    property: &IriS,
    rdf: &mut RDF,
) -> Result<Option<ASTNodeExpr>, RDFError> {
    match single::<RDF>(neighs, property)? {
        None => Ok(None),
        Some(term) => Ok(Some(node_expr_from_term(&term, rdf)?)),
    }
}

/// Parses a SHACL list of node expressions, starting at `head`.
fn expr_list<RDF: FocusRDF>(head: &RDF::Term, rdf: &mut RDF) -> Result<Vec<ASTNodeExpr>, RDFError> {
    rdf.set_focus(head);
    let members = ListParser::new().parse_focused(rdf)?;

    members
        .iter()
        .map(|member| node_expr_from_term(member, rdf))
        .collect()
}

/// Parses `shnex:nodes`, which these expressions require.
fn required_nodes<RDF: FocusRDF>(
    neighs: &Neighs<RDF>,
    rdf: &mut RDF,
    kind: &str,
) -> Result<ASTNodeExpr, RDFError> {
    optional_expr::<RDF>(neighs, &ShaclNodeExprVocab::shnex_nodes(), rdf)?.ok_or_else(|| RDFError::DefaultError {
        msg: format!("{kind} requires shnex:nodes"),
    })
}

fn non_negative<RDF: FocusRDF>(term: &RDF::Term, property: &str) -> Result<usize, RDFError> {
    let value = term_to_int::<RDF>(term)?;

    usize::try_from(value).map_err(|_| RDFError::DefaultError {
        msg: format!("{property} must be non-negative, found {value}"),
    })
}

/// All values of `property` on the node expression, if it has any.
fn values<'a, RDF: FocusRDF>(neighs: &'a Neighs<RDF>, property: &IriS) -> Option<&'a HashSet<RDF::Term>> {
    let key: RDF::IRI = property.clone().into();
    neighs.get(&key)
}

fn to_object<RDF: FocusRDF>(term: &RDF::Term) -> Result<Object, RDFError> {
    term.clone().try_into().map_err(|_| RDFError::FailedTermToObjectError {
        term: term.to_string(),
    })
}
