use crate::cli::parser::LoadArgs;
use crate::commands::base::{Command, CommandContext};
use crate::commands::connect::ConnectionDetails;
use anyhow::Result;
use rudof_lib::formats::{DataFormat, DataReaderMode, ShaclFormat};

/// Implementation of the `load` command.
///
/// Copies RDF data into a LadybugDB property graph database. By default the
/// data is validated against SHACL shapes before anything is inserted, so
/// the database only ever receives conforming data; subsequent mutations are
/// constrained by the node/rel tables (DDL) created from the data
/// (see discussion #747).
pub struct LoadCommand {
    args: LoadArgs,
}

impl LoadCommand {
    pub fn new(args: LoadArgs) -> Self {
        Self { args }
    }
}

impl Command for LoadCommand {
    fn name(&self) -> &'static str {
        "load"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;
        let details = ConnectionDetails::resolve(args.db.as_deref(), args.connection.as_deref())?;

        let data_format: DataFormat = args.data_format.into();
        let reader_mode: DataReaderMode = args.reader_mode.into();
        let shapes_format: ShaclFormat = args.shapes_format.into();

        let mut loading = ctx
            .rudof
            .load_pg_db(&args.data, &mut ctx.writer)
            .with_db(&details.path, details.read_only)
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode)
            .with_shapes_format(&shapes_format)
            .with_skip_validation(args.skip_validation);
        if let Some(shapes) = &args.shapes {
            loading = loading.with_shapes(shapes);
        }
        if let Some(base) = args.base_data.as_deref() {
            loading = loading.with_base_data(base);
        }
        if let Some(base) = args.base_shapes.as_deref() {
            loading = loading.with_base_shapes(base);
        }
        loading.execute()?;

        Ok(())
    }
}
