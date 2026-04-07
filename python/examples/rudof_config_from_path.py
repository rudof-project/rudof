from pyrudof import Rudof, RudofConfig

config = RudofConfig.from_path("../../rudof_lib/src/default_config.toml")
rudof = Rudof(config)

print("RUDOF_CONFIG_FROM_PATH_OK")
print(type(rudof).__name__)
