use crate::{Rudof, api::pg_db::PgDbOperations};

/// Builder for the `reset_pg_db_connection` operation.
pub struct ResetPgDbConnectionBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetPgDbConnectionBuilder<'a> {
    /// Create a new reset builder.
    ///
    /// Internal: called by `Rudof::reset_pg_db_connection()`.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Execute the reset of the property graph database connection info.
    pub fn execute(self) {
        <Rudof as PgDbOperations>::reset_pg_db_connection(self.rudof)
    }
}
