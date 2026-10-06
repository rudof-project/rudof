# Issue policy

Every change to rudof starts as an issue. The issue is where the problem is agreed on; the pull
request is only where the solution is reviewed.

**Exception for trivial changes.** Trivial fixes can go straight to a pull request, there the pull request *is* the issue. Anything that changes behaviour, adds a feature or restructures code needs an issue first.

## Opening an issue

Use one of the [templates](https://github.com/rudof-project/rudof/issues/new/choose). Blank issues
are disabled: the templates ask for the things that would otherwise be asked in the first reply.

Questions about how to use rudof are not issues. They belong in
[Discussions](https://github.com/rudof-project/rudof/discussions), and the
[FAQ](https://github.com/rudof-project/rudof/wiki/FAQ) may already answer them. Security
vulnerabilities are not issues either — see
[`SECURITY.md`](https://github.com/rudof-project/rudof/blob/master/SECURITY.md).

Every new issue arrives with `status/needs-triage`.

## Issue types

The *type* says what kind of work it is.

| Type | Use it for |
| --- | --- |
| `Bug` | Something behaves differently from what is documented or specified |
| `Feature` | New capability, or an extension of an existing one |
| `Documentation` | The manual, the README, doc comments, examples |
| `Refactor` | Internal change with no observable difference in behaviour |
| `Task` | Everything else: CI, dependencies, releases, housekeeping |

Exactly one type per issue, set by the template and corrected during triage if needed.

## Labels

Labels say *where* the issue belongs and *how urgent* it is. The full catalogue is in the [label reference](./labels.md). In short:

- `area/*` — at least one, and as many as the change touches
- `platform/*`, `spec/*` — only when the issue is specific to that target or specification
- `status/*` — at most one, tracking the lifecycle
- `priority/*` — at most one, set by maintainers only

## Triage

Maintainers triage the incoming queue (the `status/needs-triage` view of the project board).
Triaging an issue means:

1. Confirm it is reproducible and not a duplicate. If it duplicates another issue, link the
   original, apply `status/duplicate` and close.
2. Set the type if the template got it wrong.
3. Apply `area/*` and any `platform/*` or `spec/*` labels.
4. Decide a priority, or leave it unset if it is not scheduled work.
5. Add it to a milestone **only** if it is committed to a specific release.
6. Remove `status/needs-triage`.

If information is missing, ask and apply `status/needs-info`.

## Milestones

One milestone per released version, named exactly as the version: `0.3.25`, `0.4.0`.

A milestone means *committed to this release*, not *would be nice by then*. The backlog lives in
the project board.

- The milestone for the next version is created when the previous one closes.
- Issues get a milestone when work on them is actually scheduled, usually at triage or when
  someone picks them up.
- Publishing the release closes the issues it ships; closing the milestone is the last step of
  publishing it. See [Releases](./releases.md).
- If the release ships without an issue that was in its milestone, move the issue to the next
  milestone before closing — never close it as done.

## The project board

All issues live in a single [project](https://github.com/orgs/rudof-project/projects), which
provides several views over the same set rather than splitting the backlog across boards.

The board follows a kanban flow:

```text
Triage → Backlog → Ready → In progress → In review → Merged → Released
```

| Column | Means |
| --- | --- |
| `Triage` | Just arrived, carries `status/needs-triage` |
| `Backlog` | Triaged and accepted, not scheduled |
| `Ready` | Specified well enough that someone can start today |
| `In progress` | Someone is working on it; the issue is assigned and a branch exists |
| `In review` | A pull request is open |
| `Merged` | Merged into `master`, carries `status/pending-release` |
| `Released` | Published; the issue is closed |

Besides the board there are views by priority, grouped by milestone, filtered by area, and the
triage queue.

Items move on their own, driven by
[`project-board.yml`](https://github.com/rudof-project/rudof/blob/master/.github/workflows/project-board.yml):
new issues land in `Triage`, and the `Refs #` lines of a pull request move the issues they name to
`In progress` while it is a draft, `In review` once it is ready for review, and `Merged` when it is
merged. GitHub's own board automations are not used, because they key on the linked pull request
that `Refs #` deliberately never creates.

## Lifecycle and closing

An issue is **closed when the fix is released, not when the pull request is merged.** The reason is
that an issue reports a problem a user has, and that problem is only gone once the user can install
a version without it.

This has one practical consequence for every contributor:

> Pull requests must reference issues with `Refs #123`, never `Closes #123` or `Fixes #123`.

The full path of an issue:

1. Opened → `status/needs-triage`, lands in `Triage`.
2. Triaged → type, `area/*` and maybe `priority/*`; moves to `Backlog` or `Ready`.
3. Picked up → assigned, moved to `In progress`, a branch is created following the
   [development workflow](./workflow.md).
4. Pull request opened → `In review`.
5. Pull request merged → `status/pending-release`, `Merged`, and the issue is assigned the
   milestone of the version it will ship in. **It stays open.**
6. Release published →
   [`project-release.yml`](https://github.com/rudof-project/rudof/blob/master/.github/workflows/project-release.yml)
   closes every issue sitting in `Merged`, removes `status/pending-release` and moves it to
   `Released`. A release candidate publishes nothing, so it leaves the column alone.

Issues can also be closed without a release, as `status/duplicate`, `status/invalid` or
`status/wontfix`. Always say why in a comment.
