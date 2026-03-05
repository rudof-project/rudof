"""
Registry of documented examples loaded from examples.toml and the .py source files.
"""

import tomllib
from pathlib import Path
from typing import TypedDict, Optional, List


class ExampleFiles(TypedDict, total=False):
    """Files associated with an example (relative to python/examples/)."""
    schema: str
    data: str
    shapemap: Optional[str]
    query: Optional[str]
    config: Optional[str]


class Example(TypedDict):
    """Metadata for a single example."""
    code: str              # Read from the .py file at load time
    files: ExampleFiles
    description: str
    category: str
    title: str
    source_file: str
    expected_output: list[str]
    skip_test: bool        # True → document but do not run in test suite


# ---------------------------------------------------------------------------
# Locate the examples directory (works from python/tests/ and python/docs/)
# ---------------------------------------------------------------------------
_EXAMPLES_DIR = Path(__file__).resolve().parent.parent / "examples"
_MANIFEST_PATH = _EXAMPLES_DIR / "examples.toml"


def _load_manifest() -> dict:
    """Parse examples.toml and return the raw TOML dict."""
    with open(_MANIFEST_PATH, "rb") as f:
        return tomllib.load(f)


def _build_catalog() -> tuple[dict[str, Example], dict, list[str]]:
    """Build the examples catalog, categories info, and category order."""
    manifest = _load_manifest()

    categories = manifest.get("categories", {})
    category_order = manifest.get("category_order", {}).get("order", sorted(categories.keys()))

    catalog: dict[str, Example] = {}
    for entry in manifest.get("example", []):
        key = entry["key"]
        source_file = entry["source_file"]
        code_path = _EXAMPLES_DIR / source_file
        code = code_path.read_text(encoding="utf-8")

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


# ---------------------------------------------------------------------------
# Helper functions
# ---------------------------------------------------------------------------

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


def get_all_categories() -> List[str]:
    """Get category keys in the order defined in examples.toml."""
    return list(CATEGORY_ORDER)


def get_category_info(category: str) -> dict:
    """Return {title, description} for a category, with sensible defaults."""
    info = CATEGORIES_INFO.get(category, {})
    return {
        "title": info.get("title", category.upper()),
        "description": info.get("description", ""),
    }

