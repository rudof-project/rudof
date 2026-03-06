import pyrudof

# Create configuration
config = pyrudof.GeneratorConfig()

# Configure generation parameters
config.set_entity_count(50)
config.set_seed(12345)  # For reproducible results

# Configure output
config.set_output_path("generated_data.ttl")
config.set_output_format(pyrudof.OutputFormat.Turtle)
config.set_write_stats(True)

# Configure schema
config.set_schema_format(pyrudof.SchemaFormat.ShEx)
config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)

# Configure parallelism
config.set_worker_threads(4)
config.set_batch_size(10)

# Create generator
generator = pyrudof.DataGenerator(config)
print(f"Generator configured: {config.get_entity_count()} entities")
