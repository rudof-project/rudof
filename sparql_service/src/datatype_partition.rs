use iri_s::IriS;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
pub struct DatatypePartition {
    datatype: IriS,
}
