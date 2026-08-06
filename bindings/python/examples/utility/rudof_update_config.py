from pyrudof import Rudof, RudofConfig

initial = RudofConfig()
updated = RudofConfig.from_path("example.toml")

rudof = Rudof(initial)
rudof.update_config(updated)
