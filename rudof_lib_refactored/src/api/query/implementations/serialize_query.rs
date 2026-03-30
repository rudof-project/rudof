use crate::{Result, Rudof, errors::QueryError};
use std::io;

pub fn serialize_query<W: io::Write>(rudof: &Rudof, writer: &mut W) -> Result<()> {
    let query = rudof.query.as_ref().ok_or(QueryError::NoQueryLoaded)?;

    writeln!(writer, "{}", query.serialize())
        .map_err(|e| QueryError::FailedSerializingQuery { error: e.to_string() })?;

    Ok(())
}
