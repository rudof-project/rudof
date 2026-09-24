"""
Run every example in the manifest and check its recorded output.
"""

import runpy

import pytest

from examples._registry import EXAMPLES_CATALOG, EXAMPLES_DIR

RUNNABLE = {k: e for k, e in EXAMPLES_CATALOG.items() if not e["skip_test"]}
SKIPPED = {k: e for k, e in EXAMPLES_CATALOG.items() if e["skip_test"]}


@pytest.mark.parametrize("key", sorted(RUNNABLE), ids=sorted(RUNNABLE))
def test_example(
    key: str,
    capsys: pytest.CaptureFixture[str],
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    example = RUNNABLE[key]
    monkeypatch.chdir(EXAMPLES_DIR)

    runpy.run_path(str(EXAMPLES_DIR / example["source_file"]), run_name="__main__")

    out = capsys.readouterr().out
    for expected in example["expected_output"]:
        assert expected in out, f"{key}: missing {expected!r}\n--- actual ---\n{out}"


@pytest.mark.parametrize("key", sorted(SKIPPED), ids=sorted(SKIPPED) or ["none"])
def test_skipped_example_is_still_importable(key: str) -> None:
    """A skipped example is not run, but it must still parse and define ``main``."""
    source = EXAMPLES_DIR / SKIPPED[key]["source_file"]
    compile(source.read_text(encoding="utf-8"), str(source), "exec")
    assert "def main()" in source.read_text(encoding="utf-8")


def test_every_example_file_is_in_the_manifest() -> None:
    """An example script that nobody registered is an example nobody tests."""
    on_disk = {
        str(p.relative_to(EXAMPLES_DIR))
        for p in EXAMPLES_DIR.rglob("*.py")
        if not p.name.startswith("_")
    }
    registered = {e["source_file"] for e in EXAMPLES_CATALOG.values()}
    assert on_disk == registered
