use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Target<R: Rdf> {
    TargetNode(TObject<R>),
    TargetClass(TObject<R>),
    TargetSubjectsOf(TPredicate<R>),
    TargetObjectsOf(TPredicate<R>),
}
