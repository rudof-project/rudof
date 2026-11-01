"""Load a GeneratorConfig from TOML and from JSON: the same format (version 2) in both."""

import json
from pathlib import Path
from tempfile import TemporaryDirectory

from pyrudof import GeneratorConfig

TOML = """
version = 2

[run]
seed = 11
scale = { entities = 3 }

[output]
dir = "out"
"""


def main() -> None:
    with TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)

        toml_path = tmp_path / "generator.toml"
        toml_path.write_text(TOML, encoding="utf-8")
        loaded_toml = GeneratorConfig.from_toml_file(toml_path)
        loaded_toml.validate()
        print(f"from TOML, entity_count: {loaded_toml.get_entity_count()}")
        print(f"from TOML, seed: {loaded_toml.get_seed()}")

        json_path = tmp_path / "generator.json"
        json_path.write_text(
            json.dumps(
                {
                    "version": 2,
                    "run": {"seed": 5, "scale": {"entities": 4}},
                    "defaults": {"cardinality": "min"},
                    "output": {"dir": str(tmp_path / "out"), "formats": ["ntriples"]},
                }
            ),
            encoding="utf-8",
        )
        loaded_json = GeneratorConfig.from_json_file(json_path)
        print(f"from JSON, entity_count: {loaded_json.get_entity_count()}")


if __name__ == "__main__":
    main()
