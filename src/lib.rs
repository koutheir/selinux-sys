#![cfg(all(target_os = "linux", not(target_env = "kernel")))]
#![doc(html_root_url = "https://docs.rs/selinux-sys/0.4.1")]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(clippy::redundant_static_lifetimes, clippy::upper_case_acronyms)]

/*!
# `selinux-sys`: Unsafe Rust bindings for `libselinux`

SELinux is a flexible Mandatory Access Control (MAC) for Linux.

This crate exposes neither *deprecated* nor *undocumented* SELinux API functions
and types.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

## Supported environment variables

This crate depends on some environment variables, and *variants* of those.
For each environment variable (e.g., `CC`), the following are the accepted
variants of it:
- `<var>_<target>`, e.g., `CC_aarch64-unknown-linux-gnu`.
- `<var>_<target-with-underscores>`, e.g., `CC_aarch64_unknown_linux_gnu`.
- `TARGET_<var>`, e.g., `TARGET_CC`.
- `<var>`, e.g., `CC`.

The following environment variables (and their variants) affect how this crate
is built:
- `SELINUX_STATIC`
- `SELINUX_PATH`
- `SELINUX_INCLUDE_DIR`
- `SELINUX_LIB_DIR`
- `CC`
- `CFLAGS`

## Dynamic or static linking

This crate links to `libselinux` dynamically if possible, except when targeting
platforms based on the `musl` C library.

This behavior can be changed either by setting the environment variable
`SELINUX_STATIC` to `1`, or by enabling the crate feature `static`.
If both are defined, then the value of `SELINUX_STATIC` takes precedence.

Setting `SELINUX_STATIC` to `0` mandates dynamic linking.

## Finding SELinux library and headers

By default, this crate finds SELinux headers and library based on the default
target C compiler.

This behavior can be changed by:
- Either defining the environment variable `SELINUX_PATH` to the path of
  a directory containing the sub-directories `include` and `lib` where
  the headers and library are installed.
- Or by defining one or both of the environment variables `SELINUX_INCLUDE_DIR`
  and `SELINUX_LIB_DIR` to paths to the directories where headers and library
  are present. If `SELINUX_PATH` is also defined, then `SELINUX_INCLUDE_DIR`
  and `SELINUX_LIB_DIR` take precedence.

## Depending on this crate

This crate provides the following variables to other crates that depend on it:
- `DEP_SELINUX_INCLUDE`: Path of the directory where library C header files reside.
- `DEP_SELINUX_LIB`: Path of the directory where the library binary resides.

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
*/

include!(concat!(env!("OUT_DIR"), "/selinux-sys.rs"));

/// Unspecified SID.
pub const SECSID_WILD: security_id_t = std::ptr::null_mut();

/// Initialize an `avc_entry_ref` structure.
///
/// # Safety
/// `aeref` is assumed to be a valid pointer to a mutable `avc_entry_ref` structure.
pub unsafe fn avc_entry_ref_init(aeref: *mut avc_entry_ref) {
    if let Some(aeref) = aeref.as_mut() {
        aeref.ae = std::ptr::null_mut();
    }
}

#[cfg(test)]
mod tests;
