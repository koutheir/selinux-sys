[![crates.io](https://img.shields.io/crates/v/selinux-sys.svg)](https://crates.io/crates/selinux-sys)
[![docs.rs](https://docs.rs/selinux-sys/badge.svg)](https://docs.rs/selinux-sys)
[![license](https://img.shields.io/github/license/koutheir/selinux-sys?color=black)](https://raw.githubusercontent.com/koutheir/selinux-sys/master/LICENSE.md)

# `selinux-sys`: Unsafe Rust bindings for `libselinux`

SELinux is a flexible Mandatory Access Control (MAC) for Linux.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

## Linking options

This crate finds `libselinux` based on `pkg-config`.
Environment variables controlling the [`pkg-config`] crate also affect this crate.

[`pkg-config`]: https://crates.io/crates/pkg-config

## Depending on this crate

This crate provides the following variables to other crates that depend on it:
- `DEP_SELINUX_INCLUDE`: Path of the directory where library C header files reside.
- `DEP_SELINUX_LIB`: Path of the directory where the library binary resides.

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
