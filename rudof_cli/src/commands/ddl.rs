use crate::cli::parser::DdlArgs;
use crate::cli::wrappers::DdlDialectCli;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::formats::{DataFormat, DataReaderMode, DdlDialect};
use std::io::Write;

/// Implementation of the `ddl` command.
///
/// Derives a property graph schema from RDF data and writes the DDL needed
/// to materialize it in a property graph database. This command is
/// stateless: it never opens a database.
pub struct DdlCommand {
    args: DdlArgs,
}

impl DdlCommand {
    pub fn new(args: DdlArgs) -> Self {
        Self { args }
    }
}

impl Command for DdlCommand {
    fn name(&self) -> &'static str {
        "ddl"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format: DataFormat = self.args.data_format.into();
        let reader_mode: DataReaderMode = self.args.reader_mode.into();
        let dialect: DdlDialect = match self.args.dialect {
            DdlDialectCli::Cypher => DdlDialect::Cypher,
            DdlDialectCli::Gql => DdlDialect::Gql,
        };

        let mut ddl_builder = ctx
            .rudof
            .pg_db_ddl(&self.args.data)
            .with_dialect(&dialect)
            .with_graph_type_name(&self.args.graph_type_name)
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base_data.as_deref() {
            ddl_builder = ddl_builder.with_base_data(base);
        }

        let ddl = ddl_builder.execute()?;
        writeln!(ctx.writer, "{ddl}")?;

        Ok(())
    }
}
