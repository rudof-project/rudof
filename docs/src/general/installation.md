# Installation

### Official releases

You can download a binary from the [latest release](https://github.com/rudof-project/rudof/releases/latest) page. There you will also find the compiled packages for the installation on your system using a package manager.

#### Ubuntu

Download the binary from [https://github.com/rudof-project/rudof/releases] and install the `.deb` package running the following commands after replacing X.X.X by the latest version:

```sh
wget https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_amd64.deb
sudo dpkg -i rudof_vX.X.X_amd64.deb
```

#### Windows

The binary can be downloaded from [releases](https://github.com/rudof-project/rudof/releases)

#### Mac

The binary is available at: [releases](https://github.com/rudof-project/rudof/releases) so you can download the corresponding binary to your machine.

The usual way to run/install a binary in Mac is to download it in a folder, add that folder to your PATH and activating the binary using:

```sh
chmod +x <binary_file>
```

After that, I think the processor may complain the first time about security and you have to agree to use it...once you agree, it should work.

### Compiling from source

`rudof` has been implemented in Rust and is compiled using [cargo](https://doc.rust-lang.org/cargo/). The command `cargo run` can be used to compile and run locally the code.

For example:

```sh
cargo run -- validate --data examples/user.ttl --schema examples/user.shex --shapemap examples/user.sm 
```

### Compiling from source and installing the binary (Debian)

Install `cargo deb` (only the first time)

```sh
cargo install cargo-deb
```

Create the `.deb` package by:

```sh
cargo deb
```

And run:

```sh
sudo dpkg -i target/debian/rudof_0.0.11-1_amd64.deb
```

## Docker

The library is also published as a Docker image.
