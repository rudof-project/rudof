from pyrudof import CardinalityStrategy, GeneratorConfig, OutputFormat, SchemaFormat

config = GeneratorConfig()
config.set_entity_count(5)
config.set_seed(7)
config.set_output_path("core_output.ttl")
config.set_output_format(OutputFormat.Turtle)
config.set_schema_format(SchemaFormat.ShEx)
config.set_cardinality_strategy(CardinalityStrategy.Balanced)

print("GEN_CONFIG_CORE_OK")
print(f"Entity count: {config.get_entity_count()}")
print(f"Seed: {config.get_seed()}")
print(f"Output path: {config.get_output_path()}")
