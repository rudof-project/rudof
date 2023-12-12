use iri_s::IriS;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemAct {
    name: IriS,
    code: Option<String>,
}
