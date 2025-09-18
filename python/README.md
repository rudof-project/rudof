# Rudof Python bindings

The Python bindings for [rudof](https://rudof-project.github.io/) are called `pyrudof`. They are available at [pypi](https://pypi.org/project/pyrudof/).

For more information, you can access the [readthedocs documentation](https://pyrudof.readthedocs.io/en/latest/).

After compiling and installing this module, a Python library  called `pyrudof` should be available.  

## Build the development version

This module is based on [pyo3](https://pyo3.rs/) and [maturin](https://www.maturin.rs/).

To build and install the development version of `pyrudof` you need to clone this git repository, go to the `python` directory (the one this README is in) and run:

```
pip install maturin
```

followed by:

```sh
pip install .
```

## Running the tests

Go to the tests folder: 

```sh
cd tests
```

and run: 

```sh
python3 -m unittest discover -vvv
```