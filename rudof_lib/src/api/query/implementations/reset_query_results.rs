use crate::Rudof;

pub fn reset_query_results(rudof: &mut Rudof) {
    rudof.query = None;
    rudof.query_results = None;
}
