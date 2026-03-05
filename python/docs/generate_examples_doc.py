#!/usr/bin/env python3
"""
Generate examples.rst from examples.toml + .py source files.

The examples.toml manifest and the .py files in python/examples/ are the
single source of truth.  This script reads both (via examples_registry) and
produces structured RST documentation.

Usage:
    python generate_examples_doc.py --update   # Write examples.rst
    python generate_examples_doc.py --dry-run  # Preview without writing
    python generate_examples_doc.py --check    # Fail if examples.rst is stale
"""

import argparse
import sys
from pathlib import Path

# Add tests/ to sys.path so we can import examples_registry
_tests_dir = Path(__file__).resolve().parent.parent / "tests"
sys.path.insert(0, str(_tests_dir))

from examples_registry import (
    EXAMPLES_CATALOG,
    CATEGORY_ORDER,
    get_examples_by_category,
    get_category_info,
)

GITHUB_BASE = "https://github.com/rudof-project/rudof/blob/master/python/examples"


# ---------------------------------------------------------------------------
# RST helpers
# ---------------------------------------------------------------------------

def _code_block(code: str) -> list[str]:
    """Format Python code as an RST code-block."""
    lines = [".. code-block:: python", ""]
    for line in code.rstrip("\n").split("\n"):
        lines.append(f"    {line}")
    lines.append("")
    return lines


def _file_links(files: dict) -> list[str]:
    """Format referenced files as RST links."""
    lines = []
    for file_type, file_path in files.items():
        url = f"{GITHUB_BASE}/{file_path}"
        lines.append(f"- **{file_type.capitalize()}**: `{file_path} <{url}>`_")
    return lines


# ---------------------------------------------------------------------------
# Per-example and per-category RST generation
# ---------------------------------------------------------------------------

def _example_entry(example_id: str, data: dict) -> list[str]:
    """Generate RST for one example."""
    lines = []

    title = data.get("title", example_id.replace("_", " ").title())
    lines.append(title)
    lines.append("^" * len(title))
    lines.append("")
    lines.append(data["description"])
    lines.append("")

    source = data.get("source_file", "")
    if source:
        lines.append(f"**Source**: `{source} <{GITHUB_BASE}/{source}>`_")
        lines.append("")

    lines.append("**Python Code:**")
    lines.append("")
    lines.extend(_code_block(data["code"]))

    files = data.get("files", {})
    if files:
        lines.append("**Referenced Files:**")
        lines.append("")
        lines.extend(_file_links(files))
        lines.append("")

    lines.append("")
    return lines


def _category_section(category: str) -> list[str]:
    """Generate RST for one category (heading + all its examples)."""
    info = get_category_info(category)
    examples = get_examples_by_category(category)
    if not examples:
        return []

    title = info["title"]
    lines = [title, "-" * len(title), ""]
    for desc_line in info["description"].strip().splitlines():
        lines.append(desc_line)
    lines += ["", ""]

    for eid, edata in examples.items():
        lines.extend(_example_entry(eid, edata))

    return lines


# ---------------------------------------------------------------------------
# Full document
# ---------------------------------------------------------------------------

def generate_full_examples_rst() -> str:
    """Return the complete examples.rst content."""
    lines = [
        "Examples",
        "=" * 8,
        "",
        "This page contains validated Python examples that demonstrate pyrudof functionality.",
        "Each example includes executable Python code that can be copied and pasted into a",
        "Jupyter notebook or Python script, along with links to any referenced files.",
        "",
        "",
    ]

    for cat in CATEGORY_ORDER:
        lines.extend(_category_section(cat))

    return "\n".join(lines)


# ---------------------------------------------------------------------------
# CLI actions
# ---------------------------------------------------------------------------

def _stats(content: str) -> str:
    return (
        f"  - Total lines: {len(content.splitlines())}\n"
        f"  - Examples: {len(EXAMPLES_CATALOG)}\n"
        f"  - Categories: {len(CATEGORY_ORDER)}"
    )


def cmd_update():
    content = generate_full_examples_rst()
    rst_file = Path(__file__).parent / "examples.rst"
    rst_file.write_text(content, encoding="utf-8")
    print(f"Generated {rst_file.name}\n{_stats(content)}")


def cmd_dry_run():
    content = generate_full_examples_rst()
    print("=" * 60)
    print("DRY RUN — Preview of examples.rst")
    print("=" * 60)
    print(content[:3000])
    if len(content) > 3000:
        print("\n... (truncated) ...\n")
    print("=" * 60)
    print(_stats(content))


def cmd_check():
    """Exit non-zero if examples.rst is stale (useful in CI)."""
    rst_file = Path(__file__).parent / "examples.rst"
    if not rst_file.exists():
        print(f"FAIL: {rst_file.name} does not exist. Run with --update.")
        sys.exit(1)

    expected = generate_full_examples_rst()
    actual = rst_file.read_text(encoding="utf-8")

    if actual == expected:
        print(f"OK: {rst_file.name} is up to date.")
        sys.exit(0)
    else:
        print(
            f"FAIL: {rst_file.name} is stale. "
            f"Run `python {Path(__file__).name} --update` to regenerate."
        )
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description="Generate examples.rst from examples.toml + .py files"
    )
    parser.add_argument("--dry-run", action="store_true", help="Preview without writing")
    parser.add_argument("--update", action="store_true", help="Write examples.rst")
    parser.add_argument("--check", action="store_true", help="Fail if examples.rst is stale (for CI)")

    args = parser.parse_args()

    if args.check:
        cmd_check()
    elif args.dry_run:
        cmd_dry_run()
    elif args.update:
        cmd_update()
    else:
        parser.print_help()
        print("\nUse --update to generate, --dry-run to preview, or --check for CI.")


if __name__ == "__main__":
    main()
