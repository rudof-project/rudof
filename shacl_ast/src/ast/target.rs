use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;
use srdf::model::rdf::TPredicateRef;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Target<R: Rdf> {
    TargetNode(TObjectRef<R>),
    TargetClass(TObjectRef<R>),
    TargetSubjectsOf(TPredicateRef<R>),
    TargetObjectsOf(TPredicateRef<R>),
}
