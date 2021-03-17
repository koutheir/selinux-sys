#![doc(html_root_url = "https://docs.rs/selinux-sys/0.2.0")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]

/*!
# `selinux-sys`: Unsafe Rust bindings for `libselinux`

SELinux is a flexible Mandatory Access Control (MAC) for Linux.

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
