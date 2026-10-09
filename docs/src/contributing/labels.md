# Label reference

Labels answer *where* an issue belongs and *how urgent* it is. They deliberately do **not** say
what kind of work it is, that is the [issue type](./issues.md#issue-types).

Every label belongs to a namespace written as a prefix. The machine-readable source of truth is
[`.github/labels.yml`](https://github.com/rudof-project/rudof/blob/master/.github/labels.yml).

| Namespace | Meaning | How many per issue |
| --- | --- | --- |
| `area/` | Which part of rudof is affected | At least one |
| `platform/` | Binding, operating system or packaging target | Zero or more |
| `spec/` | Specification version, cutting across areas | Zero or more |
| `status/` | Where the issue is in its lifecycle | At most one |
| `priority/` | How urgent it is | At most one |

Two labels stay unprefixed on purpose: GitHub matches the exact names `help wanted` and
`good first issue` in its contributor discovery pages, so renaming them would hide the issues from
people looking for somewhere to start.

## `area/` — what is affected

Pick every area the change touches; a pull request that fixes a ShEx bug in the CLI carries both `area/shex` and `area/cli`.

| Label | Use it for |
| --- | --- |
| `area/shex` | ShEx: schemas, validation, the ShEx AST |
| `area/shacl` | SHACL support and the validation algorithm |
| `area/rdf` | RDF parsing, serialization and the data model |
| `area/sparql` | SPARQL queries and result formats |
| `area/dctap` | DCTAP support |
| `area/pg` | Property graphs: the data model, parsing and serialization |
| `area/pgschema` | PG-Schema: schemas and property constraints over property graphs |
| `area/rdf-config` | rdf-config files from DBCLS |
| `area/generate` | The synthetic RDF data generator |
| `area/cli` | The `rudof` command line interface |
| `area/shell` | The interactive shell |
| `area/mcp` | The Model Context Protocol server |
| `area/backend` | Storage backends |
| `area/architecture` | Crate layout, public APIs, cross-cutting design |
| `area/docs` | The manual, the README, doc comments and examples |
| `area/tests` | Test suites, test infrastructure, submodules |
| `area/ci` | GitHub Actions workflows and CI infrastructure |
| `area/release` | Releases, packaging and distribution |
| `area/dependencies` | Dependency updates and version alignment |
| `area/benchmarks` | Benchmarks and performance measurement |

`area/docs` marks the documentation itself, the same way `area/cli` marks the command line
interface. It is not a substitute for the `Documentation` type: the type says the work *is*
writing documentation, the label says the manual is what the work touches. A page that documents
ShEx carries `area/docs` and `area/shex`.

## `platform/` — where it happens

Only when the issue is specific to that target. A ShEx bug that happens everywhere carries no
platform label; the same bug that only shows up in the Python bindings carries `platform/python`.

`platform/python`, `platform/wasm`, `platform/windows`, `platform/linux`, `platform/macos`,
`platform/nix`, `platform/docker`.

## `spec/` — specification version

`spec/rdf-1.2` and `spec/shacl-1.2` mark work that exists because a specification version requires
it. They cut across areas: RDF 1.2 triple terms affect `area/rdf`, `area/shex` and `area/shacl`
alike, and grouping that work is easier with a label than with a milestone.

## `status/` — where it is in its lifecycle

At most one at a time; they are mutually exclusive by construction.

| Label | Meaning | Who sets it |
| --- | --- | --- |
| `status/needs-triage` | Not yet reviewed by a maintainer | The issue templates, automatically |
| `status/needs-info` | Waiting for the reporter | Triager |
| `status/blocked` | Cannot progress until something else resolves | Assignee |
| `status/pending-release` | Merged into `master`, not yet published | Automatically, when the pull request merges |
| `status/duplicate` | Already tracked elsewhere — link the original | Triager, before closing |
| `status/invalid` | Not an actionable report | Triager, before closing |
| `status/wontfix` | Deliberately not going to be done — say why | Maintainer, before closing |

An issue with no `status/` label has been triaged and is simply waiting for someone to pick it up.

## `priority/` — how urgent it is

Priority is a maintainer's judgement about scheduling, not a promise of a date. Only maintainers
set it, during triage.

| Label | Meaning |
| --- | --- |
| `priority/p0-critical` | Data loss, a security issue, or `master` is broken. Drop everything |
| `priority/p1-high` | Blocks real usage with no workaround. Goes into the next release |
| `priority/p2-medium` | Should be done and has a workaround. Scheduled when there is room |
| `priority/p3-low` | Nice to have. No committed timeframe |

Most of the backlog carries no priority label at all, and that is fine: it means *not yet
scheduled*. Do not label everything `p2` to make the backlog look tidy.

## Changing the taxonomy

Adding a label is a pull request that edits `.github/labels.yml` and this page in the same commit.
Before adding an `area/`, check that the work does not fit an existing one.
