# Installation and Building

You can download a binary from the [download page](https://github.com/weso/pgschemapc/releases), where you will also find the compiled packages for the installation on your system using a package manager.

## Building

The project has been implemented in Rust and uses [cargo](https://doc.rust-lang.org/cargo/) for building. Once you install `cargo`, you can run:

```
cargo build
```

to compile and generate a binary which will be available in `target/debug/pgschemapc`. If you want a more performant binary, you can use the `--release` option.
