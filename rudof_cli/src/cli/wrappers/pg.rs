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
