# Type stubs for pyrudof
from typing import List, Optional, Tuple
from enum import IntEnum

__version__: str
__author__: str

# ---------------------------------------------------------------------------
# Enums — core
# ---------------------------------------------------------------------------

class RDFFormat(IntEnum):
    """RDF serialization format for reading or writing data graphs."""
    Turtle = 0
    NTriples = 1
    RdfXml = 2
    TriG = 3
    N3 = 4
    NQuads = 5
    JsonLd = 6

class ResultDataFormat(IntEnum):
    """Output format when serializing an RDF data graph."""
    Turtle = 0
    NTriples = 1
    RdfXml = 2
    TriG = 3
    N3 = 4
    NQuads = 5
    Compact = 6
    Json = 7
    PlantUML = 8
    Svg = 9
    Png = 10

class ShExFormat(IntEnum):
    """ShEx schema serialization format."""
    ShExC = 0
    ShExJ = 1
    Turtle = 2

class ResultShexValidationFormat(IntEnum):
    """Output format for ShEx validation results."""
    Details = 0
    Turtle = 1
    NTriples = 2
    RdfXml = 3
    TriG = 4
    N3 = 5
    NQuads = 6
    Compact = 7
    Json = 8
    Csv = 9

class ShaclFormat(IntEnum):
    """SHACL shapes graph serialization format."""
    Turtle = 0
    NTriples = 1
    RdfXml = 2
    TriG = 3
    N3 = 4
    NQuads = 5

class ShaclValidationMode(IntEnum):
    """SHACL validation engine to use."""
    Native = 0
    Sparql = 1

class ShaclValidationSortMode(IntEnum):
    """Sort order for SHACL validation result rows."""
    Severity = 0
    Node = 1
    Component = 2
    Value = 3
    Path = 4
    SourceShape = 5
    Details = 6

class ResultShaclValidationFormat(IntEnum):
    """Output format for SHACL validation results."""
    Details = 0
    Turtle = 1
    NTriples = 2
    RdfXml = 3
    TriG = 4
    N3 = 5
    NQuads = 6
    Minimal = 7
    Compact = 8
    Json = 9
    Csv = 10

class ShapeMapFormat(IntEnum):
    """ShapeMap serialization format."""
    Compact = 0
    Json = 1

class DCTapFormat(IntEnum):
    """DCTAP input format."""
    Csv = 0
    Xlsx = 1

class ResultDCTapFormat(IntEnum):
    """Output format for DCTAP serialization."""
    Internal = 0
    Json = 1

class ConversionMode(IntEnum):
    """Input schema language used during conversion."""
    Shacl = 0
    ShEx = 1
    Dctap = 2

class ResultConversionMode(IntEnum):
    """Target representation produced by schema conversion."""
    Sparql = 0
    ShEx = 1
    Uml = 2
    Html = 3
    Shacl = 4

class ConversionFormat(IntEnum):
    """Input serialization format used during conversion."""
    Csv = 0
    ShExC = 1
    ShExJ = 2
    Turtle = 3
    Xlsx = 4

class ResultConversionFormat(IntEnum):
    """Output serialization format produced by conversion."""
    Default = 0
    Internal = 1
    Json = 2
    ShExC = 3
    ShExJ = 4
    Turtle = 5
    PlantUML = 6
    Html = 7
    Svg = 8
    Png = 9

class ServiceDescriptionFormat(IntEnum):
    """Service Description serialization format."""
    Internal = 0
    Json = 1
    Mie = 2

class QueryResultFormat(IntEnum):
    """Output format for SPARQL query results."""
    Turtle = 0
    NTriples = 1
    RdfXml = 2
    TriG = 3
    N3 = 4
    NQuads = 5
    Csv = 6

class QueryType(IntEnum):
    """SPARQL query type."""
    Select = 0
    Construct = 1
    Ask = 2
    Describe = 3

class ReaderMode(IntEnum):
    """Parser error-handling strategy."""
    Lax = 0
    Strict = 1

class SortModeResultMap(IntEnum):
    """Sort order for ShEx validation result rows."""
    Node = 0
    Shape = 1
    Status = 2
    Details = 3

class ShapesGraphSource(IntEnum):
    """Source of the SHACL shapes graph."""
    CurrentData = 0
    CurrentSchema = 1

# ---------------------------------------------------------------------------
# Enums — data generation
# ---------------------------------------------------------------------------

class SchemaFormat(IntEnum):
    """Schema format for the data generator."""
    ShEx = 0
    SHACL = 1

class OutputFormat(IntEnum):
    """Output format for generated RDF data."""
    Turtle = 0
    NTriples = 1

class CardinalityStrategy(IntEnum):
    """Strategy for handling cardinalities during generation."""
    Minimum = 0
    Maximum = 1
    Random = 2
    Balanced = 3

class EntityDistribution(IntEnum):
    """Distribution strategy across shapes when generating entities."""
    Equal = 0

class DataQuality(IntEnum):
    """Data quality level for generated values."""
    Low = 0
    Medium = 1
    High = 2

# ---------------------------------------------------------------------------
# Core classes
# ---------------------------------------------------------------------------

class RudofError(Exception):
    """Exception raised for errors in rudof operations."""
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class RudofConfig:
    """Configuration for a Rudof instance.

    Can be created with defaults or loaded from a TOML configuration file.
    Once created it is immutable.
    """

    def __init__(self) -> None:
        """Create a RudofConfig with default values."""
        ...

    @staticmethod
    def from_path(path: str) -> 'RudofConfig':
        """Load configuration from a TOML file.

        Args:
            path: Path to the configuration file.

        Raises:
            RudofError: If the file cannot be read or parsed.
        """
        ...

    def __repr__(self) -> str: ...

class Rudof:
    """Main interface for RDF data, schemas, validation, and queries.

    A single instance maintains independent state for:

    * RDF data graph
    * ShEx schema + ShapeMap
    * SHACL shapes graph
    * SPARQL query + results
    * DCTAP profile
    * Service description
    """

    def __init__(self, config: RudofConfig) -> None:
        """Create a Rudof instance with the given configuration.

        Args:
            config: Configuration object.

        Raises:
            RudofError: If initialization fails.
        """
        ...

    def __repr__(self) -> str: ...

    # -- Configuration -------------------------------------------------------

    def update_config(self, config: RudofConfig) -> None:
        """Replace the current configuration.

        Does not affect already-loaded data or schemas.

        Args:
            config: New configuration to apply.
        """
        ...

    # -- Reset ---------------------------------------------------------------

    def reset_data(self) -> None:
        """Clear the loaded RDF data graph."""
        ...

    def reset_shex(self) -> None:
        """Clear the loaded ShEx schema."""
        ...

    def reset_shacl(self) -> None:
        """Clear the loaded SHACL shapes graph."""
        ...

    def reset_shapemap(self) -> None:
        """Clear the loaded ShapeMap."""
        ...

    def reset_query(self) -> None:
        """Clear the loaded SPARQL query."""
        ...

    def reset_all(self) -> None:
        """Clear all loaded state (data, schemas, queries, validation results)."""
        ...

    def reset_validation_results(self) -> None:
        """Clear ShEx validation results."""
        ...

    # -- Version -------------------------------------------------------------

    def get_version(self) -> str:
        """Return the Rudof library version string (semver format)."""
        ...

    # -- Node inspection -----------------------------------------------------

    def node_info(
        self,
        node_selector: str,
        predicates: Optional[List[str]] = None,
        mode: Optional[str] = None,
        show_colors: Optional[bool] = None,
        depth: Optional[int] = None,
    ) -> str:
        """Return formatted information about a node in the RDF graph.

        Args:
            node_selector: IRI (``<http://…>``), prefixed name (``:alice``), or
                blank node (``_:b1``).
            predicates: Restrict output to these predicates. Empty list = all.
            mode: One of ``"outgoing"``, ``"incoming"``, ``"both"`` (default).
            show_colors: Use ANSI terminal colors. Defaults to ``True``.
            depth: Neighborhood distance. Defaults to ``1``.

        Returns:
            Formatted string describing the node neighbourhood.

        Raises:
            RudofError: If the node selector is invalid.
        """
        ...

    # -- RDF data ------------------------------------------------------------

    def read_data(
        self,
        input: Optional[str] = None,
        format: Optional[RDFFormat] = None,
        base: Optional[str] = None,
        reader_mode: Optional[ReaderMode] = None,
        merge: Optional[bool] = None,
    ) -> None:
        """Load RDF data from a string, file path, or URL.

        Args:
            input: Inline RDF string, file path, or URL. ``None`` reads stdin.
            format: Serialization format. Default: ``RDFFormat.Turtle``.
            base: Base IRI for resolving relative IRIs.
            reader_mode: Error-handling strategy. Default: ``ReaderMode.Lax``.
            merge: If ``True`` merge with existing data; ``False`` replaces it.

        Raises:
            RudofError: If input is unreadable or malformed (in Strict mode).
        """
        ...

    def serialize_data(self, format: Optional[ResultDataFormat] = None) -> str:
        """Serialize the loaded RDF data to a string.

        Args:
            format: Output format. Default: ``ResultDataFormat.Compact``.

        Returns:
            Serialized RDF data.

        Raises:
            RudofError: If serialization fails.
        """
        ...

    # -- ShEx ----------------------------------------------------------------

    def read_shex(
        self,
        input: str,
        format: Optional[ShExFormat] = None,
        base: Optional[str] = None,
        reader_mode: Optional[ReaderMode] = None,
    ) -> None:
        """Load a ShEx schema from a string, file path, or URL.

        Args:
            input: Inline ShEx string, file path, or URL.
            format: Schema format. Default: ``ShExFormat.ShExC``.
            base: Base IRI.
            reader_mode: Error-handling strategy. Default: ``ReaderMode.Lax``.

        Raises:
            RudofError: If the schema is unreadable or malformed.
        """
        ...

    def serialize_current_shex(
        self,
        shape_label: Optional[str] = None,
        show_dependencies: Optional[bool] = None,
        show_statistics: Optional[bool] = None,
        show_schema: Optional[bool] = None,
        show_time: Optional[bool] = None,
        format: Optional[ShExFormat] = None,
    ) -> str:
        """Serialize the loaded ShEx schema to a string.

        Args:
            shape_label: Restrict output to this shape.
            show_dependencies: Include dependency information.
            show_statistics: Include schema statistics.
            show_schema: Include full schema.
            show_time: Include timing information.
            format: Output format. Default: ``ShExFormat.ShExC``.

        Returns:
            Serialized ShEx schema.

        Raises:
            RudofError: If no schema is loaded or serialization fails.
        """
        ...

    def read_shapemap(
        self,
        input: str,
        format: Optional[ShapeMapFormat] = None,
        base_nodes: Optional[str] = None,
        base_shapes: Optional[str] = None,
    ) -> None:
        """Load a ShapeMap from a string, file path, or URL.

        Args:
            input: Inline ShapeMap string, file path, or URL.
            format: Format. Default: ``ShapeMapFormat.Compact``.
            base_nodes: Base IRI for node IRIs.
            base_shapes: Base IRI for shape IRIs.

        Raises:
            RudofError: If the ShapeMap is unreadable or malformed.
        """
        ...

    def serialize_shapemap(self, format: Optional[ShapeMapFormat] = None) -> str:
        """Serialize the loaded ShapeMap to a string.

        Args:
            format: Output format. Default: ``ShapeMapFormat.Compact``.

        Returns:
            Serialized ShapeMap.

        Raises:
            RudofError: If serialization fails.
        """
        ...

    def validate_shex(self) -> None:
        """Validate the loaded RDF data against the loaded ShEx schema.

        Requires data, schema, and ShapeMap to be loaded first.

        Raises:
            RudofError: If any required component is missing or validation fails.
        """
        ...

    # -- SHACL ---------------------------------------------------------------

    def read_shacl(
        self,
        input: Optional[str] = None,
        format: Optional[ShaclFormat] = None,
        base: Optional[str] = None,
        reader_mode: Optional[ReaderMode] = None,
    ) -> None:
        """Load a SHACL shapes graph from a string, file path, or URL.

        If *input* is ``None``, shapes are extracted from the current data graph.

        Args:
            input: Inline RDF string, file path, or URL.
            format: RDF format. Default: ``ShaclFormat.Turtle``.
            base: Base IRI.
            reader_mode: Error-handling strategy. Default: ``ReaderMode.Lax``.

        Raises:
            RudofError: If shapes are unreadable or malformed.
        """
        ...

    def serialize_shacl(self, format: Optional[ShaclFormat] = None) -> str:
        """Serialize the loaded SHACL shapes graph to a string.

        Args:
            format: Output format. Default: ``ShaclFormat.Turtle``.

        Returns:
            Serialized SHACL shapes.

        Raises:
            RudofError: If no shapes are loaded or serialization fails.
        """
        ...

    def validate_shacl(self, mode: Optional[ShaclValidationMode] = None) -> None:
        """Validate the loaded RDF data against the loaded SHACL shapes.

        Args:
            mode: Validation engine. Default: ``ShaclValidationMode.Native``.

        Raises:
            RudofError: If no data or shapes are loaded, or validation fails.
        """
        ...

    def serialize_shacl_validation_results(
        self,
        format: Optional[ResultShaclValidationFormat] = None,
        sort_mode: Optional[ShaclValidationSortMode] = None,
    ) -> str:
        """Serialize the results of the last SHACL validation to a string.

        Args:
            format: Output format. Default: ``ResultShaclValidationFormat.Details``.
            sort_mode: Sort order. Default: ``ShaclValidationSortMode.Severity``.

        Returns:
            Serialized SHACL validation results.

        Raises:
            RudofError: If no validation results are available or serialization fails.
        """
        ...

    # -- DCTAP ---------------------------------------------------------------

    def read_dctap(self, input: str, format: Optional[DCTapFormat] = None) -> None:
        """Load a DCTAP profile from a string, file path, or URL.

        Args:
            input: Inline CSV string, file path, or URL.
            format: Format. Default: ``DCTapFormat.Csv``.

        Raises:
            RudofError: If the DCTAP data is malformed.
        """
        ...

    def serialize_dctap(self, format: Optional[ResultDCTapFormat] = None) -> str:
        """Serialize the loaded DCTAP profile to a string.

        Args:
            format: Output format. Default: ``ResultDCTapFormat.Internal``.

        Returns:
            Serialized DCTAP profile.

        Raises:
            RudofError: If no DCTAP profile is loaded or serialization fails.
        """
        ...

    # -- SPARQL --------------------------------------------------------------

    def read_query(
        self,
        input: str,
        query_type: Optional[QueryType] = None,
    ) -> None:
        """Load a SPARQL query from a string, file path, or URL.

        Args:
            input: Inline SPARQL string, file path, or URL.
            query_type: Query type hint. Default: auto-detect.

        Raises:
            RudofError: If the query is unreadable or malformed.
        """
        ...

    def run_query(self) -> None:
        """Execute the loaded SPARQL query against the loaded data.

        Raises:
            RudofError: If no query or data is loaded, or execution fails.
        """
        ...

    def serialize_query_results(
        self, format: Optional[QueryResultFormat] = None
    ) -> str:
        """Serialize the results of the last executed query to a string.

        Args:
            format: Output format. Default: ``QueryResultFormat.Csv``.

        Returns:
            Serialized query results.

        Raises:
            RudofError: If no results are available or serialization fails.
        """
        ...

    # -- Endpoints -----------------------------------------------------------

    def list_endpoints(self) -> List[Tuple[str, str]]:
        """Return a list of known SPARQL endpoints as ``(name, url)`` tuples."""
        ...

    # -- Service description -------------------------------------------------

    def read_service_description(
        self,
        input: str,
        format: Optional[RDFFormat] = None,
        base: Optional[str] = None,
        reader_mode: Optional[ReaderMode] = None,
    ) -> None:
        """Load a SPARQL service description from a file path or URL.

        Args:
            input: File path or URL.
            format: RDF format. Default: ``RDFFormat.Turtle``.
            base: Base IRI.
            reader_mode: Error-handling strategy. Default: ``ReaderMode.Lax``.

        Raises:
            RudofError: If input is unreadable or malformed.
        """
        ...

    def serialize_service_description(
        self, format: Optional[ServiceDescriptionFormat] = None
    ) -> str:
        """Serialize the loaded service description to a string.

        Args:
            format: Output format. Default: ``ServiceDescriptionFormat.Internal``.

        Returns:
            Serialized service description.

        Raises:
            RudofError: If no description is loaded or serialization fails.
        """
        ...

    # -- Schema comparison ---------------------------------------------------

    def convert_schemas(
        self,
        schema: str,
        base: Optional[str],
        reader_mode: Optional[ReaderMode],
        input_mode: ConversionMode,
        output_mode: ResultConversionMode,
        input_format: ConversionFormat,
        output_format: ResultConversionFormat,
        shape: Optional[str],
        templates_folder: Optional[str],
        output_folder: Optional[str],
    ) -> str:
        """Convert a schema to a target representation/format.

        Args:
            schema: Input schema (inline string, file path, or URL).
            base: Base IRI used to resolve relative IRIs.
            reader_mode: Error-handling strategy while reading input.
            input_mode: Input schema language.
            output_mode: Target representation.
            input_format: Input schema format.
            output_format: Result format.
            shape: Optional shape label to focus conversion.
            templates_folder: Optional templates folder path (for HTML output).
            output_folder: Optional output folder path.

        Returns:
            Conversion output as string.

        Raises:
            RudofError: If parsing or conversion fails.
        """
        ...

    def compare_schemas(
        self,
        schema1: str,
        schema2: str,
        mode1: str,
        mode2: str,
        format1: str,
        format2: str,
        base1: Optional[str] = None,
        base2: Optional[str] = None,
        label1: Optional[str] = None,
        label2: Optional[str] = None,
        reader_mode: Optional[ReaderMode] = None,
    ) -> str:
        """Compare two schemas for structural equivalence.

        Args:
            schema1: First schema content (inline string, file path, or URL).
            schema2: Second schema content.
            mode1: First schema language, e.g. ``"shex"`` or ``"shacl"``.
            mode2: Second schema language.
            format1: First schema format, e.g. ``"shexc"`` or ``"turtle"``.
            format2: Second schema format.
            base1: Base IRI for the first schema.
            base2: Base IRI for the second schema.
            label1: Shape label to compare in the first schema.
            label2: Shape label to compare in the second schema.
            reader_mode: Error-handling strategy.

        Returns:
            Comparison result as a string.

        Raises:
            RudofError: If either schema is malformed or comparison fails.
        """
        ...

# ---------------------------------------------------------------------------
# Data generation
# ---------------------------------------------------------------------------

class GeneratorConfig:
    """Configuration for the synthetic RDF data generator."""

    def __init__(self) -> None:
        """Create a GeneratorConfig with default values."""
        ...

    @staticmethod
    def from_toml_file(path: str) -> 'GeneratorConfig':
        """Load configuration from a TOML file.

        Args:
            path: Path to the TOML configuration file.

        Raises:
            ValueError: If the file cannot be read or parsed.
        """
        ...

    @staticmethod
    def from_json_file(path: str) -> 'GeneratorConfig':
        """Load configuration from a JSON file.

        Args:
            path: Path to the JSON configuration file.

        Raises:
            ValueError: If the file cannot be read or parsed.
        """
        ...

    def to_toml_file(self, path: str) -> None:
        """Save configuration to a TOML file.

        Args:
            path: Destination path.

        Raises:
            ValueError: If the file cannot be written.
        """
        ...

    def validate(self) -> None:
        """Validate the configuration.

        Raises:
            ValueError: If the configuration is invalid.
        """
        ...

    def show(self) -> str:
        """Return a debug string representation of the configuration."""
        ...

    # Entity / schema settings
    def set_entity_count(self, count: int) -> None: ...
    def get_entity_count(self) -> int: ...
    def set_seed(self, seed: Optional[int]) -> None: ...
    def get_seed(self) -> Optional[int]: ...
    def set_entity_distribution(self, distribution: EntityDistribution) -> None: ...
    def set_cardinality_strategy(self, strategy: CardinalityStrategy) -> None: ...
    def set_schema_format(self, format: Optional[SchemaFormat]) -> None: ...
    def set_data_quality(self, quality: DataQuality) -> None: ...
    def set_locale(self, locale: str) -> None: ...
    def get_locale(self) -> str: ...

    # Output settings
    def set_output_path(self, path: str) -> None: ...
    def get_output_path(self) -> str: ...
    def set_output_format(self, format: OutputFormat) -> None: ...
    def set_compress(self, compress: bool) -> None: ...
    def get_compress(self) -> bool: ...
    def set_write_stats(self, write_stats: bool) -> None: ...
    def get_write_stats(self) -> bool: ...

    # Parallelism settings
    def set_parallel_writing(self, parallel_writing: bool) -> None: ...
    def get_parallel_writing(self) -> bool: ...
    def set_parallel_file_count(self, count: int) -> None: ...
    def get_parallel_file_count(self) -> int: ...
    def set_worker_threads(self, threads: Optional[int]) -> None: ...
    def get_worker_threads(self) -> Optional[int]: ...
    def set_batch_size(self, batch_size: int) -> None: ...
    def get_batch_size(self) -> int: ...
    def set_parallel_shapes(self, enabled: bool) -> None: ...
    def get_parallel_shapes(self) -> bool: ...
    def set_parallel_fields(self, enabled: bool) -> None: ...
    def get_parallel_fields(self) -> bool: ...


class DataGenerator:
    """Generates synthetic RDF data from a ShEx or SHACL schema."""

    def __init__(self, config: GeneratorConfig) -> None:
        """Create a DataGenerator with the given configuration.

        Args:
            config: Generator configuration.

        Raises:
            RuntimeError: If the generator cannot be initialized.
        """
        ...

    def load_shex_schema(self, path: str) -> None:
        """Load a ShEx schema from a file.

        Args:
            path: Path to the ShEx schema file.

        Raises:
            ValueError: If the schema cannot be loaded or parsed.
            RuntimeError: If the generator is not initialized.
        """
        ...

    def load_shacl_schema(self, path: str) -> None:
        """Load a SHACL schema from a file.

        Args:
            path: Path to the SHACL schema file.

        Raises:
            ValueError: If the schema cannot be loaded or parsed.
            RuntimeError: If the generator is not initialized.
        """
        ...

    def load_schema_auto(self, path: str) -> None:
        """Load a schema with automatic format detection.

        Args:
            path: Path to the schema file.

        Raises:
            ValueError: If the schema cannot be loaded or parsed.
            RuntimeError: If the generator is not initialized.
        """
        ...

    def generate(self) -> None:
        """Generate synthetic data and write to the configured output path.

        Requires a schema to be loaded first.

        Raises:
            ValueError: If generation fails.
            RuntimeError: If the generator is not initialized or no schema is loaded.
        """
        ...

    def run_with_format(
        self,
        schema_path: str,
        format: Optional[SchemaFormat] = None,
    ) -> None:
        """Load schema and generate data in one step.

        Args:
            schema_path: Path to the schema file.
            format: Schema format, or ``None`` for auto-detection.

        Raises:
            ValueError: If the schema cannot be loaded or generation fails.
        """
        ...

    def run(self, schema_path: str) -> None:
        """Load schema (auto-detect format) and generate data in one step.

        Args:
            schema_path: Path to the schema file.

        Raises:
            ValueError: If the schema cannot be loaded or generation fails.
        """
        ...
