"""
Registry of documented examples, loaded from ``examples.toml`` and the ``.py`` sources.
"""

import sys
from pathlib import Path
from typing import Any, Optional, TypedDict

if sys.version_info >= (3, 11):
    import tomllib
else:
    import tomli as tomllib


class ExampleFiles(TypedDict, total=False):
    """Files associated with an example, relative to ``bindings/python/examples/``."""

    schema: str
    data: str
    shapemap: Optional[str]
    query: Optional[str]
    config: Optional[str]
    map_state: Optional[str]


class Example(TypedDict):
    """Metadata for a single example."""

    code: str
    files: ExampleFiles
    description: str
    category: str
    title: str
    source_file: str
    expected_output: list[str]
    skip_test: bool


EXAMPLES_DIR = Path(__file__).resolve().parent
_MANIFEST_PATH = EXAMPLES_DIR / "examples.toml"


def _load_manifest() -> dict[str, Any]:
    """Parse ``examples.toml`` and return the raw TOML dict."""
    with open(_MANIFEST_PATH, "rb") as f:
        manifest: dict[str, Any] = tomllib.load(f)
    return manifest


def _validate_entry(entry: dict[str, Any]) -> None:
    """Validate a single ``[[example]]`` entry from ``examples.toml``."""
    key = entry.get("key", "<missing-key>")

    source_file = entry.get("source_file")
    if not source_file:
        raise ValueError(f"Example '{key}' is missing required field 'source_file'")

    source_path = EXAMPLES_DIR / source_file
    if not source_path.exists():
        raise ValueError(f"Example '{key}' source_file not found: {source_file}")

    expected_output = entry.get("expected_output")
    if not isinstance(expected_output, list):
        raise ValueError(f"Example '{key}' must define an 'expected_output' list")

    if not entry.get("skip_test", False) and not expected_output:
        raise ValueError(
            f"Example '{key}' has an empty 'expected_output': a runnable example "
            f"must assert something beyond 'did not crash'"
        )

    for i, item in enumerate(expected_output):
        if not isinstance(item, str) or not item.strip():
            raise ValueError(f"Example '{key}' has invalid expected_output[{i}]: {item!r}")

    files = entry.get("files", {})
    if not isinstance(files, dict):
        raise ValueError(f"Example '{key}' field 'files' must be a table/dict")

    for field_name, rel_path in files.items():
        if rel_path is None:
            continue
        if not (EXAMPLES_DIR / rel_path).exists():
            raise ValueError(
                f"Example '{key}' references missing file '{field_name}': {rel_path}"
            )


def _build_catalog() -> tuple[dict[str, Example], dict[str, Any], list[str]]:
    """Build the examples catalog, the categories info, and the category order."""
    manifest = _load_manifest()

    categories: dict[str, Any] = manifest.get("categories", {})
    category_order: list[str] = manifest.get("category_order", {}).get(
        "order", sorted(categories.keys())
    )

    catalog: dict[str, Example] = {}
    for entry in manifest.get("example", []):
        _validate_entry(entry)

        key = entry["key"]
        source_file = entry["source_file"]
        code = (EXAMPLES_DIR / source_file).read_text(encoding="utf-8")

        catalog[key] = Example(
            code=code,
            files=entry.get("files", {}),
            description=entry["description"],
            category=entry["category"],
            title=entry.get("title", key.replace("_", " ").title()),
            source_file=source_file,
            expected_output=entry.get("expected_output", []),
            skip_test=entry.get("skip_test", False),
        )

    return catalog, categories, category_order


EXAMPLES_CATALOG, CATEGORIES_INFO, CATEGORY_ORDER = _build_catalog()


def get_examples_by_category(category: str) -> dict[str, Example]:
    """Get all examples of a specific category."""
    return {k: v for k, v in EXAMPLES_CATALOG.items() if v["category"] == category}


def get_shex_examples() -> dict[str, Example]:
    return get_examples_by_category("shex")


def get_shacl_examples() -> dict[str, Example]:
    return get_examples_by_category("shacl")


def get_rdf_examples() -> dict[str, Example]:
    return get_examples_by_category("rdf")


def get_dctap_examples() -> dict[str, Example]:
    return get_examples_by_category("dctap")


def get_sparql_examples() -> dict[str, Example]:
    return get_examples_by_category("sparql")


def get_generate_examples() -> dict[str, Example]:
    return get_examples_by_category("generate")


def get_all_categories() -> list[str]:
    """Get category keys in the order defined in ``examples.toml``."""
    return list(CATEGORY_ORDER)


def get_category_info(category: str) -> dict[str, str]:
    """Return ``{title, description}`` for a category, with sensible defaults."""
    info = CATEGORIES_INFO.get(category, {})
    return {
        "title": info.get("title", category.upper()),
        "description": info.get("description", ""),
    }
