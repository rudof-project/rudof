from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat

with TemporaryDirectory() as tmpdir:
    tmp_path = Path(tmpdir)

    out_with_format = tmp_path / "run_with_format.ttl"
    config1 = GeneratorConfig()
    config1.set_entity_count(1)
    config1.set_output_path(str(out_with_format))
    config1.set_output_format(OutputFormat.Turtle)
    generator1 = DataGenerator(config1)
    generator1.run_with_format("../../examples/simple.shex", SchemaFormat.ShEx)

    out_auto = tmp_path / "run_auto.ttl"
    config2 = GeneratorConfig()
    config2.set_entity_count(1)
    config2.set_output_path(str(out_auto))
    config2.set_output_format(OutputFormat.Turtle)
    generator2 = DataGenerator(config2)
    generator2.run("../../examples/simple.shex")
