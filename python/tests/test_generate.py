import json
import tempfile
import unittest
from pathlib import Path

import pyrudof


class TestGeneratorConfig(unittest.TestCase):
    def test_core_settings(self):
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(12)
        config.set_seed(99)
        config.set_output_path("output.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_schema_format(pyrudof.SchemaFormat.ShEx)
        config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)

        self.assertEqual(config.get_entity_count(), 12)
        self.assertEqual(config.get_seed(), 99)
        self.assertEqual(config.get_output_path(), "output.ttl")

    def test_parallel_settings(self):
        config = pyrudof.GeneratorConfig()
        config.set_compress(True)
        config.set_write_stats(True)
        config.set_parallel_writing(True)
        config.set_parallel_file_count(3)
        config.set_worker_threads(4)
        config.set_batch_size(64)
        config.set_parallel_shapes(True)
        config.set_parallel_fields(True)

        self.assertTrue(config.get_compress())
        self.assertTrue(config.get_write_stats())
        self.assertTrue(config.get_parallel_writing())
        self.assertEqual(config.get_parallel_file_count(), 3)
        self.assertEqual(config.get_worker_threads(), 4)
        self.assertEqual(config.get_batch_size(), 64)
        self.assertTrue(config.get_parallel_shapes())
        self.assertTrue(config.get_parallel_fields())

    def test_quality_and_locale_settings(self):
        config = pyrudof.GeneratorConfig()
        config.set_entity_distribution(pyrudof.EntityDistribution.Equal)
        config.set_locale("en")
        config.set_data_quality(pyrudof.DataQuality.Medium)

        self.assertEqual(config.get_locale(), "en")

    def test_persistence_toml_and_json(self):
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(7)
        config.set_seed(123)

        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)

            toml_path = tmp / "config.toml"
            config.to_toml_file(str(toml_path))
            loaded_toml = pyrudof.GeneratorConfig.from_toml_file(str(toml_path))

            self.assertEqual(loaded_toml.get_entity_count(), 7)
            self.assertEqual(loaded_toml.get_seed(), 123)

            json_path = tmp / "config.json"
            json_path.write_text(
                json.dumps(
                    {
                        "generation": {
                            "entity_count": 4,
                            "seed": 1,
                            "schema_format": "ShEx",
                            "cardinality_strategy": "Minimum",
                            "entity_distribution": "Equal",
                        },
                        "output": {
                            "path": str(tmp / "out.nt"),
                            "format": "NTriples",
                            "compress": False,
                            "write_stats": False,
                            "parallel_writing": False,
                            "parallel_file_count": 1,
                        },
                        "parallel": {
                            "worker_threads": 1,
                            "batch_size": 8,
                            "parallel_shapes": False,
                            "parallel_fields": False,
                        },
                        "field_generators": {
                            "default": {
                                "locale": "en",
                                "quality": "Low",
                            }
                        },
                    }
                ),
                encoding="utf-8",
            )
            loaded_json = pyrudof.GeneratorConfig.from_json_file(str(json_path))

            self.assertEqual(loaded_json.get_entity_count(), 4)

    def test_validate_and_show(self):
        config = pyrudof.GeneratorConfig()
        config.validate()
        shown = config.show()

        self.assertIsInstance(shown, str)
        self.assertIn("GeneratorConfig", shown)

    def test_loading_nonexistent_files_raises(self):
        with self.assertRaises(Exception):
            pyrudof.GeneratorConfig.from_toml_file("/no/such/config.toml")

        with self.assertRaises(Exception):
            pyrudof.GeneratorConfig.from_json_file("/no/such/config.json")


class TestDataGenerator(unittest.TestCase):
    def _make_config(self, output_path: str, schema_format=None):
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(1)
        config.set_output_path(output_path)
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        if schema_format is not None:
            config.set_schema_format(schema_format)
        return config

    def test_load_shex_schema_and_generate(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "from_shex.ttl"
            config = self._make_config(str(output), pyrudof.SchemaFormat.ShEx)
            generator = pyrudof.DataGenerator(config)

            generator.load_shex_schema("../../examples/simple.shex")
            generator.generate()

            self.assertTrue(output.exists())

    def test_load_shacl_schema(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "from_shacl.ttl"
            config = self._make_config(str(output), pyrudof.SchemaFormat.Shacl)
            generator = pyrudof.DataGenerator(config)

            generator.load_shacl_schema("../../examples/simple_shacl.ttl")

    def test_load_schema_auto(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "auto.ttl"
            config = self._make_config(str(output))
            generator = pyrudof.DataGenerator(config)

            generator.load_schema_auto("../../examples/simple.shex")

    def test_run_with_format(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "run_with_format.ttl"
            config = self._make_config(str(output))
            generator = pyrudof.DataGenerator(config)

            generator.run_with_format("../../examples/simple.shex", pyrudof.SchemaFormat.ShEx)

            self.assertTrue(output.exists())

    def test_run(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "run_auto.ttl"
            config = self._make_config(str(output))
            generator = pyrudof.DataGenerator(config)

            generator.run("../../examples/simple.shex")

            self.assertTrue(output.exists())

    def test_nonexistent_schema_raises(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            output = Path(tmpdir) / "none.ttl"
            config = self._make_config(str(output), pyrudof.SchemaFormat.ShEx)
            generator = pyrudof.DataGenerator(config)

            with self.assertRaises(Exception):
                generator.load_shex_schema("/no/such/schema.shex")

            with self.assertRaises(Exception):
                generator.load_schema_auto("/no/such/schema.shex")


if __name__ == "__main__":
    unittest.main(verbosity=2)
