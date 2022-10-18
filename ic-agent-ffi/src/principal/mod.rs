//! # Intro
//!
//! An `FFI Wrapper` for [`Principal`].

use crate::{ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use ic_types::principal::Principal;
use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::fmt::Display;

/// Construct the [`Principal`] of management canister.
#[no_mangle]
pub extern "C" fn principal_management_canister(ret_cb: UnsizedCallBack) {
    ret_unsized(ret_cb, Principal::management_canister());
}

/// Construct a [`Principal`] from a public key.
///
/// # Arguments
///
/// * `public_key` - The public key represented as u8 array.
/// * `public_key_len` - The length of public key.
#[no_mangle]
pub extern "C" fn principal_self_authenticating(
    public_key: *const u8,
    public_key_len: c_int,
    ret_cb: UnsizedCallBack,
) {
    let public_key = unsafe { std::slice::from_raw_parts(public_key, public_key_len as usize) };

    ret_unsized(ret_cb, Principal::self_authenticating(public_key));
}

/// Construct anonymous [`Principal`].
#[no_mangle]
pub extern "C" fn principal_anonymous(ret_cb: UnsizedCallBack) {
    ret_unsized(ret_cb, Principal::anonymous());
}

/// Construct a [`Principal`] from an array of bytes and pass the data of that principal to outside.
///
/// # Arguments
///
/// * `bytes` - A pointer points to a chunk of memory that stores data waiting for conversion.
/// * `bytes_len` - The size(in bytes) of memory to which `bytes` points..
#[no_mangle]
pub extern "C" fn principal_from_bytes(
    bytes: *const u8,
    bytes_len: c_int,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(bytes, bytes_len as usize) };

    let principal = Principal::try_from_slice(slice);

    __todo_replace_this_by_macro(ret_cb, err_cb, principal)
}

/// Construct a [`Principal`] from C style String.
///
/// # Arguments
///
/// * `text` - A C-Style String.
#[no_mangle]
pub extern "C" fn principal_from_text(
    text: *const c_char,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let text = unsafe { CStr::from_ptr(text).to_str().map_err(AnyErr::from) };

    let principal = text.and_then(|text| Principal::from_text(text).map_err(AnyErr::from));

    __todo_replace_this_by_macro(ret_cb, err_cb, principal)
}

/// Return the textual representation of [`Principal`].
///
/// # Arguments
///
/// * `bytes` - A pointer points to a chunk of memory that stores data waiting for conversion.
/// * `bytes_len` - The size(in bytes) of memory to which `bytes` points..
#[no_mangle]
pub extern "C" fn principal_to_text(
    bytes: *const u8,
    bytes_len: c_int,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(bytes, bytes_len as usize) };

    let principal = Principal::try_from_slice(slice).map_err(AnyErr::from);
    let text = principal
        .map(|principal| principal.to_text())
        .and_then(|text| CString::new(text).map_err(AnyErr::from))
        .map(|text| text.into_bytes_with_nul());

    __todo_replace_this_by_macro(ret_cb, err_cb, text)
}

pub(crate) fn __todo_replace_this_by_macro<T, E>(
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
    r: Result<T, E>,
) -> StateCode
where
    T: AsRef<[u8]>,
    E: Display,
{
    match r {
        Ok(v) => {
            ret_unsized(ret_cb, v);

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string());

            StateCode::Err
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_int;

    extern "C" fn empty_err_cb(_data: *const u8, _len: c_int) {}

    // TODO: Use macro to reduce the generation of callbacks.

    #[test]
    fn principal_management_canister_should_work() {
        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, &[0u8; 0]);
            assert_eq!(len, 0);
        }

        principal_management_canister(ret_cb);
    }

    #[test]
    fn principal_self_authenticating_should_work() {
        const PUBLIC_KEY_LEN: usize = 32;
        const SEED: [u8; PUBLIC_KEY_LEN] = [
            0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
            0x11, 0x00, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44,
            0x33, 0x22, 0x11, 0x00,
        ];
        const ANSWER: [u8; 29] = [
            0x2f, 0x8e, 0x47, 0x38, 0xf9, 0xd7, 0x68, 0x16, 0x82, 0x99, 0x85, 0x41, 0x52, 0x67,
            0x86, 0x38, 0x07, 0xd3, 0x7d, 0x20, 0x6a, 0xd9, 0x0f, 0xea, 0x72, 0xbf, 0x9d, 0xcf,
            0x02,
        ];

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, &ANSWER);
            assert_eq!(len as usize, ANSWER.len());
        }

        principal_self_authenticating(SEED.as_ptr(), SEED.len() as c_int, ret_cb);
    }

    #[test]
    fn principal_anonymous_should_work() {
        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, &[4u8; 1]);
            assert_eq!(len, 1);
        }

        principal_anonymous(ret_cb);
    }

    #[test]
    fn principal_from_bytes_should_work() {
        const BYTES: [u8; 16] = [0x00; 16];

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, &BYTES);
            assert_eq!(len as usize, BYTES.len());
        }

        assert_eq!(
            principal_from_bytes(BYTES.as_ptr(), BYTES.len() as c_int, ret_cb, empty_err_cb,),
            StateCode::Ok
        );
    }

    #[test]
    fn principal_from_text_should_work() {
        const ANONYMOUS_TEXT: &[u8; 10] = b"2vxsx-fae\0";
        const ANONYMOUS_BYTES: [u8; 1] = [0x04u8];

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, &ANONYMOUS_BYTES);
            assert_eq!(len as usize, ANONYMOUS_BYTES.len());
        }

        assert_eq!(
            principal_from_text(
                ANONYMOUS_TEXT.as_ptr() as *const c_char,
                ret_cb,
                empty_err_cb,
            ),
            StateCode::Ok
        );
    }

    #[test]
    fn principal_to_text_should_work() {
        const ANONYMOUS_TEXT: &[u8; 10] = b"2vxsx-fae\0";
        const ANONYMOUS_BYTES: [u8; 1] = [0x04u8];

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, ANONYMOUS_TEXT);
            assert_eq!(len as usize, ANONYMOUS_TEXT.len());
        }

        assert_eq!(
            principal_to_text(
                ANONYMOUS_BYTES.as_ptr(),
                ANONYMOUS_BYTES.len() as c_int,
                ret_cb,
                empty_err_cb,
            ),
            StateCode::Ok
        );
    }
}
