"""Semantic-less tool for the Semantic Web.

``pyrudof`` wraps `rudof <https://github.com/rudof-project/rudof>`_.

The entry point is :class:`Rudof`, a **stateful session**. Data, schemas and results stay
loaded across calls until a ``reset_*`` method clears them, which is what lets a pipeline
read once and then validate, query and serialize against the same graph.
"""

from importlib.metadata import PackageNotFoundError, version as _version

from ._pyrudof import (
    # --- exceptions ---
    RudofError,
    ComparisonError,
    ConfigError,
    ConversionError,
    DCTapError,
    DataError,
    GenerateError,
    InputError,
    IriError,
    MapStateError,
    MaterializeError,
    NodeInspectionError,
    PgDbError,
    PgSchemaError,
    PrefixesError,
    QueryError,
    RdfConfigError,
    ServiceError,
    ShExError,
    ShaclError,
    ShapeMapError,
    UnsupportedOperationError,
    ValidationError,
    # --- session ---
    Rudof,
    RudofConfig,
    # --- results ---
    ShExValidationReport,
    ShExValidationEntry,
    ShExValidationEntryIterator,
    ShaclValidationReport,
    ShaclValidationEntry,
    ShaclValidationEntryIterator,
    PgSchemaValidationReport,
    PgSchemaValidationEntry,
    PgSchemaValidationEntryIterator,
    QueryResults,
    QueryRowIterator,
    NeighborArc,
    NeighborArcIterator,
    ArcDirection,
    # --- formats: data ---
    RDFFormat,
    ResultDataFormat,
    ReaderMode,
    # --- formats: shex ---
    ShExFormat,
    ResultShexValidationFormat,
    ShexValidationSortMode,
    # --- formats: shacl ---
    ShaclFormat,
    ShaclValidationMode,
    ShaclValidationSortMode,
    ResultShaclValidationFormat,
    ShapesGraphSource,
    # --- formats: shapemap ---
    ShapeMapFormat,
    # --- formats: dctap ---
    DCTapFormat,
    ResultDCTapFormat,
    # --- formats: property graph ---
    PgSchemaFormat,
    ResultPgSchemaValidationFormat,
    DbEngine,
    DdlDialect,
    # --- formats: query ---
    QueryType,
    QueryResultFormat,
    # --- formats: conversion ---
    ConversionMode,
    ResultConversionMode,
    ConversionFormat,
    ResultConversionFormat,
    # --- formats: service and rdf-config ---
    ServiceDescriptionFormat,
    RdfConfigFormat,
    ResultRdfConfigFormat,
    # --- generation ---
    DataGenerator,
    GeneratorConfig,
    SchemaFormat,
    OutputFormat,
    CardinalityStrategy,
    EntityDistribution,
    DataQuality,
)

try:
    __version__ = _version("pyrudof")
except PackageNotFoundError:  # pragma: no cover
    __version__ = "0.0.0+unknown"

__all__ = [
    # Errors.
    "RudofError",
    "ComparisonError",
    "ConfigError",
    "ConversionError",
    "DCTapError",
    "DataError",
    "GenerateError",
    "InputError",
    "IriError",
    "MapStateError",
    "MaterializeError",
    "NodeInspectionError",
    "PgDbError",
    "PgSchemaError",
    "PrefixesError",
    "QueryError",
    "RdfConfigError",
    "ServiceError",
    "ShExError",
    "ShaclError",
    "ShapeMapError",
    "UnsupportedOperationError",
    "ValidationError",
    # Session.
    "Rudof",
    "RudofConfig",
    # Results.
    "ShExValidationReport",
    "ShExValidationEntry",
    "ShExValidationEntryIterator",
    "ShaclValidationReport",
    "ShaclValidationEntry",
    "ShaclValidationEntryIterator",
    "PgSchemaValidationReport",
    "PgSchemaValidationEntry",
    "PgSchemaValidationEntryIterator",
    "QueryResults",
    "QueryRowIterator",
    "NeighborArc",
    "NeighborArcIterator",
    "ArcDirection",
    # Formats: data.
    "RDFFormat",
    "ResultDataFormat",
    "ReaderMode",
    # Formats: shex.
    "ShExFormat",
    "ResultShexValidationFormat",
    "ShexValidationSortMode",
    # Formats: shacl.
    "ShaclFormat",
    "ShaclValidationMode",
    "ShaclValidationSortMode",
    "ResultShaclValidationFormat",
    "ShapesGraphSource",
    # Formats: shapemap.
    "ShapeMapFormat",
    # Formats: dctap.
    "DCTapFormat",
    "ResultDCTapFormat",
    # Formats: property graph.
    "PgSchemaFormat",
    "ResultPgSchemaValidationFormat",
    "DbEngine",
    "DdlDialect",
    # Formats: query.
    "QueryType",
    "QueryResultFormat",
    # Formats: conversion.
    "ConversionMode",
    "ResultConversionMode",
    "ConversionFormat",
    "ResultConversionFormat",
    # Formats: service and rdf-config.
    "ServiceDescriptionFormat",
    "RdfConfigFormat",
    "ResultRdfConfigFormat",
    # Generation.
    "DataGenerator",
    "GeneratorConfig",
    "SchemaFormat",
    "OutputFormat",
    "CardinalityStrategy",
    "EntityDistribution",
    "DataQuality",
]
