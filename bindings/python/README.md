# pyrudof

`pyrudof` provides Python bindings for [rudof](https://rudof-project.github.io/rudof), a
Rust toolkit for the Semantic Web. It is a thin binding: every piece of Semantic Web logic
lives in the `rudof_lib` and `rudof_generate` crates, and this package's job is to move
values across the language boundary, turn Rust errors into Python exceptions, and present
the result as an API that feels like Python.

- [User documentation](https://pyrudof.readthedocs.io) - start here to *use* the library:
  API reference, guides and runnable examples.
- [Internals](https://rudof-project.github.io/rudof/internals/crates/python.html) -
  package layout and how the examples manifest drives both the
  tests and the documentation.

## Installation

Wheels are published on [PyPI](https://pypi.org/project/pyrudof/) and the package requires
Python 3.10 or newer:

```sh
pip install pyrudof
```

The wheels are `abi3-py310`, so a single wheel per platform serves every supported Python
version. The package has no Python runtime dependencies.

## Development

All commands below are run from `bindings/python/`. They are the same ones CI runs, in
`.github/workflows/python.yml`.

### Setting up

```sh
python -m venv .venv
source .venv/bin/activate          # Windows: .venv\Scripts\Activate.ps1

pip install -e '.[dev]'
```

`pip install -e` builds the extension through maturin, so the first run compiles the Rust
workspace and takes a while. Afterwards, rebuild just the extension with:

```sh
maturin develop                    # debug build, fast to compile
maturin develop --release          # optimised, what you want for benchmarks
```

### Testing

```sh
pytest                             # the whole suite
pytest -k shex                     # just the ShEx examples
pytest -x -q                       # stop at the first failure

mypy                               # strict type-check of python/, examples/ and tests/
```

### Regenerating what is generated

```sh
# 1. Type stubs, from the Rust annotations
cargo run --bin stub_gen --features stub-gen

# 2. The documentation's examples page, from examples.toml
python docs/generate_examples_doc.py --update
python docs/generate_examples_doc.py --check       # non-zero if stale
python docs/generate_examples_doc.py --dry-run     # preview without writing

# 3. The HTML documentation
python -m sphinx -b html docs docs/_build/html
```

`-W` to turn warnings into errors, and `-E` to force a full rebuild rather than an
incremental one:

```sh
python -m sphinx -b html -E -W docs docs/_build/html
python -m sphinxlint docs/*.rst
```

Then open `docs/_build/html/index.html`.

### Building wheels

```sh
maturin build --release            # wheel into target/wheels/
maturin sdist                      # source distribution
pip install --force-reinstall target/wheels/pyrudof-*.whl
```
