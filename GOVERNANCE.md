# Governance

rudof is developed in the open by the [WESO](https://www.weso.es/) research group and its
contributors. This document says who decides what in rudof, so that the policies in
[CONTRIBUTING.md](./CONTRIBUTING.md) have someone behind them.

## Roles

### Contributor

Anyone who opens an issue, a pull request or a discussion. No permissions are needed and none are
granted. Contributors are expected to follow the [Code of Conduct](./CODE_OF_CONDUCT.md).

### Maintainer

Contributors with write access to the repository. Maintainers:

- triage incoming issues: type, labels, priority, milestone
- review and approve pull requests
- merge approved pull requests once CI is green
- decide what goes into a milestone

The current maintainers are the people with write access shown on the
[organisation members page](https://github.com/orgs/rudof-project/people). Maintainers are invited
by the project lead.

### Release manager

Maintainers who can run the `Release` workflow and publish to crates.io and npm. Publishing a
release follows the checklist in
[Releases](https://rudof-project.github.io/rudof/contributing/releases.html).

### Project lead

[@labra](https://github.com/labra) is the project lead. The lead sets the technical direction,
resolves disagreements that reviewers cannot settle among themselves, and decides who becomes a
maintainer.

## How decisions are made

Most decisions are made in the open, in the issue or pull request they concern, by the people
working on that area.

- **Ordinary changes** need one approving review from a maintainer who is not the author.
- **Changes to public APIs, the CLI surface, or the crate architecture** should be agreed in an
  issue before the code is written, and reviewed by a maintainer who works on that area. When in
  doubt, ask in the issue rather than in the pull request.
- **Disagreements** are settled by discussion. If that fails, the project lead decides.
- **Changes to these policies** (this file, `CONTRIBUTING.md`, the pages under
  `docs/src/contributing/`, the issue templates and `.github/labels.yml`) can be debated in the
  corresponding discussion and are handled as pull requests like any other. The policy for the
  internal board is not here: it lives in `rudof-project/weso_rudof`.

Nobody merges their own pull request, with one exception: a maintainer may self-merge a trivial
change and should say so in the pull request.

## Reviewers

[`.github/CODEOWNERS`](./.github/CODEOWNERS) maps parts of the repository to the people who know
them best, so that reviews are requested automatically. Being listed there means *you are a good
person to ask*, not that you own the code or that your approval is the only one that counts.
