from pyrudof import Rudof, RudofConfig

initial = RudofConfig()
updated = RudofConfig.from_path("../../rudof_lib/src/default_config.toml")

rudof = Rudof(initial)
rudof.update_config(updated)
