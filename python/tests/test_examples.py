"""
Test suite for documented examples from python/examples/.

Tests are auto-generated from examples.toml.  Each example's .py file is
executed as a subprocess so it runs exactly as a user would run it.

Run all tests (skipped examples are skipped by default):
    python -m unittest test_examples -v

Run ALL examples including those marked skip_test=true:
    RUN_SKIPPED_EXAMPLES=1 python -m unittest test_examples -v          # Linux/macOS
    $env:RUN_SKIPPED_EXAMPLES="1"; python -m unittest test_examples -v  # PowerShell
"""

import os
import subprocess
import sys
import unittest
from pathlib import Path

from examples_registry import EXAMPLES_CATALOG, get_all_categories, get_examples_by_category


# Resolve once: python/tests/ -> python/ -> python/examples/
_EXAMPLES_DIR = Path(__file__).resolve().parent.parent / "examples"

# Ensure subprocesses use UTF-8 (avoids cp1252 errors on Windows)
_ENV = {**os.environ, "PYTHONUTF8": "1"}

# When set to a truthy value, tests marked skip_test=true will run anyway.
_RUN_SKIPPED = os.environ.get("RUN_SKIPPED_EXAMPLES", "").strip() not in ("", "0")


def _make_test(example_key: str):
    """Create a test method for a single example."""

    def test_method(self):
        example = EXAMPLES_CATALOG[example_key]

        if example.get("skip_test", False) and not _RUN_SKIPPED:
            self.skipTest(f"skip_test=true for '{example_key}' (set RUN_SKIPPED_EXAMPLES=1 to run)")

        source_file = example["source_file"]
        expected_output = example.get("expected_output", [])

        result = subprocess.run(
            [sys.executable, source_file],
            cwd=str(_EXAMPLES_DIR),
            capture_output=True,
            text=True,
            timeout=60,
            env=_ENV,
        )

        self.assertEqual(
            result.returncode, 0,
            f"Example '{example_key}' ({source_file}) failed:\n"
            f"--- stdout ---\n{result.stdout}\n"
            f"--- stderr ---\n{result.stderr}",
        )

        # Non-empty output expected from every example
        self.assertTrue(
            len(result.stdout.strip()) > 0,
            f"Example '{example_key}' produced no output",
        )

        # Check expected substrings if any
        for expected in expected_output:
            self.assertIn(
                expected, result.stdout,
                f"Example '{example_key}': expected '{expected}' in output",
            )

    test_method.__doc__ = f"Test example: {example_key}"
    return test_method


def _build_test_classes():
    """Dynamically create one TestCase class per category."""
    for category in get_all_categories():
        examples = get_examples_by_category(category)
        if not examples:
            continue

        class_name = f"Test{category.capitalize()}Examples"
        attrs: dict = {}

        for key in examples:
            method_name = f"test_{key}"
            attrs[method_name] = _make_test(key)

        cls = type(class_name, (unittest.TestCase,), attrs)
        # Register in module globals so unittest discovery finds them
        globals()[class_name] = cls


_build_test_classes()


if __name__ == "__main__":
    unittest.main(verbosity=2)

