use crate::{Result, Rudof, errors::QueryError};
use std::io;

pub fn serialize_sparql_query<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    let query = rudof.sparql_query.as_ref().ok_or(QueryError::NoQueryLoaded)?;

    writeln!(writer, "{}", query.serialize())
        .map_err(|e| QueryError::FailedSerializingQuery { error: e.to_string() })?;

    Ok(())
}
