use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Target<R: Rdf> {
    TargetNode(Object<R>),
    TargetClass(Object<R>),
    TargetSubjectsOf(Predicate<R>),
    TargetObjectsOf(Predicate<R>),
}
