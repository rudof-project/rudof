# Security policy

## Supported versions

Only the most recently published version of rudof is supported. Fixes are not backported: a
security fix ships in the next release, and upgrading is the way to get it.

| Version | Supported |
| --- | --- |
| Latest release | Yes |
| Anything older | No |

This applies to every artifact built from this repository: the `rudof` binaries, the crates on
crates.io, the `@rudof/rudof` npm package, the Python bindings and the Docker images.

## Reporting a vulnerability

**Do not open a public issue for a security problem.**

Report it through GitHub's private vulnerability reporting, from the
[Security tab](https://github.com/rudof-project/rudof/security/advisories/new) of this repository.
That opens a private advisory visible only to you and the maintainers.

A useful report includes:

- the version of rudof (`rudof --version`) and how it was installed
- which interface is affected: the CLI, the library, the Python or WebAssembly bindings, the MCP
  server
- what an attacker can achieve, and what they need in order to do it
- the smallest input that demonstrates the problem

## What happens next

- We will tell you whether we consider it a vulnerability, and why.
- Confirmed vulnerabilities are fixed on a private branch and published in the next release,
  together with a GitHub Security Advisory.
- You will be credited in the advisory unless you would rather not be.

Please give us a reasonable chance to release a fix before disclosing the problem publicly.

## Scope

rudof processes untrusted input by design: RDF data, ShEx and SHACL schemas, SPARQL queries and
configuration files, from files, from URLs and from remote endpoints. Problems in that processing
are in scope.

Out of scope: vulnerabilities in dependencies that we cannot influence (report them upstream, and
tell us so we can update), and findings from automated scanners with no demonstrated impact.
