# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.12.1] - 2024-06-19

### Changed
- The TypeScript deserialization functions now have a more permissive type signature.

## [0.12.0] - 2024-06-19

### Changed
- Typical no longer requires specifying `$field` when serializing choices.

## [0.11.0] - 2024-06-18

### Changed
- Typical now requires optional struct fields to be explicitly set to `undefined` in TypeScript to avoid setting them. Previously, such fields could be omitted entirely, but that meant typos in optional field names could go unnoticed.

## [0.10.0] - 2024-06-13

### Added
- Typical now requires field index gaps to be marked as deleted.

## [0.9.7] - 2024-06-14

### Changed
- Some redundant conversions are no longer emitted by the Rust code generator.

## [0.9.6] - 2023-06-18

### Added
- Typical supports a new platform: Windows on AArch64.

## [0.9.5] - 2023-06-02

### Added
- Typical supports a new platform: musl Linux on AArch64.

## [0.9.4] - 2023-05-23

### Added
- Typical supports a new platform: GNU Linux on AArch64.

## [0.9.3] - 2023-05-13

### Added
- Typical supports a new platform: macOS on Apple silicon.

## [0.9.2] - 2022-01-11

### Changed
- The deserialization code generated for Rust is now significantly faster for certain types of messages.
- The serialization and deserialization functions generated for Rust now have more general type signatures.

## [0.9.1] - 2022-01-11

### Changed
- The generated Rust code is now slightly more DRY.

## [0.9.0] - 2022-01-11

### Changed
- The generated Rust code now uses an asymptotically more efficient serialization algorithm. The binary encoding is unchanged.
- The generated TypeScript code now exports a `size` function for computing the expected size of a message on the wire.
- Identifiers are no longer allowed to begin with underscores to allow code generators to use underscore-prefixed names without worrying about colliding with user-defined fields.

## [0.8.7] - 2022-01-10

### Fixed
- Fixed a bug in the TypeScript code generator that allowed for a name collision with user-defined fields named `size`.

## [0.8.6] - 2022-01-10

### Changed
- The generated TypeScript code now exports a safer API for deserialization.

## [0.8.5] - 2022-01-09

### Changed
- The performance of string serialization for TypeScript has been significantly optimized.

## [0.8.4] - 2022-01-09

### Changed
- The generated TypeScript code now exports a simpler API for serialization for convenience.

## [0.8.3] - 2022-01-09

### Changed
- The generated TypeScript code now uses an asymptotically more efficient serialization algorithm. The binary encoding is unchanged.

## [0.8.2] - 2022-01-08

### Changed
- The generated Rust and TypeScript code now features exported utility functions near the beginning of the file for better visibility.

## [0.8.1] - 2022-01-07

### Changed
- Both code generators now produce slightly more efficient code.

## [0.8.0] - 2022-01-06

### Changed
- The signature of the TypeScript deserialization functions has been simplified.

## [0.7.2] - 2022-01-06

### Changed
- The TypeScript code generator now produces more efficient code.

## [0.7.1] - 2022-01-06

### Changed
- The TypeScript code generator now produces code that makes ESLint happier.

### Changed

## [0.7.0] - 2022-01-06

### Changed
- Choices now have a different representation in TypeScript and JavaScript. The payload is now a property named after the appropriate field, rather than the `value` property. To avoid collisions with user-defined fields, the `field` and `fallback` fields have been renamed to `$field` and `$fallback`, respectively. For example, `{ type: 'foo', value: 0 }` is now `{ $type: 'foo', foo: 0 }`.

## [0.6.0] - 2022-01-04

### Fixed
- Fixed several bugs in the TypeScript code generator. A comprehensive integration test suite has been added which features the same tests as the Rust suite. The binary data produced by both suites are validated to be bit-for-bit identical.

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
