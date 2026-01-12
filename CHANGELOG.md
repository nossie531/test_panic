# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-01-13

### Added

- Add: `assert_eqa` macro.
- Add: `assert_eqn` macro.

### Changed

- Polish documentation.
- Minor refactoring.
- Rename: `eq_almost` from `almost_eq`.
- Rename: `eq_nearly` from `nearly_eq`.

## [0.6.0] - 2025-12-28

### Added

- Add `ok`, `ng`, and `msg` method.

### Changed

- Polish documentation.
- Minor refactoring.

### Fixed

- Fix failing test case (`with_panic_empty`).  
  It had broken without noticing (I hadn't run CI...).

## [0.5.0] - 2025-12-16

### Added

- Add `nearly_eq` and `get_message` method for `TestPanicResult`.
- Add `Eq` and `PartialEq` trait implementation for `TestPanicResult`.

### Changed

- Minor refactoring.

## [0.4.1] - 2025-07-14

### Changed

- Polish documentation.

## [0.4.0] - 2025-06-23

### Added

- Add `prelude` module (Although this crate is very small).

### Changed

- Rust edition is updated to 2024.
- Follow latest `std` API (`PanicHookInfo` instead of `PanicInfo`).
- Fix broken unit tests.
- Polish documentation.

## [0.3.1] - 2024-01-15

### Changed

- Minor refactoring.

[0.7.0]: https://github.com/nossie531/test_panic/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/nossie531/test_panic/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/nossie531/test_panic/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/nossie531/test_panic/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/nossie531/test_panic/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/nossie531/test_panic/compare/v0.3.0...v0.3.1
