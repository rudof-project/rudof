#!/usr/bin/env python3
"""
Example: Using configuration files with rudof_generate Python bindings

This example demonstrates how to use TOML and JSON configuration files,
similar to the CLI interface.
"""

import pyrudof
import tempfile
import os
import json


def example_toml_config():
    """Example creating and using a TOML configuration file"""
    print("=" * 70)
    print("Example 1: TOML Configuration File")
    print("=" * 70)
    
    # Create a comprehensive configuration
    config = pyrudof.GeneratorConfig()
    
    # Generation settings
    config.set_entity_count(1000)
    config.set_seed(42)
    config.set_entity_distribution(pyrudof.EntityDistribution.Equal)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    config.set_schema_format(pyrudof.SchemaFormat.ShEx)
    
    # Field generation settings
    config.set_locale("en")
    config.set_data_quality(pyrudof.DataQuality.High)
    
    # Output settings
    config.set_output_path("output/data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_compress(False)
    config.set_write_stats(True)
    config.set_parallel_writing(True)
    config.set_parallel_file_count(4)
    
    # Parallel processing settings
    config.set_worker_threads(8)
    config.set_batch_size(100)
    config.set_parallel_shapes(True)
    config.set_parallel_fields(True)
    
    # Save to TOML file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
        toml_path = f.name
    
    try:
        config.to_toml_file(toml_path)
        print(f"✓ Configuration saved to: {toml_path}\n")
        
        # Show the TOML content
        with open(toml_path, 'r') as f:
            content = f.read()
            print("TOML Configuration:")
            print("-" * 70)
            print(content)
            print("-" * 70)
        
        # Load the configuration back
        loaded_config = pyrudof.GeneratorConfig.from_toml_file(toml_path)
        print(f"\n✓ Configuration loaded from TOML file")
        print(f"  Entity count: {loaded_config.get_entity_count()}")
        print(f"  Seed: {loaded_config.get_seed()}")
        print(f"  Locale: {loaded_config.get_locale()}")
        print(f"  Worker threads: {loaded_config.get_worker_threads()}")
        
        # Use it to create a generator
        generator = pyrudof.DataGenerator(loaded_config)
        print(f"✓ DataGenerator created from loaded config\n")
        
    finally:
        if os.path.exists(toml_path):
            os.unlink(toml_path)


def example_json_config():
    """Example creating and using a JSON configuration file"""
    print("\n" + "=" * 70)
    print("Example 2: JSON Configuration File (Manual)")
    print("=" * 70)
    
    # Create a JSON configuration manually
    json_config = {
        "generation": {
            "entity_count": 500,
            "seed": 12345,
            "entity_distribution": "Equal",
            "cardinality_strategy": "Random",
            "schema_format": "ShEx"
        },
        "field_generators": {
            "default": {
                "locale": "es",
                "quality": "Medium"
            },
            "datatypes": {},
            "properties": {}
        },
        "output": {
            "path": "output/spanish_data.ttl",
            "format": "Turtle",
            "compress": False,
            "write_stats": True,
            "parallel_writing": False,
            "parallel_file_count": 0
        },
        "parallel": {
            "worker_threads": None,
            "batch_size": 50,
            "parallel_shapes": True,
            "parallel_fields": True
        }
    }
    
    # Save to JSON file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json_path = f.name
        json.dump(json_config, f, indent=2)
    
    try:
        print(f"✓ JSON configuration created: {json_path}\n")
        
        # Show the JSON content
        with open(json_path, 'r') as f:
            content = f.read()
            print("JSON Configuration:")
            print("-" * 70)
            print(content)
            print("-" * 70)
        
        # Load the configuration
        loaded_config = pyrudof.GeneratorConfig.from_json_file(json_path)
        print(f"\n✓ Configuration loaded from JSON file")
        print(f"  Entity count: {loaded_config.get_entity_count()}")
        print(f"  Seed: {loaded_config.get_seed()}")
        print(f"  Locale: {loaded_config.get_locale()}")
        print(f"  Batch size: {loaded_config.get_batch_size()}")
        
        # Validate
        loaded_config.validate()
        print(f"✓ Configuration validated successfully")
        
        # Use it to create a generator
        generator = pyrudof.DataGenerator(loaded_config)
        print(f"✓ DataGenerator created from loaded config\n")
        
    finally:
        if os.path.exists(json_path):
            os.unlink(json_path)


def example_cli_like_workflow():
    """Example simulating CLI-like workflow with config file + overrides"""
    print("\n" + "=" * 70)
    print("Example 3: CLI-like Workflow (Config File + Overrides)")
    print("=" * 70)
    
    # Create a base configuration file (like --config in CLI)
    base_config = pyrudof.GeneratorConfig()
    base_config.set_locale("en")
    base_config.set_data_quality(pyrudof.DataQuality.High)
    base_config.set_batch_size(100)
    base_config.set_parallel_shapes(True)
    base_config.set_parallel_fields(True)
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
        config_path = f.name
    
    try:
        base_config.to_toml_file(config_path)
        print(f"✓ Base configuration saved to: {config_path}")
        
        # Load base configuration (like --config myconfig.toml)
        config = pyrudof.GeneratorConfig.from_toml_file(config_path)
        print(f"✓ Base configuration loaded")
        
        # Apply command-line style overrides
        config.set_entity_count(2000)  # Like --entities 2000
        config.set_output_path("/tmp/custom_output.ttl")  # Like --output /tmp/custom_output.ttl
        config.set_seed(99999)  # Like --seed 99999
        config.set_worker_threads(16)  # Like --parallel 16
        
        print(f"\n  Applied overrides:")
        print(f"    --entities {config.get_entity_count()}")
        print(f"    --output {config.get_output_path()}")
        print(f"    --seed {config.get_seed()}")
        print(f"    --parallel {config.get_worker_threads()}")
        
        # Validate merged configuration
        config.validate()
        print(f"\n✓ Merged configuration validated")
        
        # Create generator (like running the CLI)
        generator = pyrudof.DataGenerator(config)
        print(f"✓ DataGenerator ready to run")
        print(f"\n  To generate data, you would call:")
        print(f"    generator.run('schema.shex')")
        print(f"  Or:")
        print(f"    generator.load_shex_schema('schema.shex')")
        print(f"    generator.generate()\n")
        
    finally:
        if os.path.exists(config_path):
            os.unlink(config_path)


def example_different_locales():
    """Example showing different locale configurations"""
    print("\n" + "=" * 70)
    print("Example 4: Different Locale Configurations")
    print("=" * 70)
    
    locales = ["en", "es", "fr", "de", "it"]
    
    for locale in locales:
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        config.set_locale(locale)
        config.set_output_path(f"output/data_{locale}.ttl")
        
        generator = pyrudof.DataGenerator(config)
        print(f"  ✓ Generator configured for locale '{locale}' -> {config.get_output_path()}")


def example_quality_levels():
    """Example showing different data quality levels"""
    print("\n" + "=" * 70)
    print("Example 5: Data Quality Levels")
    print("=" * 70)
    
    qualities = [
        (pyrudof.DataQuality.Low, "Fast, simple data"),
        (pyrudof.DataQuality.Medium, "Realistic patterns"),
        (pyrudof.DataQuality.High, "Complex, correlated data"),
    ]
    
    for quality, description in qualities:
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        config.set_data_quality(quality)
        
        generator = pyrudof.DataGenerator(config)
        print(f"  ✓ {quality}: {description}")


def main():
    """Run all configuration file examples"""
    print("\n" + "=" * 70)
    print(" RUDOF_GENERATE - Configuration File Examples")
    print("=" * 70)
    print("\nThese examples show how to use configuration files,")
    print("similar to the CLI interface.\n")
    
    try:
        example_toml_config()
        example_json_config()
        example_cli_like_workflow()
        example_different_locales()
        example_quality_levels()
        
        print("\n" + "=" * 70)
        print("✓ All configuration file examples completed!")
        print("=" * 70)
        print("\nKey takeaways:")
        print("  1. Save configurations to TOML/JSON files for reuse")
        print("  2. Load configurations from files: from_toml_file() / from_json_file()")
        print("  3. Override specific settings after loading (like CLI --options)")
        print("  4. Use validate() to check configuration before running")
        print("  5. Set locale and quality for customized data generation")
        print("\n")
        
    except Exception as e:
        print(f"\n✗ Error running examples: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0


if __name__ == "__main__":
    exit(main())
