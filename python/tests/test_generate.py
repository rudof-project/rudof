"""
Unit tests for rudof_generate Python bindings
"""

import unittest
import tempfile
import os
from pathlib import Path

import pyrudof


class TestGeneratorConfig(unittest.TestCase):
    """Test cases for GeneratorConfig class"""

    def test_default_config(self):
        """Test creating a default configuration"""
        config = pyrudof.GeneratorConfig()
        self.assertIsNotNone(config)

    def test_entity_count(self):
        """Test setting and getting entity count"""
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        self.assertEqual(config.get_entity_count(), 100)

    def test_seed(self):
        """Test setting and getting random seed"""
        config = pyrudof.GeneratorConfig()
        config.set_seed(42)
        self.assertEqual(config.get_seed(), 42)
        
        # Test None seed
        config.set_seed(None)
        self.assertIsNone(config.get_seed())

    def test_output_path(self):
        """Test setting and getting output path"""
        config = pyrudof.GeneratorConfig()
        test_path = "/tmp/test_output.ttl"
        config.set_output_path(test_path)
        self.assertEqual(config.get_output_path(), test_path)

    def test_output_format(self):
        """Test setting output format"""
        config = pyrudof.GeneratorConfig()
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        # No getter for format, just ensure it doesn't raise

        config.set_output_format(pyrudof.OutputFormat.NTriples)

    def test_schema_format(self):
        """Test setting schema format"""
        config = pyrudof.GeneratorConfig()
        config.set_schema_format(pyrudof.SchemaFormat.ShEx)
        config.set_schema_format(pyrudof.SchemaFormat.SHACL)
        config.set_schema_format(None)

    def test_cardinality_strategy(self):
        """Test setting cardinality strategy"""
        config = pyrudof.GeneratorConfig()
        strategies = [
            pyrudof.CardinalityStrategy.Minimum,
            pyrudof.CardinalityStrategy.Maximum,
            pyrudof.CardinalityStrategy.Random,
            pyrudof.CardinalityStrategy.Balanced,
        ]
        for strategy in strategies:
            config.set_cardinality_strategy(strategy)

    def test_compress(self):
        """Test setting compress option"""
        config = pyrudof.GeneratorConfig()
        config.set_compress(True)
        config.set_compress(False)

    def test_write_stats(self):
        """Test setting write_stats option"""
        config = pyrudof.GeneratorConfig()
        config.set_write_stats(True)
        config.set_write_stats(False)

    def test_parallel_writing(self):
        """Test setting parallel_writing option"""
        config = pyrudof.GeneratorConfig()
        config.set_parallel_writing(True)
        config.set_parallel_writing(False)

    def test_parallel_file_count(self):
        """Test setting parallel file count"""
        config = pyrudof.GeneratorConfig()
        config.set_parallel_file_count(4)

    def test_worker_threads(self):
        """Test setting worker threads"""
        config = pyrudof.GeneratorConfig()
        config.set_worker_threads(4)
        config.set_worker_threads(None)

    def test_batch_size(self):
        """Test setting batch size"""
        config = pyrudof.GeneratorConfig()
        config.set_batch_size(100)

    def test_show(self):
        """Test configuration string representation"""
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(50)
        config.set_seed(123)
        
        config_str = config.show()
        self.assertIsInstance(config_str, str)
        self.assertIn("GeneratorConfig", config_str)

    def test_config_from_toml_file_nonexistent(self):
        """Test loading from non-existent TOML file"""
        with self.assertRaises(Exception):
            pyrudof.GeneratorConfig.from_toml_file("/nonexistent/path.toml")

    def test_config_from_json_file_nonexistent(self):
        """Test loading from non-existent JSON file"""
        with self.assertRaises(Exception):
            pyrudof.GeneratorConfig.from_json_file("/nonexistent/path.json")

    def test_config_to_toml_file(self):
        """Test saving configuration to TOML file"""
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(100)
        config.set_seed(42)
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.toml', delete=False) as f:
            temp_path = f.name
        
        try:
            config.to_toml_file(temp_path)
            self.assertTrue(os.path.exists(temp_path))
            
            # Load it back
            loaded_config = pyrudof.GeneratorConfig.from_toml_file(temp_path)
            self.assertEqual(loaded_config.get_entity_count(), 100)
            self.assertEqual(loaded_config.get_seed(), 42)
        finally:
            if os.path.exists(temp_path):
                os.unlink(temp_path)


class TestDataGenerator(unittest.TestCase):
    """Test cases for DataGenerator class"""

    def test_create_generator(self):
        """Test creating a data generator"""
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(10)
        
        generator = pyrudof.DataGenerator(config)
        self.assertIsNotNone(generator)

    def test_create_generator_with_custom_config(self):
        """Test creating generator with custom configuration"""
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(50)
        config.set_seed(12345)
        config.set_output_path("/tmp/test.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        
        generator = pyrudof.DataGenerator(config)
        self.assertIsNotNone(generator)

    def test_load_nonexistent_schema(self):
        """Test loading a non-existent schema file"""
        config = pyrudof.GeneratorConfig()
        generator = pyrudof.DataGenerator(config)
        
        with self.assertRaises(Exception):
            generator.load_shex_schema("/nonexistent/schema.shex")

    def test_load_schema_auto_nonexistent(self):
        """Test auto-loading a non-existent schema file"""
        config = pyrudof.GeneratorConfig()
        generator = pyrudof.DataGenerator(config)
        
        with self.assertRaises(Exception):
            generator.load_schema_auto("/nonexistent/schema.shex")


class TestEnums(unittest.TestCase):
    """Test cases for enum types"""

    def test_schema_format_enum(self):
        """Test SchemaFormat enum values"""
        self.assertNotEqual(pyrudof.SchemaFormat.ShEx, pyrudof.SchemaFormat.SHACL)
        
        # Test equality
        self.assertEqual(pyrudof.SchemaFormat.ShEx, pyrudof.SchemaFormat.ShEx)
        self.assertEqual(pyrudof.SchemaFormat.SHACL, pyrudof.SchemaFormat.SHACL)

    def test_output_format_enum(self):
        """Test OutputFormat enum values"""
        self.assertNotEqual(pyrudof.OutputFormat.Turtle, pyrudof.OutputFormat.NTriples)
        
        # Test equality
        self.assertEqual(pyrudof.OutputFormat.Turtle, pyrudof.OutputFormat.Turtle)
        self.assertEqual(pyrudof.OutputFormat.NTriples, pyrudof.OutputFormat.NTriples)

    def test_cardinality_strategy_enum(self):
        """Test CardinalityStrategy enum values"""
        strategies = [
            pyrudof.CardinalityStrategy.Minimum,
            pyrudof.CardinalityStrategy.Maximum,
            pyrudof.CardinalityStrategy.Random,
            pyrudof.CardinalityStrategy.Balanced,
        ]
        
        # All strategies should be unique
        for i, s1 in enumerate(strategies):
            for j, s2 in enumerate(strategies):
                if i == j:
                    self.assertEqual(s1, s2)
                else:
                    self.assertNotEqual(s1, s2)


class TestIntegration(unittest.TestCase):
    """Integration tests for the complete workflow"""

    def test_complete_configuration_workflow(self):
        """Test a complete configuration and setup workflow"""
        # Create and configure
        config = pyrudof.GeneratorConfig()
        config.set_entity_count(20)
        config.set_seed(42)
        config.set_output_path("/tmp/integration_test.ttl")
        config.set_output_format(pyrudof.OutputFormat.Turtle)
        config.set_schema_format(pyrudof.SchemaFormat.ShEx)
        config.set_cardinality_strategy(pyrudof.CardinalityStrategy.Balanced)
        config.set_write_stats(True)
        config.set_compress(False)
        config.set_worker_threads(2)
        config.set_batch_size(10)
        
        # Create generator
        generator = pyrudof.DataGenerator(config)
        self.assertIsNotNone(generator)
        
        # Verify configuration
        self.assertEqual(config.get_entity_count(), 20)
        self.assertEqual(config.get_seed(), 42)


if __name__ == '__main__':
    unittest.main(verbosity=2)
