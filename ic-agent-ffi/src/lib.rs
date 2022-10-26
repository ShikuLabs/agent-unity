#![feature(unsize)]
#![feature(concat_bytes)]

use anyhow::Error as AnyErr;
use anyhow::Result as AnyResult;
use libc::c_int;
use std::marker::Unsize;

// TODO: Delete it
mod host;

mod agent;
mod identity;
mod principal;

/// NOTE: New Things

/// A callback used to give the unsized value to caller.
type UnsizedCallBack = extern "C" fn(*const u8, c_int);

/// TODO: Use macro to abstract the same parts from the different functions for reducing duplicated code.
fn ret_unsized(unsized_cb: UnsizedCallBack, s: impl AsRef<[u8]>) {
    let arr = s.as_ref();
    let len = arr.len() as c_int;

    unsized_cb(arr.as_ptr(), len);
}

unsafe fn ret_thin_ptr<T>(f2ptr: *mut *const T, t: T) {
    let boxed = Box::new(t);
    let raw = Box::into_raw(boxed);

    *f2ptr = raw;
}

unsafe fn ret_fat_ptr<ST, DT>(f2fptr: *mut *const DT, t: ST)
where
    // Static Type
    ST: Unsize<DT>,
    // Dynamic Type
    DT: ?Sized,
{
    let boxed = Box::new(t);
    let raw = Box::into_raw(boxed);

    *f2fptr = raw;
}

#[cfg(test)]
pub(crate) mod tests_util {
    use std::marker::Unsize;

    #[inline]
    pub const fn apply_fptr<ST, DT>() -> *const DT
    where
        // Static Type
        ST: Unsize<DT>,
        // Dynamic Type
        DT: ?Sized,
    {
        0u128 as *const ST as *const DT
    }

    #[inline]
    pub const fn apply_ptr<ST>() -> *const ST {
        0u64 as *const ST
    }
}

/// The state code represented the status of calling ffi functions.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum StateCode {
    /// Ok
    Ok = 0,
    /// Error
    Err = -1,
}
