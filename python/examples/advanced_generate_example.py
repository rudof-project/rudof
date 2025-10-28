#!/usr/bin/env python3
"""
Advanced example: Using rudof_generate Python bindings

This example demonstrates advanced usage patterns for the rudof_generate
Python bindings, including:
- Configuration from files
- Different schema formats (ShEx and SHACL)
- Parallel processing
- Error handling
"""

import pyrudof
import tempfile
import os


def example_basic_generation():
    """Most basic example - minimal configuration"""
    print("\n" + "=" * 70)
    print("Example 1: Basic Data Generation")
    print("=" * 70)
    
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(10)
    config.set_output_path("/tmp/basic_output.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    
    generator = pyrudof.DataGenerator(config)
    print("✓ Created basic generator configuration")
    print(f"  Entities: {config.get_entity_count()}")
    print(f"  Output: {config.get_output_path()}")


def example_reproducible_generation():
    """Example with random seed for reproducible results"""
    print("\n" + "=" * 70)
    print("Example 2: Reproducible Generation")
    print("=" * 70)
    
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(100)
    config.set_seed(42)  # Fixed seed for reproducibility
    config.set_output_path("/tmp/reproducible_output.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    
    generator = pyrudof.DataGenerator(config)
    print("✓ Generator with fixed seed for reproducible results")
    print(f"  Seed: {config.get_seed()}")
    print("  Running this configuration multiple times will produce identical output")


def example_parallel_generation():
    """Example using parallel processing"""
    print("\n" + "=" * 70)
    print("Example 3: Parallel Generation")
    print("=" * 70)
    
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(1000)
    
    # Configure parallel processing
    config.set_worker_threads(4)
    config.set_batch_size(100)
    config.set_parallel_writing(True)
    config.set_parallel_file_count(4)
    
    config.set_output_path("/tmp/parallel_output")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    
    generator = pyrudof.DataGenerator(config)
    print("✓ Generator configured for parallel processing")
    print(f"  Worker threads: 4")
    print(f"  Batch size: 100")
    print(f"  Parallel files: 4")
    print("  This will generate 4 separate output files")


def example_with_stats():
    """Example that writes generation statistics"""
    print("\n" + "=" * 70)
    print("Example 4: Generation with Statistics")
    print("=" * 70)
    
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(50)
    config.set_output_path("/tmp/output_with_stats.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_write_stats(True)
    
    generator = pyrudof.DataGenerator(config)
    print("✓ Generator will write statistics file")
    print(f"  Data output: {config.get_output_path()}")
    print(f"  Stats output: {config.get_output_path().replace('.ttl', '_stats.json')}")


def example_cardinality_strategies():
    """Example showing different cardinality strategies"""
    print("\n" + "=" * 70)
    print("Example 5: Cardinality Strategies")
    print("=" * 70)
    
    strategies = {
        "Minimum": pyrudof.CardinalityStrategy.Minimum,
        "Maximum": pyrudof.CardinalityStrategy.Maximum,
        "Random": pyrudof.CardinalityStrategy.Random,
        "Balanced": pyrudof.CardinalityStrategy.Balanced,
    }
    
    for name, strategy in strategies.items():
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(20)
        config.set_cardinality_strategy(strategy)
        config.set_output_path(f"/tmp/output_{name.lower()}.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        
        generator = pyrudof.DataGenerator(config)
        print(f"  ✓ {name:10} - Uses {name.lower()} cardinality for relationships")


def example_different_output_formats():
    """Example showing different output formats"""
    print("\n" + "=" * 70)
    print("Example 6: Output Formats")
    print("=" * 70)
    
    # Turtle format (more readable)
    config_turtle = pyrudof.GeneratorConfig()
    config_turtle.set_entity_count(10)
    config_turtle.set_output_path("/tmp/output.ttl")
    config_turtle.set_output_format(pyrudof.OutputFormat.Turtle)
    gen1 = pyrudof.DataGenerator(config_turtle)
    print("  ✓ Turtle format - Human-readable, with prefixes")
    
    # NTriples format (more compact)
    config_ntriples = pyrudof.GeneratorConfig()
    config_ntriples.set_entity_count(10)
    config_ntriples.set_output_path("/tmp/output.nt")
    config_ntriples.set_output_format(pyrudof.OutputFormat.NTriples)
    gen2 = pyrudof.DataGenerator(config_ntriples)
    print("  ✓ NTriples format - Simple, one triple per line")


def example_schema_formats():
    """Example showing different schema formats"""
    print("\n" + "=" * 70)
    print("Example 7: Schema Formats")
    print("=" * 70)
    
    # ShEx schema
    config_shex = pyrudof.GeneratorConfig()
    config_shex.set_schema_format(pyrudof.SchemaFormat.ShEx)
    config_shex.set_entity_count(20)
    config_shex.set_output_path("/tmp/shex_output.ttl")
    gen1 = pyrudof.DataGenerator(config_shex)
    print("  ✓ ShEx schema format")
    print("    Use with: generator.load_shex_schema('schema.shex')")
    
    # SHACL schema
    config_shacl = pyrudof.GeneratorConfig()
    config_shacl.set_schema_format(pyrudof.SchemaFormat.SHACL)
    config_shacl.set_entity_count(20)
    config_shacl.set_output_path("/tmp/shacl_output.ttl")
    gen2 = pyrudof.DataGenerator(config_shacl)
    print("  ✓ SHACL schema format")
    print("    Use with: generator.load_shacl_schema('shapes.ttl')")


def example_config_persistence():
    """Example showing configuration save/load"""
    print("\n" + "=" * 70)
    print("Example 8: Configuration Persistence")
    print("=" * 70)
    
    # Create a configuration
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(100)
    config.set_seed(42)
    config.set_output_path("/tmp/saved_config_output.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    config.set_write_stats(True)
    
    # Save to TOML file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
        config_path = f.name
    
    try:
        config.to_toml_file(config_path)
        print(f"✓ Configuration saved to: {config_path}")
        
        # Load it back
        loaded_config = pyrudof.GeneratorConfig.from_toml_file(config_path)
        print(f"✓ Configuration loaded from file")
        print(f"  Entity count: {loaded_config.get_entity_count()}")
        print(f"  Random seed: {loaded_config.get_seed()}")
        print(f"  Output path: {loaded_config.get_output_path()}")
        
        # Can also load from JSON (if you create a JSON config)
        print("\n  Supported formats:")
        print("    - TOML: GeneratorConfig.from_toml_file(path)")
        print("    - JSON: GeneratorConfig.from_json_file(path)")
    finally:
        if os.path.exists(config_path):
            os.unlink(config_path)


def example_error_handling():
    """Example showing proper error handling"""
    print("\n" + "=" * 70)
    print("Example 9: Error Handling")
    print("=" * 70)
    
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(10)
    config.set_output_path("/tmp/error_test.ttl")
    
    generator = pyrudof.DataGenerator(config)
    
    # Try to load a non-existent schema
    try:
        generator.load_shex_schema("/nonexistent/schema.shex")
        print("  This shouldn't print")
    except Exception as e:
        print(f"✓ Caught expected error when loading non-existent schema:")
        print(f"  Error type: {type(e).__name__}")
        print(f"  Error message: {str(e)[:60]}...")
    
    # Try to load invalid config file
    try:
        config = pyrudof.GeneratorConfig.from_toml_file("/nonexistent/config.toml")
        print("  This shouldn't print")
    except Exception as e:
        print(f"✓ Caught expected error when loading non-existent config:")
        print(f"  Error type: {type(e).__name__}")


def example_complete_workflow():
    """Example of a complete workflow (conceptual)"""
    print("\n" + "=" * 70)
    print("Example 10: Complete Workflow (Conceptual)")
    print("=" * 70)
    
    print("""
A complete workflow would look like:

1. Create and configure:
   config = pyrudof.GeneratorConfig()
   config.set_entity_count(1000)
   config.set_seed(42)
   config.set_output_path("output.ttl")
   config.set_output_format(pyrudof.OutputFormat.Turtle)
   config.set_schema_format(pyrudof.SchemaFormat.ShEx)
   config.set_write_stats(True)

2. Create generator:
   generator = pyrudof.DataGenerator(config)

3. Load schema and generate (option A - separate steps):
   generator.load_shex_schema("schema.shex")
   generator.generate()

4. OR use the convenience method (option B - one step):
   generator.run("schema.shex")

5. Check output:
   - Data file: output.ttl
   - Stats file: output_stats.json

Note: This example shows the API structure. To actually run it,
      you need a valid ShEx or SHACL schema file.
    """)


def main():
    """Run all examples"""
    print("\n" + "=" * 70)
    print(" RUDOF_GENERATE Python Bindings - Advanced Examples")
    print("=" * 70)
    print("\nThese examples demonstrate the Python API for rudof_generate.")
    print("They show configuration patterns without actually generating data.")
    
    try:
        example_basic_generation()
        example_reproducible_generation()
        example_parallel_generation()
        example_with_stats()
        example_cardinality_strategies()
        example_different_output_formats()
        example_schema_formats()
        example_config_persistence()
        example_error_handling()
        example_complete_workflow()
        
        print("\n" + "=" * 70)
        print("✓ All examples completed successfully!")
        print("=" * 70)
        print("\nFor actual data generation, you need:")
        print("  1. A valid ShEx schema (.shex file)")
        print("  2. OR a valid SHACL schema (.ttl file with SHACL shapes)")
        print("\nSee the repository examples/ directory for sample schemas.")
        print("\n")
        
    except Exception as e:
        print(f"\n✗ Error running examples: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0


if __name__ == "__main__":
    exit(main())
