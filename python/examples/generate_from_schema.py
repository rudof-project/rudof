#!/usr/bin/env python3
"""
Practical example: Generate synthetic data from a ShEx schema

This example demonstrates how to generate synthetic RDF data from
the example ShEx schema in the repository.
"""

import pyrudof
import os
import sys

def generate_from_schema():
    """Generate data from the simple.shex example schema"""
    
    # Path to example schema
    schema_path = "../../examples/simple.shex"
    
    # Check if schema exists
    if not os.path.exists(schema_path):
        print(f"Warning: Schema file not found at {schema_path}")
        print("This is a demonstration of the API, even without the actual file.")
        schema_exists = False
    else:
        schema_exists = True
        print(f"✓ Found schema: {schema_path}")
    
    # Create configuration
    config = pyrudof.GeneratorConfig()
    
    # Configure generation
    config.set_entity_count(20)
    config.set_seed(42)  # For reproducible results
    
    # Configure output
    output_dir = "/tmp/pyrudof_generate"
    os.makedirs(output_dir, exist_ok=True)
    output_file = os.path.join(output_dir, "generated_simple.ttl")
    stats_file = os.path.join(output_dir, "generated_simple_stats.json")
    
    config.set_output_path(output_file)
    config.set_output_format(pyrudof.OutputFormat.Turtle)
    config.set_write_stats(True)
    config.set_compress(False)
    
    # Configure schema
    config.set_schema_format(pyrudof.SchemaFormat.ShEx)
    config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
    
    print("\nConfiguration:")
    print(f"  Entities to generate: {config.get_entity_count()}")
    print(f"  Random seed: {config.get_seed()}")
    print(f"  Output file: {config.get_output_path()}")
    print(f"  Statistics file: {stats_file}")
    
    # Create generator
    print("\n✓ Creating DataGenerator...")
    generator = pyrudof.DataGenerator(config)
    print("✓ DataGenerator created successfully")
    
    if schema_exists:
        try:
            # Load schema and generate
            print(f"\nLoading schema from: {schema_path}")
            generator.run(schema_path)
            
            print(f"\n✓ Data generation completed!")
            print(f"  Output written to: {output_file}")
            
            if os.path.exists(stats_file):
                print(f"  Statistics written to: {stats_file}")
            
            # Show file size
            if os.path.exists(output_file):
                size = os.path.getsize(output_file)
                print(f"  Generated file size: {size} bytes")
                
                # Show first few lines
                print(f"\nFirst 10 lines of generated data:")
                print("-" * 60)
                with open(output_file, 'r') as f:
                    for i, line in enumerate(f):
                        if i >= 10:
                            break
                        print(line.rstrip())
                print("-" * 60)
            
        except Exception as e:
            print(f"\n✗ Error during generation: {e}")
            import traceback
            traceback.print_exc()
            return 1
    else:
        print("\nSkipping actual generation (schema file not found)")
        print("To run with a real schema, provide a valid ShEx or SHACL file.")
    
    return 0

if __name__ == "__main__":
    print("=" * 60)
    print("Practical Example: Generate Synthetic RDF Data")
    print("=" * 60)
    
    result = generate_from_schema()
    
    print("\n" + "=" * 60)
    if result == 0:
        print("Example completed successfully!")
    else:
        print("Example completed with errors.")
    print("=" * 60)
    
    sys.exit(result)
