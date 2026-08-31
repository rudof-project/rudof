use crate::Rudof;

pub fn reset_pg_db_connection(rudof: &mut Rudof) {
    rudof.pg_db_connection = None;
}
