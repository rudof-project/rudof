# Type stubs for pyrudof.rudof_generate module
from typing import Optional
from enum import Enum

__version__: str

# Enums

class SchemaFormat(Enum):
    """Schema format for the generator"""
    ShEx: int
    SHACL: int

class OutputFormat(Enum):
    """Output format for generated data"""
    Turtle: int
    NTriples: int

class CardinalityStrategy(Enum):
    """Strategy for handling cardinalities in relationships"""
    Minimum: int
    Maximum: int
    Random: int
    Balanced: int

# Classes

class GeneratorConfig:
    """Configuration for the data generator
    
    This class provides configuration options for controlling how synthetic
    RDF data is generated from schemas.
    """
    
    def __init__(self) -> None:
        """Create a new GeneratorConfig with default values"""
        ...
    
    @staticmethod
    def from_toml_file(path: str) -> 'GeneratorConfig':
        """Load configuration from a TOML file
        
        Args:
            path: Path to the TOML configuration file
            
        Returns:
            GeneratorConfig instance loaded from the file
            
        Raises:
            ValueError: If the file cannot be read or parsed
        """
        ...
    
    @staticmethod
    def from_json_file(path: str) -> 'GeneratorConfig':
        """Load configuration from a JSON file
        
        Args:
            path: Path to the JSON configuration file
            
        Returns:
            GeneratorConfig instance loaded from the file
            
        Raises:
            ValueError: If the file cannot be read or parsed
        """
        ...
    
    def to_toml_file(self, path: str) -> None:
        """Save configuration to a TOML file
        
        Args:
            path: Path where the TOML file will be written
            
        Raises:
            ValueError: If the file cannot be written
        """
        ...
    
    def set_entity_count(self, count: int) -> None:
        """Set the number of entities to generate
        
        Args:
            count: Number of entities to generate
        """
        ...
    
    def get_entity_count(self) -> int:
        """Get the number of entities to generate
        
        Returns:
            Number of entities that will be generated
        """
        ...
    
    def set_seed(self, seed: Optional[int]) -> None:
        """Set the random seed for reproducible generation
        
        Args:
            seed: Random seed value, or None for non-deterministic generation
        """
        ...
    
    def get_seed(self) -> Optional[int]:
        """Get the random seed
        
        Returns:
            The random seed value, or None if not set
        """
        ...
    
    def set_output_path(self, path: str) -> None:
        """Set the output file path
        
        Args:
            path: Path to the output file
        """
        ...
    
    def get_output_path(self) -> str:
        """Get the output file path
        
        Returns:
            Path to the output file
        """
        ...
    
    def set_output_format(self, format: OutputFormat) -> None:
        """Set the output format
        
        Args:
            format: Output format (Turtle or NTriples)
        """
        ...
    
    def set_schema_format(self, format: Optional[SchemaFormat]) -> None:
        """Set the schema format
        
        Args:
            format: Schema format (ShEx or SHACL), or None for auto-detection
        """
        ...
    
    def set_cardinality_strategy(self, strategy: CardinalityStrategy) -> None:
        """Set the cardinality strategy
        
        Args:
            strategy: Strategy for handling cardinalities in relationships
        """
        ...
    
    def set_compress(self, compress: bool) -> None:
        """Set whether to compress output
        
        Args:
            compress: True to compress output, False otherwise
        """
        ...
    
    def set_write_stats(self, write_stats: bool) -> None:
        """Set whether to write generation statistics
        
        Args:
            write_stats: True to write statistics file, False otherwise
        """
        ...
    
    def set_parallel_writing(self, parallel_writing: bool) -> None:
        """Set whether to use parallel writing
        
        Args:
            parallel_writing: True to use parallel file writing, False otherwise
        """
        ...
    
    def set_parallel_file_count(self, count: int) -> None:
        """Set the number of parallel output files
        
        Args:
            count: Number of files to write in parallel
        """
        ...
    
    def set_worker_threads(self, threads: Optional[int]) -> None:
        """Set the number of worker threads
        
        Args:
            threads: Number of worker threads, or None for auto-detection
        """
        ...
    
    def set_batch_size(self, batch_size: int) -> None:
        """Set the batch size for parallel processing
        
        Args:
            batch_size: Number of items to process in each batch
        """
        ...
    
    def show(self) -> str:
        """Get a string representation of the configuration
        
        Returns:
            String representation of the configuration
        """
        ...

class DataGenerator:
    """Main data generator class
    
    This class handles loading schemas and generating synthetic RDF data
    based on the provided configuration.
    """
    
    def __init__(self, config: GeneratorConfig) -> None:
        """Create a new DataGenerator with the given configuration
        
        Args:
            config: GeneratorConfig object containing the configuration
            
        Raises:
            RuntimeError: If the generator cannot be initialized
        """
        ...
    
    def load_shex_schema(self, path: str) -> None:
        """Load and process a ShEx schema file
        
        Args:
            path: Path to the ShEx schema file
            
        Raises:
            ValueError: If the schema cannot be loaded or parsed
            RuntimeError: If the generator is not initialized
        """
        ...
    
    def load_shacl_schema(self, path: str) -> None:
        """Load and process a SHACL schema file
        
        Args:
            path: Path to the SHACL schema file
            
        Raises:
            ValueError: If the schema cannot be loaded or parsed
            RuntimeError: If the generator is not initialized
        """
        ...
    
    def load_schema_auto(self, path: str) -> None:
        """Auto-detect schema format and load
        
        Args:
            path: Path to the schema file
            
        Raises:
            ValueError: If the schema cannot be loaded or parsed
            RuntimeError: If the generator is not initialized
        """
        ...
    
    def generate(self) -> None:
        """Generate synthetic data and write to output
        
        The schema must be loaded first using one of the load_*_schema methods.
        
        Raises:
            ValueError: If data generation fails
            RuntimeError: If the generator is not initialized or no schema is loaded
        """
        ...
    
    def run_with_format(self, schema_path: str, format: Optional[SchemaFormat] = None) -> None:
        """Run the complete generation pipeline with schema format detection
        
        This is a convenience method that loads the schema and generates data
        in one step.
        
        Args:
            schema_path: Path to the schema file
            format: Optional schema format (ShEx or SHACL). If None, auto-detect
            
        Raises:
            ValueError: If the schema cannot be loaded or data generation fails
            RuntimeError: If the generator is not initialized
        """
        ...
    
    def run(self, schema_path: str) -> None:
        """Run the complete generation pipeline with automatic schema format detection
        
        This is a convenience method that loads the schema and generates data
        in one step, with automatic format detection.
        
        Args:
            schema_path: Path to the schema file
            
        Raises:
            ValueError: If the schema cannot be loaded or data generation fails
            RuntimeError: If the generator is not initialized
        """
        ...
