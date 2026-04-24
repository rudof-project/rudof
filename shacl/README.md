# SHACL

[SHACL](https://www.w3.org/TR/shacl/) implementation in Rust.

This project started as a re-implementation in Rust of [SHACL-s](https://github.com/weso/shacl-s).

## Configuration for tests

The tests depend on the [shacl-testsuite](https://w3c.github.io/data-shapes/data-shapes-test-suite/) which is available as a git submodule. In order to run tests it is necessary to run in the root folder of rudof:

```sh
 git submodule update --init --recursive 
```



