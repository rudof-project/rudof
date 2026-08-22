use crate::Rudof;

pub fn reset_sparql_query(rudof: &mut Rudof) {
    rudof.sparql_query = None;
}
