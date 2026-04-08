from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat

with TemporaryDirectory() as tmpdir:
    tmp_path = Path(tmpdir)

    shex_out = tmp_path / "from_shex.ttl"
    config_shex = GeneratorConfig()
    config_shex.set_entity_count(1)
    config_shex.set_output_path(str(shex_out))
    config_shex.set_output_format(OutputFormat.Turtle)
    config_shex.set_schema_format(SchemaFormat.ShEx)
    gen_shex = DataGenerator(config_shex)
    gen_shex.load_shex_schema("../../examples/simple.shex")
    gen_shex.generate()

    config_shacl = GeneratorConfig()
    config_shacl.set_entity_count(1)
    config_shacl.set_output_path(str(tmp_path / "from_shacl.ttl"))
    config_shacl.set_output_format(OutputFormat.Turtle)
    config_shacl.set_schema_format(SchemaFormat.Shacl)
    gen_shacl = DataGenerator(config_shacl)
    gen_shacl.load_shacl_schema("../../examples/simple_shacl.ttl")

    config_auto = GeneratorConfig()
    config_auto.set_entity_count(1)
    config_auto.set_output_path(str(tmp_path / "auto.ttl"))
    config_auto.set_output_format(OutputFormat.Turtle)
    gen_auto = DataGenerator(config_auto)
    gen_auto.load_schema_auto("../../examples/simple.shex")
