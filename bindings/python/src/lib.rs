#![cfg(not(target_family = "wasm"))]
use pyo3::prelude::*;

#[macro_use]
mod macros;

mod api;
mod config;
mod error;
mod formats;
mod generate;
mod input;
mod output;

#[pymodule]
pub mod _pyrudof {
    #[pymodule_export]
    pub use crate::api::PyRudof;

    #[pymodule_export]
    pub use crate::config::PyRudofConfig;

    #[pymodule_export]
    pub use crate::error::{
        ComparisonError, ConfigError, ConversionError, DCTapError, DataError, GenerateError, InputError, IriError,
        MapStateError, MaterializeError, NodeInspectionError, PgDbError, PgSchemaError, PrefixesError, QueryError,
        RdfConfigError, RudofError, ServiceError, ShExError, ShaclError, ShapeMapError, UnsupportedOperationError,
        ValidationError,
    };

    #[pymodule_export]
    pub use crate::formats::{
        PyArcDirection, PyConversionFormat, PyConversionMode, PyDCTapFormat, PyDbEngine, PyDdlDialect, PyNeighborArc,
        PyNodeNeighborhood, PyPgSchemaFormat, PyQueryResultFormat, PyQueryType, PyRDFFormat, PyRdfConfigFormat,
        PyReaderMode, PyResultConversionFormat, PyResultConversionMode, PyResultDCTapFormat, PyResultDataFormat,
        PyResultPgSchemaValidationFormat, PyResultRdfConfigFormat, PyResultShaclValidationFormat,
        PyResultShexValidationFormat, PyServiceDescriptionFormat, PyShExFormat, PyShaclFormat, PyShaclValidationMode,
        PyShaclValidationSortMode, PyShapeMapFormat, PyShapesGraphSource, PyShexValidationSortMode,
    };

    #[pymodule_export]
    pub use crate::generate::{
        PyCardinalityStrategy, PyDataGenerator, PyDataQuality, PyEntityDistribution, PyGeneratorConfig, PyOutputFormat,
        PySchemaFormat,
    };

    #[pymodule_export]
    pub use crate::formats::{
        PyPgSchemaValidationEntries, PyPgSchemaValidationEntry, PyPgSchemaValidationReport, PyQueryResults,
        PyQueryRows, PyShExValidationEntries, PyShExValidationEntry, PyShExValidationReport, PyShaclValidationEntries,
        PyShaclValidationEntry, PyShaclValidationReport,
    };
}

// Produces `pyrudof::stub_info()`, which `src/bin/stub_gen.rs` calls to write the type stub.
#[cfg(feature = "stub-gen")]
::pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
