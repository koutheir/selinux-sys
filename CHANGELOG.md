# Change log

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.9] - 2024-03-27

### Changed

- Updated dependencies.
- Moved repository to `codeberg.org`.

## [0.6.8] - 2024-01-19

### Changed

- Updated dependencies.

## [0.6.7] - 2023-10-31

### Changed

- When include and/or link paths are specified explicitly, these paths must be provided to all
  compiler instances, both when discovering compiler paths, and when executing `bindgen`.
- Updated dependencies.

## [0.6.6] - 2023-08-09

### Changed

- Updated dependencies.

## [0.6.5] - 2023-06-07

### Changed

- Updated build script to better integrate with `cargo`.

## [0.6.4] - 2023-04-18

### Changed

- Updated dependencies.

## [0.6.3] - 2023-02-23

### Changed

- Updated dependencies.

## [0.6.2] - 2022-11-22

### Changed

- Updated dependencies.

## [0.6.1] - 2022-11-20

### Changed

- Updated dependencies.

## [0.6.0] - 2022-11-14

### Changed

- Rust edition is updated to 2021.

  > ⚠️ **This is a breaking change**.

- Updated dependencies.

## [0.5.3] - 2022-08-24

### Changed

- Pass headers include path to the compiler when generating bindings.
  Specifying the `SELINUX_INCLUDE_DIR` or `SELINUX_PATH` environment variables
  now also affects the command line of the compiler used to generate bindings.
- Updated dependencies.

Thank you very much, *etienne-cor*.

## [0.5.2] - 2022-02-02

### Added

- Support building for target triplets that end with `-linux`.

## [0.5.1] - 2021-08-01

### Changed

- Stopped using `std::slice::strip_prefix()`, in order to reduce the minimum
  supported Rust version for this crate.

## [0.5.0] - 2021-07-28

### Changed

- Updated dependencies.

### Removed

- Removed all fixed C integer types, such as `__uint8_t`, `__uint32_t`, etc.

  > ⚠️ **This is a breaking change**.

## [0.4.2] - 2021-06-01

### Added

- Implemented `Eq` and `Ord` for generated structures.

## [0.4.1] - 2021-05-09

### Added

- Exposed the `getseuser()` API.

## [0.4.0] - 2021-05-04

### Changed

- Instead of using the `pkg-config` crate, we now detect compilers and flags
  mostly based on the `cc` crate. This allows cross-compilation.
- Updated documentation.

## [0.3.1] - 2021-04-17

### Changed

- Updated documentation.

## [0.3.0] - 2021-04-17

### Changed

- Building for the following platforms is now supported, but it results in
  an empty crate:
  - Non-Linux systems.
  - The Linux kernel.

## [0.2.1] - 2021-04-17

### Changed

- Updated dependencies.

## [0.2.0] - 2021-03-17

### Fixed

- Make sure `DEP_SELINUX_INCLUDE` and `DEP_SELINUX_LIB` point to directories.
  They previously pointed to files.

## [0.1.0] - 2021-03-06

### Added

- Initial release.
