use crate::Rudof;

pub fn reset_query_results(rudof: &mut Rudof) {
    rudof.sparql_query = None;
    rudof.query_results = None;
}
