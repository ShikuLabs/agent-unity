//! # Intro
//!
//! An `FFI Wrapper` for [`Principal`].

use anyhow::Error as AnyErr;
use ic_types::principal::Principal;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::string::ToString;

// TODO: Reduce the code amount by eliminating the duplication.
// TODO: More Unit-Tests.

pub type StateCode = i32;

/// Ok
pub const SC_OK: i32 = 0;
/// The data copied to the memory overflows.
pub const SC_DATA_OVERFLOW: i32 = -1;
/// The function was terminated by an internal error.
pub const SC_INTERNAL_ERR: i32 = -2;
/// The error info copied to the memory overflows.
pub const SC_ERR_INFO_OVERFLOW: i32 = -3;

/// Construct the [`Principal`] of management canister.
///
/// # Arguments
///
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `arr_size` - The size(in bytes) of memory to which `out_arr` points.
#[no_mangle]
pub extern "C" fn principal_management_canister(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
) -> StateCode {
    let management_canister = Principal::management_canister();
    let arr = management_canister.as_slice();

    if arr.len() > arr_size as usize {
        return SC_DATA_OVERFLOW;
    }

    unsafe {
        std::ptr::copy(arr.as_ptr(), out_arr, arr.len());
        *out_arr_len = arr.len() as u32;
    }

    SC_OK
}

/// Construct a [`Principal`] from a public key.
///
/// # Arguments
///
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `arr_size` - The size(in bytes) of memory to which `out_arr` points.
/// * `public_key` - The public key represented as u8 array.
/// * `public_key_size` - The length of public key.
#[no_mangle]
pub extern "C" fn principal_self_authenticating(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
    public_key: *const u8,
    public_key_size: u32,
) -> StateCode {
    let public_key =
        unsafe { std::slice::from_raw_parts(public_key, public_key_size as usize) };
    let principal = Principal::self_authenticating(public_key);

    let arr = principal.as_slice();

    if arr.len() > arr_size as usize {
        return SC_DATA_OVERFLOW;
    }

    unsafe {
        std::ptr::copy(arr.as_ptr(), out_arr, arr.len());
        *out_arr_len = arr.len() as u32;
    }

    SC_OK
}

/// Construct anonymous [`Principal`].
///
/// # Arguments
///
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `arr_size` - The size(in bytes) of memory to which `out_arr` points.
#[no_mangle]
pub extern "C" fn principal_anonymous(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
) -> StateCode {
    let anonymous = Principal::anonymous();
    let arr = anonymous.as_slice();

    if arr.len() > arr_size as usize {
        return SC_DATA_OVERFLOW;
    }

    unsafe {
        std::ptr::copy(arr.as_ptr(), out_arr, arr.len());
        *out_arr_len = arr.len() as u32;
    }

    SC_OK
}

/// Construct a [`Principal`] from an array of bytes and pass the data of that principal to outside.
///
/// # Arguments
///
/// * `bytes` - A pointer points to a chunk of memory that stores data waiting for conversion.
/// * `bytes_size` - The size(in bytes) of memory to which `bytes` points..
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `arr_size` - The size(in bytes) of memory to which `out_arr` points.
/// * `out_err_info` - A pointer points to a chunk of memory allocated outside which is used to store error information.
/// * `err_info_size` - The size(in bytes) of memory to which `out_err_info` points.
#[no_mangle]
pub extern "C" fn principal_from_bytes(
    bytes: *const u8,
    bytes_size: u32,
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
    out_err_info: *mut c_char,
    err_info_size: u32,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(bytes, bytes_size as usize) };

    match Principal::try_from_slice(slice) {
        Ok(principal) => {
            let arr = principal.as_slice();

            if arr.len() > arr_size as usize {
                return SC_DATA_OVERFLOW;
            }

            unsafe {
                std::ptr::copy(arr.as_ptr(), out_arr, arr.len());
                *out_arr_len = arr.len() as u32;
            }

            SC_OK
        }
        Err(err) => {
            let err_bytes = _err2bytes(err);

            if err_bytes.len() > err_info_size as usize {
                return SC_ERR_INFO_OVERFLOW;
            }

            let err_ptr = err_bytes.as_ptr() as *const c_char;

            unsafe {
                std::ptr::copy(err_ptr, out_err_info, err_bytes.len());
            }

            SC_INTERNAL_ERR
        }
    }
}

/// Construct a [`Principal`] from C style String.
///
/// # Arguments
///
/// * `text` - A C Style String.
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `arr_size` - The size(in bytes) of memory to which `out_arr` points.
/// * `out_err_info` - A pointer points to a chunk of memory allocated outside which is used to store error information.
/// * `err_info_size` - The size(in bytes) of memory to which `out_err_info` points.
#[no_mangle]
pub extern "C" fn principal_from_text(
    text: *const c_char,
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
    out_err_info: *mut c_char,
    err_info_size: u32,
) -> StateCode {
    let text = unsafe { CStr::from_ptr(text).to_str().map_err(AnyErr::from) };

    let principal = text.and_then(|text| Principal::from_text(text).map_err(AnyErr::from));

    match principal {
        Ok(principal) => {
            let arr = principal.as_slice();

            if arr.len() > arr_size as usize {
                return SC_DATA_OVERFLOW;
            }

            unsafe {
                std::ptr::copy(arr.as_ptr(), out_arr, arr.len());
                *out_arr_len = arr.len() as u32;
            }

            SC_OK
        }
        Err(err) => {
            let err_bytes = _err2bytes(err);

            if err_bytes.len() > err_info_size as usize {
                return SC_ERR_INFO_OVERFLOW;
            }

            let err_ptr = err_bytes.as_ptr() as *const c_char;

            unsafe {
                std::ptr::copy(err_ptr, out_err_info, err_bytes.len());
            }

            SC_INTERNAL_ERR
        }
    }
}

/// Return the textual representation of [`Principal`].
///
/// # Arguments
///
/// * `bytes` - A pointer points to a chunk of memory that stores data waiting for conversion.
/// * `bytes_size` - The size(in bytes) of memory to which `bytes` points..
/// * `out_text` - A pointer points to a chunk of memory allocated outside.
/// * `out_err_info` - A pointer points to a chunk of memory allocated outside which is used to store error information.
/// * `err_info_size` - The size(in bytes) of memory to which `out_err_info` points.
#[no_mangle]
pub extern "C" fn principal_to_text(
    bytes: *const u8,
    bytes_size: u32,
    out_text: *mut c_char,
    out_text_size: u32,
    out_err_info: *mut c_char,
    err_info_size: u32,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(bytes, bytes_size as usize) };

    let principal = Principal::try_from_slice(slice).map_err(AnyErr::from);
    let principal_text = principal
        .map(|principal| principal.to_text())
        .and_then(|text| CString::new(text).map_err(AnyErr::from))
        .map(|text| text.into_bytes_with_nul());

    match principal_text {
        Ok(principal_text) => {
            if principal_text.len() > out_text_size as usize {
                return SC_DATA_OVERFLOW;
            }

            unsafe {
                std::ptr::copy(
                    principal_text.as_ptr() as *const c_char,
                    out_text,
                    principal_text.len(),
                );
            }

            SC_OK
        }
        Err(err) => {
            let err_bytes = _err2bytes(err);

            if err_bytes.len() > err_info_size as usize {
                return SC_ERR_INFO_OVERFLOW;
            }

            let err_ptr = err_bytes.as_ptr() as *const c_char;

            unsafe {
                std::ptr::copy(err_ptr, out_err_info, err_bytes.len());
            }

            SC_INTERNAL_ERR
        }
    }
}

/// HELPER: Convert `Error` implemented trait [`Display`] to C style String(represented as u8 array).
///
/// If failed, the return C Style String will be that(IGNORE QUOTATION MARKS):
///
/// "Failed on converting from `Error` to C Style String.\0"
fn _err2bytes<E: Display>(err: E) -> Vec<u8> {
    let err_str = err.to_string();

    CString::new(err_str)
        .map(|cstr| cstr.into_bytes_with_nul())
        .unwrap_or_else(|_| b"Failed on converting from `Error` to C Style String.\0".to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::c_void;
    use std::mem::size_of;

    const ARR_SIZE: usize = 32;
    const ERR_INFO_SIZE: usize = 256;

    #[test]
    fn principal_management_canister_should_work() {
        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };

        assert_eq!(
            principal_management_canister(out_arr, out_arr_len, ARR_SIZE as u32),
            SC_OK
        );

        unsafe {
            let slice_len = *out_arr_len as usize;
            let slice = std::slice::from_raw_parts(out_arr, slice_len);

            assert_eq!(slice_len, 0);
            assert_eq!(slice, [].as_slice() as &[u8]);
        }

        // Free
        unsafe {
            libc::free(out_arr as *mut c_void);
            libc::free(out_arr_len as *mut c_void);
        }
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

        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };
        let public_key = unsafe {
            let ptr = libc::malloc(PUBLIC_KEY_LEN) as *mut u8;
            std::ptr::copy(SEED.as_ptr(), ptr, PUBLIC_KEY_LEN);

            ptr as *const u8
        };

        assert_eq!(
            principal_self_authenticating(
                out_arr,
                out_arr_len,
                ARR_SIZE as u32,
                public_key,
                PUBLIC_KEY_LEN as u32,
            ),
            SC_OK
        );

        unsafe {
            let slice_len = *out_arr_len as usize;
            let slice = std::slice::from_raw_parts(out_arr, slice_len);

            assert_eq!(slice, ANSWER.as_slice());
        }

        // Free
        unsafe {
            libc::free(out_arr as *mut c_void);
            libc::free(out_arr_len as *mut c_void);
            libc::free(public_key as *mut c_void);
        }
    }

    #[test]
    fn principal_anonymous_should_work() {
        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };

        assert_eq!(
            principal_anonymous(out_arr, out_arr_len, ARR_SIZE as u32),
            SC_OK
        );

        unsafe {
            let slice_len = *out_arr_len as usize;
            let slice = std::slice::from_raw_parts(out_arr, slice_len);

            assert_eq!(slice_len, 1);
            assert_eq!(slice, [4].as_slice());
        }

        // Free
        unsafe {
            libc::free(out_arr as *mut c_void);
            libc::free(out_arr_len as *mut c_void);
        }
    }

    #[test]
    fn principal_from_bytes_should_work() {
        const BYTES: [u8; 16] = [0x00; 16];

        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };
        let out_err_info = unsafe { libc::malloc(ERR_INFO_SIZE) as *mut c_char };
        let (bytes, bytes_size) = unsafe {
            let _1 = libc::malloc(BYTES.len()) as *mut u8;
            let _2 = BYTES.len();

            std::ptr::copy(BYTES.as_ptr(), _1, _2);

            (_1, _2)
        };

        assert_eq!(
            principal_from_bytes(
                bytes,
                bytes_size as u32,
                out_arr,
                out_arr_len,
                ARR_SIZE as u32,
                out_err_info,
                ERR_INFO_SIZE as u32,
            ),
            SC_OK
        );

        unsafe {
            let slice_len = *out_arr_len as usize;
            let slice = std::slice::from_raw_parts(out_arr, slice_len);

            assert_eq!(slice_len, BYTES.len());
            assert_eq!(slice, BYTES.as_slice());
        }

        // Free
        unsafe {
            libc::free(out_arr as *mut c_void);
            libc::free(out_arr_len as *mut c_void);

            libc::free(out_err_info as *mut c_void);

            libc::free(bytes as *mut c_void);
        }
    }

    #[test]
    fn principal_from_text_should_work() {
        const ANONYMOUS_TEXT: &[u8; 10] = b"2vxsx-fae\0";
        const ANONYMOUS_BYTES: [u8; 1] = [0x04u8];

        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };
        let out_err_info = unsafe { libc::malloc(ERR_INFO_SIZE) as *mut c_char };
        let text = unsafe {
            let text = libc::malloc(ANONYMOUS_TEXT.len()) as *mut u8;

            std::ptr::copy(ANONYMOUS_TEXT.as_ptr(), text, ANONYMOUS_TEXT.len());

            text as *const c_char
        };

        assert_eq!(
            principal_from_text(
                text,
                out_arr,
                out_arr_len,
                ARR_SIZE as u32,
                out_err_info,
                ERR_INFO_SIZE as u32
            ),
            SC_OK
        );

        unsafe {
            let slice_len = *out_arr_len as usize;
            let slice = std::slice::from_raw_parts(out_arr, slice_len);

            assert_eq!(slice_len, ANONYMOUS_BYTES.len());
            assert_eq!(slice, ANONYMOUS_BYTES.as_slice());
        }

        // Free
        unsafe {
            libc::free(out_arr as *mut c_void);
            libc::free(out_arr_len as *mut c_void);

            libc::free(out_err_info as *mut c_void);

            libc::free(text as *mut c_void);
        }
    }

    #[test]
    fn principal_to_text_should_work() {
        const ANONYMOUS_TEXT: &[u8; 10] = b"2vxsx-fae\0";
        const ANONYMOUS_BYTES: [u8; 1] = [0x04u8];

        // Allocation
        let (out_text, out_text_size) = unsafe {
            const _2: usize = 256;
            let _1 = libc::malloc(_2) as *mut c_char;

            (_1, _2)
        };
        let out_err_info = unsafe { libc::malloc(ERR_INFO_SIZE) as *mut c_char };
        let bytes = unsafe {
            let bytes = libc::malloc(ANONYMOUS_BYTES.len()) as *mut u8;

            std::ptr::copy(ANONYMOUS_BYTES.as_ptr(), bytes, ANONYMOUS_BYTES.len());

            bytes as *const u8
        };

        assert_eq!(
            principal_to_text(
                bytes,
                ANONYMOUS_BYTES.len() as u32,
                out_text,
                out_text_size as u32,
                out_err_info,
                ERR_INFO_SIZE as u32
            ),
            SC_OK
        );

        unsafe {
            let out_text = CStr::from_ptr(out_text);
            let anonymous_text = CStr::from_bytes_with_nul_unchecked(ANONYMOUS_TEXT);

            assert_eq!(out_text, anonymous_text);
        }

        // Free
        unsafe {
            libc::free(out_text as *mut c_void);

            libc::free(out_err_info as *mut c_void);

            libc::free(bytes as *mut c_void);
        }
    }

    /// HELPER: Allocate helper variable
    unsafe fn alloc_help_vars<T>(size: usize) -> (*mut T, *mut u32) {
        let _1 = libc::malloc(size) as *mut T;
        let _2 = libc::malloc(size_of::<u32>()) as *mut u32;

        (_1, _2)
    }
}
