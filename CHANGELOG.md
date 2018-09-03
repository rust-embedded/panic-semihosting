# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.4.0] - 2018-09-03

### Changed

- This crate no longer depends on `arm-none-eabi-gcc`.

- [breaking-change] Move from the `panic_implementation` attribute to the
  `panic_handler` attribute, which will be stabilized.

## [v0.3.0] - 2018-06-04

### Changed

- [breaking-change] moved from the, now removed, `panic_fmt` lang item to the
  `#[panic_implementation]` attribute.

## [v0.2.0] - 2018-05-11

### Changed

- [breaking-change] made inline assembly (`asm!`) opt-in via the `"inline-asm"` feature. This is a
  breaking change because this crate now requires `arm-none-eabi-gcc` to be installed to build
  without the `"inline-asm"` feature, which is the default.

## v0.1.0 - 2018-04-09

Initial release

[Unreleased]: https://github.com/rust-embedded/panic-semihosting/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/rust-embedded/panic-semihosting/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/rust-embedded/panic-semihosting/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/rust-embedded/panic-semihosting/compare/v0.1.0...v0.2.0
