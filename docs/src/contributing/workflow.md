# Development workflow

How code gets from an issue into `master`.

## Branches

One branch per issue, named:

```text
<type>/<issue-number>-<short-slug>
```

For example `fix/827-rdf12-triple-terms`, `feat/849-nix-modules`, `docs/851-contribution-policies`.

The type is one of `feat`, `fix`, `docs`, `refactor`, `chore`, `test`, `ci`, `perf` (the same set
used in commit messages). The slug is two to four words in lower case, separated by hyphens.

Branches are short-lived: they exist from the moment work starts until the pull request is merged,
and a branch in this repository is deleted automatically on merge.

## Where the branch lives

Maintainers with push access branch in `rudof-project/rudof`. Everybody else works in a
[fork](https://github.com/rudof-project/rudof/fork): the same branch name, created in your own copy
of the repository, and the pull request is opened from `<your-user>:<branch>` against
`rudof-project:master`. Keep a remote pointing at this repository so you can refresh your fork
before branching:

```sh
git remote add upstream https://github.com/rudof-project/rudof
git fetch upstream
git switch -c fix/827-rdf12-triple-terms upstream/master
```

## Commits

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<scope>): <summary in the imperative, lower case, no trailing period>

<optional body explaining why, not what>

<optional footers>
```

```text
fix(shacl): interpolate message variables in sh:sparql constraints
docs(nix): regenerate module options reference
refactor(shex_ast): split the schema IR out of the parser
```

## Pull requests

**Title.** The pull request title follows Conventional Commits too, and matters more than the
individual commits: it becomes the squashed commit message, and the release notes are generated
from the titles of the merged pull requests.

**Description.** Fill in the template. It asks for:

- the issue, referenced as `Refs #123`, **never** `Closes #123` or `Fixes #123`.
  [Issue policy](./issues.md#lifecycle-and-closing). `Refs #` is also why GitHub shows no linked
  pull request on the issue: to get the link without the closing semantics, pick the issue in the
  *Development* field of the pull request sidebar.
- what changed and why, enough for a reviewer who has not read the issue
- how it was verified
- whether it breaks anything for users of the CLI, the library or the bindings

**Checklist.** Before asking for review:

- `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features --workspace -- -D warnings` are clean
- `cargo test --workspace --features qlever` passes
- `cargo check --workspace --target wasm32-unknown-unknown` is clean, if the change touches a crate
  that builds for WebAssembly
- new behaviour has a test
- user-visible changes have an entry under `## [Unreleased]` in `CHANGELOG.md` — see
  [Releases](./releases.md#the-changelog)
- user-visible changes update the manual under `docs/src/`
- the commits carry no AI co-authorship trailers — see [Use of AI](./ai.md)

Installing the [pre-commit](https://pre-commit.com/) hooks (`pre-commit install`) helps with that.

**Draft pull requests** are welcome and encouraged for work in progress: open one early to get
direction before the implementation is finished. Mark it ready for review when the checklist above
holds.

## Review and merge

Every pull request needs an approving review from a maintainer, and CI must be green. Reviews are
about correctness, scope and whether the change fits the architecture; formatting is CI's job, not
a reviewer's.

Pull requests are merged with **squash merge**, always. One pull request becomes one commit on
`master`, whose message is the pull request title. This keeps `master` bisectable and keeps the
generated release notes readable.

## Who can merge what

See [`GOVERNANCE.md`](https://github.com/rudof-project/rudof/blob/master/GOVERNANCE.md).
