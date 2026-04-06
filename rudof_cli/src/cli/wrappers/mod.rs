mod comparison;
mod conversion;
mod data;
mod dctap;
mod generation;
mod node;
mod pgschema;
mod query;
mod rdf_config;
mod service;
mod shacl;
mod shapemap;
mod shex;
mod validation;

pub use comparison::*;
pub use conversion::*;
pub use data::*;
pub use dctap::*;
pub use generation::*;
pub use node::*;
pub use pgschema::*;
pub use query::*;
pub use rdf_config::*;
pub use service::*;
pub use shacl::*;
pub use shapemap::*;
pub use shex::*;
pub use validation::*;

/// CLI wrapper macro for rudof_lib types.
///
/// This macro creates a CLI-friendly enum wrapper around a core library type,
/// adding `clap::ValueEnum` support while delegating conversion logic to the lib type.
///
/// # What it generates:
/// - An enum with `#[derive(ValueEnum)]` for CLI parsing
/// - `From<CliType> -> LibType` conversion using `Display` + `FromStr`
/// - `Display` implementation via `clap::ValueEnum::to_possible_value()` (avoids recursion)
///
/// # Requirements:
/// The core library type must implement:
/// - `FromStr` (for parsing the lowercase variant name produced by `Display`)
#[macro_export]
macro_rules! cli_wrapper {
    (
        $cli:ident,
        $core:ident,
        { $($variant:ident),* $(,)? }
    ) => {
        /// CLI wrapper enum with clap::ValueEnum support.
        #[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
        #[clap(rename_all = "lower")]
        pub enum $cli {
            $( $variant ),*
        }

        /// Convert CLI enum to core library type.
        impl From<$cli> for $core {
            fn from(cli: $cli) -> Self {
                let s = cli.to_string();
                s.parse().unwrap_or_else(|e| {
                    panic!(
                        "CLI enum variant {:?} doesn't match lib enum: {:?}",
                        cli, e
                    )
                })
            }
        }

        /// Display implementation using clap's ValueEnum to avoid recursion.
        impl Display for $cli {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                let val = self.to_possible_value().expect("no skipped variants");
                write!(f, "{}", val.get_name())
            }
        }
    };
}
