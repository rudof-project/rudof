"""Load a generator schema explicitly (ShEx, SHACL) or by auto-detection."""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)

        shex_out = tmp_path / "from_shex.ttl"
        config_shex = GeneratorConfig()
        config_shex.set_entity_count(2)
        config_shex.set_seed(1)
        config_shex.set_output_path(shex_out)
        config_shex.set_output_format(OutputFormat.Turtle)
        config_shex.set_schema_format(SchemaFormat.ShEx)
        gen_shex = DataGenerator(config_shex)
        gen_shex.load_shex_schema(HERE / "person.shex")
        gen_shex.generate()
        print(f"from ShEx: {shex_out.stat().st_size > 0}")

        shacl_out = tmp_path / "from_shacl.ttl"
        config_shacl = GeneratorConfig()
        config_shacl.set_entity_count(2)
        config_shacl.set_seed(1)
        config_shacl.set_output_path(shacl_out)
        config_shacl.set_output_format(OutputFormat.Turtle)
        config_shacl.set_schema_format(SchemaFormat.Shacl)
        gen_shacl = DataGenerator(config_shacl)
        gen_shacl.load_shacl_schema(HERE / "timbl_shapes.ttl")
        gen_shacl.generate()
        print(f"from SHACL: {shacl_out.stat().st_size > 0}")

        auto_out = tmp_path / "auto.ttl"
        config_auto = GeneratorConfig()
        config_auto.set_entity_count(2)
        config_auto.set_seed(1)
        config_auto.set_output_path(auto_out)
        config_auto.set_output_format(OutputFormat.Turtle)
        gen_auto = DataGenerator(config_auto)
        gen_auto.load_schema_auto(HERE / "person.shex")
        gen_auto.generate()
        print(f"auto-detected: {auto_out.stat().st_size > 0}")


if __name__ == "__main__":
    main()
