use srdf::model::rdf::TObject;
use srdf::model::rdf::TPredicate;
use srdf::model::rdf::Rdf;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Target<R: Rdf> {
    TargetNode(TObject<R>),
    TargetClass(TObject<R>),
    TargetSubjectsOf(TPredicate<R>),
    TargetObjectsOf(TPredicate<R>),
}
