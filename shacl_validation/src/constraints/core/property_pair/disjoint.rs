use shacl_ast::compiled::component::Disjoint;
use srdf::Query;
use srdf::Sparql;

use crate::constraints::Evaluator;
use crate::value_nodes::IterationStrategy;

impl<Q: Query, I: IterationStrategy> Evaluator<Q, I> for Disjoint<Q> {}

impl<S: Sparql, I: IterationStrategy> Evaluator<S, I> for Disjoint<S> {}
