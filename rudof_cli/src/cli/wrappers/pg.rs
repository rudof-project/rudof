use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

/// Dialect used when generating property graph DDL
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum DdlDialectCli {
    /// LadybugDB/Kùzu-style Cypher DDL (`CREATE NODE TABLE` / `CREATE REL TABLE`)
    Cypher,
    /// ISO GQL-style graph type DDL (`CREATE GRAPH TYPE` with `NODE TYPE` / `EDGE TYPE`)
    Gql,
}

impl Display for DdlDialectCli {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let val = self.to_possible_value().expect("no skipped variants");
        write!(f, "{}", val.get_name())
    }
}

/// Database engine opened by `connect` and, indirectly, `load`/`query
/// --dialect cypher` (which resolve the engine from the connection details
/// file `connect` wrote). Only `lbug` (LadybugDB) exists today; the value is
/// persisted in that file precisely so other engines can be added later
/// without another CLI break.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[clap(rename_all = "lower")]
pub enum DbEngineCli {
    /// LadybugDB (see <https://github.com/LadybugDB/ladybug>)
    #[default]
    Lbug,
}

impl Display for DbEngineCli {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let val = self.to_possible_value().expect("no skipped variants");
        write!(f, "{}", val.get_name())
    }
}
