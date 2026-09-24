"""Configure the locale, quality and entity-distribution settings."""

from pyrudof import DataQuality, EntityDistribution, GeneratorConfig


def main() -> None:
    config = GeneratorConfig()
    config.set_entity_distribution(EntityDistribution.Equal)
    config.set_locale("en")
    config.set_data_quality(DataQuality.Medium)

    print(f"locale: {config.get_locale()}")


if __name__ == "__main__":
    main()
