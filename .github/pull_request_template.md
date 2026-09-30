<!--
Thanks for contributing to rudof!

The title of this pull request becomes the commit message on master and shows up in
the release notes, so please write it as a Conventional Commit:

    fix(shacl): interpolate message variables in sh:sparql constraints

Full guide: https://rudof-project.github.io/rudof/contributing/workflow.html
-->

## Related issue

Refs #

<!--
Use `Refs #123`, NOT `Closes #123` or `Fixes #123`.

Issues in rudof are closed when the release containing the fix is published, not when
the pull request is merged. See https://rudof-project.github.io/rudof/contributing/issues.html#lifecycle-and-closing

For a trivial change with no issue, say so here instead.
-->

## What this changes

<!-- What the change does and why, for a reviewer who has not read the issue. -->

## How it was verified

<!-- Tests added, commands run, schemas and data used. -->

## Impact on users

<!--
Does this change behaviour of the CLI, the library API, or the bindings?
Mark breaking changes with `!` in the title (feat(rudof_lib)!: ...) and explain them here.
Write "None" if this is internal only.
-->

## Checklist

- [ ] `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features --workspace -- -D warnings` are clean
- [ ] `cargo test --workspace --features qlever` passes
- [ ] If the change touches a crate that builds for WebAssembly: `cargo check --workspace --target wasm32-unknown-unknown` is clean
- [ ] New behaviour has a test
- [ ] User-visible changes have an entry under `## [Unreleased]` in `CHANGELOG.md`
- [ ] User-visible changes are documented under `docs/src/`
- [ ] The title follows Conventional Commits
- [ ] The commits carry no AI co-authorship trailers ([why](https://rudof-project.github.io/rudof/contributing/ai.html))
- [ ] I can explain every line of this pull request in review
