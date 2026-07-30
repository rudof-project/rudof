from pyrudof import ShExFormat, Rudof, RudofConfig

rudof = Rudof(RudofConfig())
rudof.read_shex("person.shex", ShExFormat.ShExC)

serialized = rudof.serialize_current_shex()
