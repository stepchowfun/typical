# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2022-01-03

### Changed
- The encoding has changed. In particular, two of the size modes have swapped meaning.
- Generated Rust deserialization code is now guaranteed to consume the entire buffer.
- Generated Rust code is now slightly more compact.
- The help entries now use a consistent verb tense.

## [0.4.0] - 2022-01-02

### Added
- Added support for TypeScript code generation.

## [0.3.0] - 2022-01-01

### Fixed
- Fixed a bug in the Rust code generator that would cause invalid code to be generated if the schema refers to (possibly nested) arrays of user-defined types.

### Changed
- Changed the way `S64` is encoded to enable more straightforward code generation. This change only affects values greater than or equal to `567,382,630,219,904`.

## [0.2.2] - 2021-12-22

### Fixed
- Fixed a typo in a log message.

## [0.2.1] - 2021-12-16

### Changed
- Slightly improved the `format` subcommand behavior such that it is now always possible to have a top-level comment. Previously, in some situations, a comment intended to be a top-level comment would be interpreted as a comment on the first declaration.

## [0.2.0] - 2021-12-16

### Added
- Added the `format` subcommand.

## [0.1.2] - 2021-12-07

### Added
- Added the `list-schemas` flag to the `generate` subcommand.

## [0.1.1] - 2021-12-06

### Fixed
- Fixed a bug that would prevent the generated Rust code from compiling for schemas that contain anything of type `[U64]`. The bug was discovered by a newly expanded integration test suite.

## [0.1.0] - 2021-11-08

### Changed
- Renamed `--rust-out` to `--rust` and `--typescript-out` to `--typescript`.

## [0.0.7] - 2021-10-25

### Fixed
- Fixed a bug in the variable-width integer serialization logic for Rust which affected numbers in the range [`567,382,630,219,904`, `72,624,976,668,147,840`).

## [0.0.6] - 2021-10-24

### Changed
- Messages now use a more compact binary encoding which guarantees that "zero-like" values (numerical `0`, Boolean `false`, empty arrays, etc.) consume zero bytes as fields.
- The encoding and decoding logic for variable-length integers has been optimized.

## [0.0.5] - 2021-10-23

### Added
- Introduced the `deleted` fields feature.

## [0.0.4] - 2021-10-16

### Fixed
- Fixed a bug that prevented Typical from working on Windows.

## [0.0.3] - 2021-10-14

### Changed
- The Rust code generator is now designed to be invoked by a Cargo build script.

## [0.0.2] - 2021-10-09

### Changed
- Renamed `--rust-out-file` to `--rust-out`.

## [0.0.1] - 2021-10-08

### Added
- Initial release.
