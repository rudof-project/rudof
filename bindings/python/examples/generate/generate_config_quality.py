from pyrudof import DataQuality, EntityDistribution, GeneratorConfig

config = GeneratorConfig()
config.set_entity_distribution(EntityDistribution.Equal)
config.set_locale("en")
config.set_data_quality(DataQuality.Medium)
