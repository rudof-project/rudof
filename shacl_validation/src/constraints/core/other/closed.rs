use shacl_ast::compiled::component::Closed;
use srdf::Rdf;

use crate::constraints::Evaluator;
use crate::value_nodes::IterationStrategy;

impl<R: Rdf, I: IterationStrategy> Evaluator<R, I> for Closed<R> {}
