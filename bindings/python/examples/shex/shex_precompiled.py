from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import RDFFormat, ResultShexValidationFormat, ShapeMapFormat, ShExFormat, Rudof, RudofConfig

data = """
PREFIX : <http://example.org/>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

:alice :name "Alice" ; :age 30 .
"""

shapemap = ":alice@:Person"

# Compile the schema to a cache file once...
rudof = Rudof(RudofConfig())
rudof.read_shex("person.shex", ShExFormat.ShExC)

with TemporaryDirectory() as tmpdir:
    cache_path = Path(tmpdir) / "person.shexcache"
    rudof.compile_shex_to_file(str(cache_path))

    # ...then reuse it to skip parsing and AST-to-IR compilation on future runs.
    rudof2 = Rudof(RudofConfig())
    rudof2.read_shex_precompiled(str(cache_path))
    rudof2.read_data(data, RDFFormat.Turtle)
    rudof2.read_shapemap(shapemap, ShapeMapFormat.Compact)
    rudof2.validate_shex()
    print(rudof2.serialize_shex_validation_results(ResultShexValidationFormat.Compact))
