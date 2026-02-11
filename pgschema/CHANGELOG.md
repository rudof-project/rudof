# ChangeLog

This ChangeLog tries to follow the Keep a ChangeLog guidelines](https://keepachangelog.com/).

## [Unreleased]
### Fixed
- Refactor to avoid repeating mod declarations in main.rs

## v0.1.6

### Added
- Support for edges in property graphs and property graph schemas
- Added test: edge, datatypes
- Added built-in datatypes Bool and Date

### Fixed
- Rules for CREATE GRAPH TYPE in Grammar which were empty

### Changed
- Improved error messages in test_suite runner using expect to show the name of the failing file when there is a parsing error

### Removed

## v0.1.5

### Added
- Support for edges in property graphs and property graph schemas
- Added tests: edge

### Fixed
### Changed
### Removed

## v0.1.4

### Added
- Included a folder with a test-suite
- Automatic run of tests for all examples using continuous integration
- Check for conformance of results to expected results

### Fixed
- Fixed error in Regex checker that included quotes

### Changed

### Removed

## v0.1.3
- Improved visualization of errors
- Small change in PGS grammar to avoid ambiguity when parsing OneOf record types
- Extended property graph grammar to parse edges with ids between square brackets

## v0.1.2

- Added property graphs and association maps to the examples from the paper
- Added github action workflows to automatically publish binaries and run continuous integration tests
- Added status badges

## v0.1.1

Added command line options to validate and show information about property graphs (pg), property graphs schemas (pgs) and type maps (map)
