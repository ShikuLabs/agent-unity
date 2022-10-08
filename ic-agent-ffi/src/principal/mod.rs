//! # Intro
//!
//! An `FFI Wrapper` for [`Principal`].

use ic_types::principal::Principal;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::string::ToString;

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
/// * `out_arr_size` - The size(in bytes) of memory to which `out_arr` points.
#[no_mangle]
pub extern "C" fn principal_management_canister(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    out_arr_size: u32,
) -> StateCode {
    let management_canister = Principal::management_canister();
    let arr = management_canister.as_slice();

    if arr.len() > out_arr_size as usize {
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
/// * `out_arr_size` - The size(in bytes) of memory to which `out_arr` points.
/// * `public_key` - The public key represented as u8 array.
/// * `public_key_len` - The length of public key.
#[no_mangle]
pub extern "C" fn principal_self_authenticating(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    out_arr_size: u32,
    public_key: *const u8,
    public_key_len: u32,
) -> StateCode {
    let public_key = unsafe { std::slice::from_raw_parts(public_key, public_key_len as usize) };
    let principal = Principal::self_authenticating(public_key);

    let arr = principal.as_slice();

    if arr.len() > out_arr_size as usize {
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
/// * `out_arr_size` - The size(in bytes) of memory to which `out_arr` points.
#[no_mangle]
pub extern "C" fn principal_anonymous(
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    out_arr_size: u32,
) -> StateCode {
    let anonymous = Principal::anonymous();
    let arr = anonymous.as_slice();

    if arr.len() > out_arr_size as usize {
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
/// * `in_bytes` - A pointer points to a chunk of memory that stores data waiting for conversion.
/// * `in_bytes_size` - The size(in bytes) of memory to which `in_bytes` points..
/// * `out_arr` - A pointer points to a chunk of memory allocated outside.
/// * `out_arr_len` - The size(in bytes) of data written into the memory to which `out_arr` points.
/// * `out_arr_size` - The size(in bytes) of memory to which `out_arr` points.
/// * `out_err_info` - A pointer points to a chunk of memory allocated outside which is used to store error information.
/// * `out_err_info_len` - The size(in bytes) of data written into the memory to which `out_err_info` points.
/// * `out_err_info_size` - The size(in bytes) of memory to which `out_err_info` points.
#[no_mangle]
pub extern "C" fn principal_from_bytes(
    in_bytes: *const u8,
    in_bytes_size: u32,
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    out_arr_size: u32,
    out_err_info: *mut c_char,
    out_err_info_len: *mut u32,
    out_err_info_size: u32,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(in_bytes, in_bytes_size as usize) };

    match Principal::try_from_slice(slice) {
        Ok(principal) => {
            let arr = principal.as_slice();

            if arr.len() > out_arr_size as usize {
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

            if err_bytes.len() > out_err_info_size as usize {
                return SC_ERR_INFO_OVERFLOW;
            }

            let err_ptr = err_bytes.as_ptr() as *const c_char;

            unsafe {
                std::ptr::copy(err_ptr, out_err_info, err_bytes.len());
                *out_err_info_len = err_bytes.len() as u32;
            }

            SC_INTERNAL_ERR
        }
    }
}

/// Construct a [`Principal`] from C style String.
///
/// The variable `out_bytes` points to a chunk of memory allocated outside rather than allocated
/// by rust.
///
/// If failed, the error info will be copied to the `out_err_info` which points
/// to a memory allocated outside.
#[no_mangle]
pub extern "C" fn principal_from_text(
    text: *const c_char,
    out_bytes: *mut u8,
    out_len: *mut u32,
    out_err_info: *mut c_char,
) -> bool {
    // let text = unsafe { CStr::from_ptr(text) };
    //
    // match Principal::from_text(text) {
    //     Ok(principal) => {
    //         let slice = principal.as_slice();
    //         let slice_len = slice.len();
    //
    //         unsafe {
    //             std::ptr::copy(slice.as_ptr(), out_bytes, slice_len);
    //             *out_len = slice_len as u32;
    //         }
    //
    //         true
    //     }
    //     Err(err) => {
    //         let err_bytes = _err2bytes(err);
    //         let err_ptr = err_bytes.as_ptr() as *const c_char;
    //
    //         unsafe {
    //             std::ptr::copy(err_ptr, out_err_info, err_bytes.len());
    //         }
    //
    //         false
    //     }
    // }
    todo!()
}

#[no_mangle]
pub extern "C" fn principal_to_text(
    in_bytes: *const u8,
    in_len: u32,
    out_bytes: *mut c_char,
    out_err_info: *mut c_char,
) -> bool {
    todo!()
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
        .unwrap_or(b"Failed on converting from `Error` to C Style String.\0".to_vec())
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
        // Allocation
        let (out_arr, out_arr_len) = unsafe { alloc_help_vars::<u8>(ARR_SIZE) };
        let (out_err_info, out_err_info_len) = unsafe { alloc_help_vars::<c_char>(ERR_INFO_SIZE) };
        let (in_bytes, in_bytes_size) = unsafe {
            const _2: usize = 16;
            let _1 = libc::malloc(1) as *mut u8;

            std::ptr::copy([0; _2].as_ptr(), _1, _2);

            (_1, _2)
        };

        assert_eq!(
            principal_from_bytes(
                in_bytes,
                in_bytes_size as u32,
                out_arr,
                out_arr_len,
                ARR_SIZE as u32,
                out_err_info,
                out_err_info_len,
                ERR_INFO_SIZE as u32,
            ),
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

            libc::free(out_err_info as *mut c_void);
            libc::free(out_err_info_len as *mut c_void);
        }
    }

    /// HELPER: Allocate helper variable
    unsafe fn alloc_help_vars<T>(size: usize) -> (*mut T, *mut u32) {
        let _1 = libc::malloc(size) as *mut T;
        let _2 = libc::malloc(size_of::<u32>()) as *mut u32;

        (_1, _2)
    }
}
