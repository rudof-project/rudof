"""Run generation in one call, with an explicit schema format or by auto-detection."""

from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import DataGenerator, GeneratorConfig, OutputFormat, SchemaFormat

HERE = Path(__file__).resolve().parent.parent


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)

        out_with_format = tmp_path / "run_with_format.ttl"
        config1 = GeneratorConfig()
        config1.set_entity_count(2)
        config1.set_seed(3)
        config1.set_output_path(out_with_format)
        config1.set_output_format(OutputFormat.Turtle)
        generator1 = DataGenerator(config1)
        generator1.run_with_format(HERE / "person.shex", SchemaFormat.ShEx)
        print(f"run_with_format wrote output: {out_with_format.stat().st_size > 0}")

        out_auto = tmp_path / "run_auto.ttl"
        config2 = GeneratorConfig()
        config2.set_entity_count(2)
        config2.set_seed(3)
        config2.set_output_path(out_auto)
        config2.set_output_format(OutputFormat.Turtle)
        generator2 = DataGenerator(config2)
        generator2.run(HERE / "person.shex")
        print(f"run wrote output: {out_auto.stat().st_size > 0}")


if __name__ == "__main__":
    main()
