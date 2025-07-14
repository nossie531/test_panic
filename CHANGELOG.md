# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD

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

## [0.3.0] - 2023-10-30

### Added

- Add `must_use` annotations to `TestPanicResult`.

## [0.2.0] - 2023-07-16

### Changed

- `TestPanicResult` holds value on cases where no panic occurred.
- Polish documentation.
