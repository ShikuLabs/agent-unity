use crate::{create_string_to_cs, OutValue};
use ic_types::Principal;
use libc::{c_char, c_void};
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn principal_management_canister() -> *const Principal {
    let prp = Principal::management_canister();
    let prp_boxed = Box::new(prp);

    let ptr = Box::into_raw(prp_boxed);
    std::mem::forget(ptr);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn principal_self_authenticating(
    public_key: *const u8,
    len: u32,
) -> *const Principal {
    let public_key = std::slice::from_raw_parts(public_key, len as usize);

    let prp = Principal::self_authenticating(public_key);
    let prp_boxed = Box::new(prp);

    let ptr = Box::into_raw(prp_boxed);
    std::mem::forget(ptr);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn principal_anonymous() -> *const Principal {
    let prp = Principal::anonymous();
    let prp_boxed = Box::new(prp);

    let ptr = Box::into_raw(prp_boxed);
    std::mem::forget(ptr);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn principal_from_text(text: *const c_char) -> OutValue {
    let text = CStr::from_ptr(text).to_str().unwrap();
    let prp = Principal::from_text(text);

    prp.map_or_else(
        |e| {
            let ptr = create_string_to_cs(e.to_string()) as *mut c_void;

            OutValue { ptr, is_err: true }
        },
        |prp| {
            let prp_boxed = Box::new(prp);

            let ptr = Box::into_raw(prp_boxed) as *mut c_void;
            std::mem::forget(ptr);

            OutValue { ptr, is_err: false }
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn principal_to_text(slf: *const Principal) -> *mut c_char {
    let prp_boxed = Box::from_raw(slf as *mut Principal);
    let prp_text = prp_boxed.to_text();

    std::mem::forget(Box::into_raw(prp_boxed));

    create_string_to_cs(prp_text)
}

#[no_mangle]
pub unsafe extern "C" fn principal_free(slf: *mut Principal) {
    drop(Box::from_raw(slf))
}
