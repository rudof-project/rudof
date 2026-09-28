"""Set and read the core generator configuration values."""

from pyrudof import CardinalityStrategy, GeneratorConfig, OutputFormat, SchemaFormat


def main() -> None:
    config = GeneratorConfig()
    config.set_entity_count(5)
    config.set_seed(7)
    config.set_output_path("core_output.ttl")
    config.set_output_format(OutputFormat.Turtle)
    config.set_schema_format(SchemaFormat.ShEx)
    config.set_cardinality_strategy(CardinalityStrategy.Balanced)

    config.validate()

    print(f"entity_count: {config.get_entity_count()}")
    print(f"seed: {config.get_seed()}")
    print(f"output_path: {config.get_output_path()}")


if __name__ == "__main__":
    main()
