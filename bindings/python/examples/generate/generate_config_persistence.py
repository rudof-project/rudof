import json
from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import GeneratorConfig

base = GeneratorConfig()
base.set_entity_count(3)
base.set_seed(11)
base.set_output_path("persist_output.ttl")

with TemporaryDirectory() as tmpdir:
    tmp_path = Path(tmpdir)

    toml_path = tmp_path / "generator.toml"
    base.to_toml_file(str(toml_path))
    loaded_toml = GeneratorConfig.from_toml_file(str(toml_path))
    loaded_toml.validate()

    json_path = tmp_path / "generator.json"
    json_path.write_text(
        json.dumps(
            {
                "generation": {
                    "entity_count": 4,
                    "seed": 5,
                    "schema_format": "ShEx",
                    "cardinality_strategy": "Minimum",
                    "entity_distribution": "Equal",
                },
                "output": {
                    "path": str(tmp_path / "out.nt"),
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
    loaded_json = GeneratorConfig.from_json_file(str(json_path))
