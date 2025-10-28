#!/usr/bin/env python3
"""
Example: Using rudof_generate Python bindings to generate synthetic RDF data

This example demonstrates how to:
1. Create and configure a GeneratorConfig
2. Create a DataGenerator
3. Load a schema (ShEx or SHACL)
4. Generate synthetic data
"""

import pyrudof

def example_with_config():
    """Example using configuration"""
    print("=" * 60)
    print("Example: Using GeneratorConfig")
    print("=" * 60)
    
    # Create configuration
    config = pyrudof.GeneratorConfig()
    
    # Configure generation parameters
    config.set_entity_count(50)
    config.set_seed(12345)  # For reproducible results
    
    # Configure output
    config.set_output_path("output/generated_data.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_write_stats(True)
    config.set_compress(False)
    
    # Configure schema
    config.set_schema_format(pyrudof.SchemaFormat.ShEx)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    
    # Configure parallelism
    config.set_worker_threads(4)
    config.set_batch_size(10)
    config.set_parallel_writing(False)
    
    print(f"Entity count: {config.get_entity_count()}")
    print(f"Random seed: {config.get_seed()}")
    print(f"Output path: {config.get_output_path()}")
    
    # Create generator
    generator = pyrudof.DataGenerator(config)
    print("✓ DataGenerator created successfully")
    
    # Note: To actually generate data, you would:
    # generator.load_shex_schema("path/to/schema.shex")
    # generator.generate()
    
    print()

def example_different_formats():
    """Example showing different format options"""
    print("=" * 60)
    print("Example: Different format options")
    print("=" * 60)
    
    # Turtle format
    config1 = pyrudof.GeneratorConfig()
    config1.set_output_format(pyrudof.OutputFormat.Turtle)
    print("✓ Config for Turtle format")
    
    # NTriples format
    config2 = pyrudof.GeneratorConfig()
    config2.set_output_format(pyrudof.OutputFormat.NTriples)
    print("✓ Config for NTriples format")
    
    # ShEx schema
    config3 = pyrudof.GeneratorConfig()
    config3.set_schema_format(pyrudof.SchemaFormat.ShEx)
    print("✓ Config for ShEx schema")
    
    # SHACL schema
    config4 = pyrudof.GeneratorConfig()
    config4.set_schema_format(pyrudof.SchemaFormat.SHACL)
    print("✓ Config for SHACL schema")
    
    print()

def example_cardinality_strategies():
    """Example showing different cardinality strategies"""
    print("=" * 60)
    print("Example: Cardinality strategies")
    print("=" * 60)
    
    strategies = [
        ("Minimum", pyrudof.CardinalityStrategy.Minimum),
        ("Maximum", pyrudof.CardinalityStrategy.Maximum),
        ("Random", pyrudof.CardinalityStrategy.Random),
        ("Balanced", pyrudof.CardinalityStrategy.Balanced),
    ]
    
    for name, strategy in strategies:
        config = pyrudof.GeneratorConfig()
        config.set_cardinality_strategy(strategy)
        print(f"✓ Config with {name} cardinality strategy")
    
    print()

def example_parallel_generation():
    """Example showing parallel generation options"""
    print("=" * 60)
    print("Example: Parallel generation")
    print("=" * 60)
    
    config = pyrudof.GeneratorConfig()
    
    # Enable parallel writing
    config.set_parallel_writing(True)
    config.set_parallel_file_count(4)  # Split into 4 files
    print("✓ Parallel writing enabled with 4 output files")
    
    # Configure worker threads
    config.set_worker_threads(8)
    config.set_batch_size(100)
    print("✓ Using 8 worker threads with batch size of 100")
    
    print()

def example_full_workflow():
    """Example showing the complete workflow (conceptual)"""
    print("=" * 60)
    print("Example: Complete workflow (conceptual)")
    print("=" * 60)
    
    # 1. Create and configure
    config = pyrudof.GeneratorConfig()
    config.set_entity_count(1000)
    config.set_output_path("output/large_dataset.ttl")
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_write_stats(True)
    print("✓ Step 1: Configuration created")
    
    # 2. Create generator
    generator = pyrudof.DataGenerator(config)
    print("✓ Step 2: Generator created")
    
    # 3. Load schema (example - would need actual file)
    print("  Step 3: Would call generator.load_shex_schema('schema.shex')")
    print("         or generator.load_shacl_schema('shapes.ttl')")
    print("         or generator.load_schema_auto('schema_file')")
    
    # 4. Generate data
    print("  Step 4: Would call generator.generate()")
    
    # 5. Alternative: run complete pipeline
    print("  Alternative: generator.run('schema.shex')")
    print("              or generator.run_with_format('schema.ttl', SchemaFormat.SHACL)")
    
    print()

if __name__ == "__main__":
    print("\n" + "=" * 60)
    print("RUDOF Generate Python Bindings - Examples")
    print("=" * 60 + "\n")
    
    example_with_config()
    example_different_formats()
    example_cardinality_strategies()
    example_parallel_generation()
    example_full_workflow()
    
    print("=" * 60)
    print("Examples completed!")
    print("=" * 60)
    print("\nTo use with actual schemas, you would:")
    print("1. Create a GeneratorConfig")
    print("2. Create a DataGenerator(config)")
    print("3. Call generator.load_shex_schema() or load_shacl_schema()")
    print("4. Call generator.generate()")
    print("\nOr use the convenience method:")
    print("   generator.run('path/to/schema')")
    print()
