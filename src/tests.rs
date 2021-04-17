#![cfg(all(test, target_os = "linux", not(target_env = "kernel")))]

use std::os::raw::c_char;
use std::ptr;

#[test]
fn is_selinux_enabled() {
    let r = unsafe { super::is_selinux_enabled() };
    assert!(r == 0 || r == 1);
}

#[test]
fn getcon() {
    let mut context: *mut c_char = ptr::null_mut();
    let r = unsafe { super::getcon(&mut context) };
    assert_ne!(r, -1);
    assert!(!context.is_null());

    unsafe { super::freecon(context) }
}
