from pyrudof import Rudof, RudofConfig

rudof = Rudof(RudofConfig())
version = rudof.get_version()

print("RUDOF_VERSION_OK")
print(f"Version: {version}")
