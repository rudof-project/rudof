# Contributing to rudof

Thanks for wanting to help. rudof is a semantic-less tool for the Semantic Web written in Rust,
and contributions of every size are welcome.

This page gets you started. The policies it links to explain how the project is run.

| If you want to know | Read |
| --- | --- |
| How work is organised: types, labels, priorities, milestones, the board | [Issue policy](https://rudof-project.github.io/rudof/contributing/issues.html) |
| Which label to put on something | [Label reference](https://rudof-project.github.io/rudof/contributing/labels.html) |
| How to name branches, write commits and get a pull request merged | [Development workflow](https://rudof-project.github.io/rudof/contributing/workflow.html) |
| When your change will actually ship | [Releases](https://rudof-project.github.io/rudof/contributing/releases.html) |
| Whether you can use AI assistants | [Use of AI](https://rudof-project.github.io/rudof/contributing/ai.html) |
| Who decides what | [GOVERNANCE.md](./GOVERNANCE.md) |
| How to behave | [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) |
| How to report a vulnerability | [SECURITY.md](./SECURITY.md) |

The same pages live under `docs/src/contributing/` in this repository.

## Getting set up

You need a recent stable [Rust toolchain](https://rustup.rs/).

Unless you are a maintainer with push access,
[fork the repository](https://github.com/rudof-project/rudof/fork) first and clone your fork:

```sh
git clone --recurse-submodules https://github.com/<your-user>/rudof
cd rudof
git remote add upstream https://github.com/rudof-project/rudof
cargo build --release
cargo test --workspace
```

Maintainers clone `rudof-project/rudof` directly and need no `upstream` remote.

The test suites for ShEx and SHACL are git submodules; if you cloned without `--recurse-submodules`,
run `git submodule update --init --recursive`.

Two extras that CI installs and the full test run needs: [Graphviz](https://graphviz.org/download/),
for the tests that render diagrams, and the WebAssembly target, if you touch a crate that builds
for it:

```sh
rustup target add wasm32-unknown-unknown
```

Install the [pre-commit](https://pre-commit.com/) hooks so formatting and linting run before each
commit, it is the same set of checks CI runs:

```sh
pre-commit install
```

## The short version of the process

1. **Find or open an issue.** Every change starts as an issue, using one of the
   [templates](https://github.com/rudof-project/rudof/issues/new/choose). The exception is trivial
   changes which can go straight to a pull request.
   Looking for somewhere to start? Try the [`good first issue`](https://github.com/rudof-project/rudof/labels/good%20first%20issue) label.
2. **Say you are taking it.** Comment on the issue so nobody duplicates your work, and wait for a
   maintainer to confirm the approach before writing a large change.
3. **Branch** off `master` as `<type>/<issue-number>-<slug>`, for example
   `fix/827-rdf12-triple-terms`. On a fork that branch lives in your own copy.
4. **Commit** using [Conventional Commits](https://www.conventionalcommits.org/), with no AI
   co-authorship trailers.
5. **Open a pull request** whose title also follows Conventional Commits, referencing the issue as
   `Refs #123` (not `Closes #123`). From a fork, open it against `rudof-project:master`. Add a
   `CHANGELOG.md` entry under `## [Unreleased]` if the change is visible to users.

Your issue stays open after the pull request is merged and is closed when the release that contains
your change is published. That is deliberate: the problem is not solved until people can install a
version without it.

## Questions

Questions about using rudof belong in
[Discussions](https://github.com/rudof-project/rudof/discussions) or the
[FAQ](https://github.com/rudof-project/rudof/wiki/FAQ), not in issues.

## Licensing

By contributing you agree that your contribution is dual-licensed under the MIT and Apache-2.0
licences, as described in the [README](./README.md#contribution).
