use crate::{
    PgSchemaOperations, Result,
    formats::{InputSpec, ResultPgSchemaValidationFormat}
};
use std::io;

impl PgSchemaOperations for crate::Rudof {
    fn load_pg_schema(
        &mut self,
        pg_schema: &InputSpec,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_pg_schema<W: io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_pg_schema(&mut self) {
        todo!()
    }

    fn run_pgschema_validation(&mut self) -> Result<()> {
        todo!()
    }

    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_format: Option<&ResultPgSchemaValidationFormat>,
        writer: &mut W,
    ) -> Result<()> {
        todo!()
    }

    fn reset_pg_schema_validation(&mut self) {
        todo!()
    }
}
