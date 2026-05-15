# Installation

## Official releases

You can download a binary from the [download page](https://github.com/rudof-project/rudof/releases/latest), where you will also find the compiled packages for the installation on your system using a package manager.

### Linux

In case you want to install the pre-compiled versions of `rudof` for Linux, you can take a look at the `rudof_vX.X.X_x86_64_linux_gnu` executable, where `vX.X.X` corresponds to the version to be installed.
By executing the following instructions you can get the pre-compiled binaries of the tool.

> Remember to change the `vX.X.X` thing to the corresponding version.

```sh
curl -o rudof https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_x86_64_linux_gnu
chmod +x rudof
```

#### Nix

`rudof` ships a [`flake.nix`](https://github.com/rudof-project/rudof/blob/master/flake.nix) that exposes packages, a development shell, and an overlay so it can be consumed by other Nix projects without any extra work.

##### Prerequisites

You need [Nix](https://nixos.org/download) with the `flakes` and `nix-command` experimental features enabled.

##### Try it without installing

You can run `rudof` directly from the flake without permanently installing anything:

```shell
nix run github:rudof-project/rudof#rudof -- <rudof-flags>
```

For example, to print the help message:

```shell
nix run github:rudof-project/rudof#rudof -- --help
```

##### Install into your profile

To install `rudof` persistently into your user profile, run:

```shell
nix profile install github:rudof-project/rudof#rudof
```

After this, `rudof` will be available in your `$PATH` for the current user.

To later upgrade to a newer version, run:

```shell
nix profile upgrade '.*rudof.*'
```

##### Available packages

The flake exposes the following packages:

| Package          | Description               |
|------------------|---------------------------|
| `rudof`          | The main CLI binary       |
| `rudof-generate` | Code-generation utilities |

You can list all available outputs with:

```shell
nix flake show github:rudof-project/rudof
```

##### Using the flake as an input in your project

The recommended way to consume `rudof` from another Nix project is by adding the flake as an input and applying the provided overlay.
The overlay injects all `rudof` packages into `pkgs` so they can be referenced as `pkgs.rudof.*`.

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rudof.url = "github:rudof-project/rudof";
    rudof.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rudof, ... }:
    let
      system = "your-system-architecture";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rudof.overlays.default
        ];
      };
    in
    {
      # Your outputs here
    };
}
```

Once the overlay is applied you can reference `rudof` tools anywhere in your configuration:

```nix
{ pkgs, ... }:
{
  environment.systemPackages = with pkgs.rudof; [
    rudof
    rudof-generate
  ];
}
```

##### NixOS / home-manager module

> Support for a dedicated NixOS and home-manager module is planned.


### Windows

You can download the Windows binary from the [releases](https://github.com/rudof-project/rudof/releases/latest) page.

As in the case of Linux, the name of the file will be something like `rudof_vX.X.X_x86_64_windows_msvc.exe`, where `vX.X.X` corresponds to the version to be installed.
In systems whose version is Windows 10 and above, one can run the following snippet:

> Remember to change the `vX.X.X` thing to the corresponding version.

```sh
curl -o rudof.exe https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_x86_64_windows_msvc.exe
```

### Mac

As in the two previous cases, the MacOS binaries are available at the [download page](https://github.com/rudof-project/rudof/releases/latest).

However, in the case of this operating system, two different executables are provided; namely, `rudof_vX.X.X_x86_64_apple` for Intel-based machines, and `rudof_vX.X.X_aarch64_apple` for the new M chips.

> Remember to change the `vX.X.X` thing to the corresponding version, and the `<<platform>>` tag to the corresponding platform.

```sh
curl -o rudof https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_<<platform>>_apple.exe
```

Once downloaded, you have to change the permissions of the file as:

```sh
chmod +x rudof_vX.X.X_<<platform>>_apple
```

## Using a Package Manager

### Debian

`rudof` is also bundled as a Debian package that is available in the [download page](https://github.com/rudof-project/rudof/releases/latest).
To obtain it you can follow the following steps, which are similar to the ones described in the previous sections.

> Remember to change the `vX.X.X` thing to the corresponding version.

```sh
curl -o rudof.deb https://github.com/rudof-project/rudof/releases/download/vX.X.X/rudof_vX.X.X_amd64.deb
sudo dpkg -i rudof.deb
```

## Compiling from source

Another alternative is to build the binaries on your own using [`cargo`](https://doc.rust-lang.org/cargo/), as `rudof` has been implemented in Rust.
To do so, you just have to clone the [Github repository](https://github.com/rudof-project/rudof) and build it using the appropiate command.
The workflow could be as follows:

> If you want to get the most efficient binary, at the cost of a longer compile time, you can pass the `--release` flag to the last command.

```sh
git clone https://github.com/rudof-project/rudof.git
cd rudof
cargo build
```

### Creating your own Debian package

By installing the `cargo-deb` utility you can follow the same steps as defined in the [Debian](#debian) section.
To do so, you just have to follow the steps below:

```sh
cargo install cargo-deb
```

Once you have it installed, you can call the `cargo deb` command to compile the project to a Debian project which can be installed using the [`dpkg`](https://man7.org/linux/man-pages/man1/dpkg.1.html) package manager.

> Remember to change the `X.X.X` thing to the corresponding version.

```sh
cargo deb
sudo dpkg -i target/debian/rudof_X.X.X_amd64.deb
```

## Docker

TBD
