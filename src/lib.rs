#![cfg(all(target_os = "linux", not(target_env = "kernel")))]
#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/selinux-sys/0.6.9")]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::redundant_static_lifetimes,
    clippy::upper_case_acronyms
)]

include!(concat!(env!("OUT_DIR"), "/selinux-sys.rs"));

/// Unspecified SID.
pub const SECSID_WILD: security_id_t = std::ptr::null_mut();

/// Initialize an `avc_entry_ref` structure.
///
/// # Safety
/// `aeref` is assumed to be a valid pointer to a mutable `avc_entry_ref` structure.
pub unsafe fn avc_entry_ref_init(aeref: *mut avc_entry_ref) {
    if !aeref.is_null() {
        unsafe {
            aeref.write(avc_entry_ref {
                ae: std::ptr::null_mut(),
            })
        };
    }
}

#[cfg(test)]
mod tests;
