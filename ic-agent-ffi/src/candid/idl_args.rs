use crate::{ret_thin_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use candid::parser::value::IDLValue;
use candid::IDLArgs;
use libc::{c_char, c_int};
use std::ffi::CStr;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

#[no_mangle]
pub extern "C" fn idl_args_to_text(ptr: *const IDLArgs, ret_cb: UnsizedCallBack<u8>) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };

    let idl_str = boxed.to_string() + "\0";

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    ret_unsized(ret_cb, idl_str)
}

#[no_mangle]
pub extern "C" fn idl_args_from_text(
    text: *const c_char,
    p2ptr: *mut *const IDLArgs,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let text = unsafe { CStr::from_ptr(text).to_str().map_err(AnyErr::from) };

    let idl_value = text.and_then(|text| IDLArgs::from_str(text).map_err(AnyErr::from));

    __todo_replace_this_by_macro(p2ptr, err_cb, idl_value)
}

#[no_mangle]
pub extern "C" fn idl_args_to_bytes(
    ptr: *const IDLArgs,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };

    let idl_bytes = boxed.to_bytes();

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, idl_bytes)
}

#[no_mangle]
pub extern "C" fn idl_args_from_bytes(
    bytes: *const u8,
    bytes_len: c_int,
    p2ptr: *mut *const IDLArgs,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let slice = unsafe { std::slice::from_raw_parts(bytes, bytes_len as usize) };

    let idl_args = IDLArgs::from_bytes(slice);

    __todo_replace_this_by_macro(p2ptr, err_cb, idl_args)
}

#[no_mangle]
pub extern "C" fn idl_args_ct_vec(
    elems: *const *const IDLValue,
    elems_len: c_int,
    p2ptr: *mut *const IDLArgs,
) {
    let mut values = Vec::new();

    for i in 0..elems_len as usize {
        unsafe {
            let val_ptr = *elems.add(i);
            let boxed = Box::from_raw(val_ptr as *mut IDLValue);

            values.push(boxed.deref().clone());

            // keep available the fat pointer to the [`Identity`]
            let _ = Box::into_raw(boxed);
        }
    }

    let idl_args = IDLArgs { args: values };

    unsafe {
        ret_thin_ptr(p2ptr, idl_args);
    }
}

#[no_mangle]
pub extern "C" fn idl_args_as_vec(ptr: *const IDLArgs, ret_cb: UnsizedCallBack<*const IDLValue>) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };

    let r = {
        let idl_values = boxed.args.clone();

        let mut ptrs = Vec::new();

        for idl_value in idl_values {
            let boxed = Box::new(idl_value);
            let ptr = Box::into_raw(boxed);

            ptrs.push(ptr as *const IDLValue);
        }

        ptrs
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    ret_unsized(ret_cb, r);
}

#[no_mangle]
pub extern "C" fn idl_args_free(ptr: *const IDLArgs) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro(
    p2ptr: *mut *const IDLArgs,
    err_cb: UnsizedCallBack<u8>,
    r: Result<IDLArgs, impl Display>,
) -> StateCode {
    match r {
        Ok(t) => {
            unsafe {
                ret_thin_ptr(p2ptr, t);
            }

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string() + "\0");

            StateCode::Err
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_util::{apply_ptr, empty_err_cb};
    use ic_types::Principal;
    use std::ops::Deref;

    const IDL_VALUES: [IDLValue; 3] = [
        IDLValue::Bool(true),
        IDLValue::Principal(Principal::anonymous()),
        IDLValue::Int32(-12),
    ];

    const IDL_ARGS_TEXT: &str = r#"(true, principal "2vxsx-fae", -12 : int32)"#;
    const IDL_ARGS_TEXT_C: &str = "(true, principal \"2vxsx-fae\", -12 : int32)\0";

    const IDL_ARGS_BYTES: &[u8] = &[
        68, 73, 68, 76, 0, 3, 126, 104, 117, 1, 1, 1, 4, 244, 255, 255, 255,
    ];

    extern "C" fn empty_cb(_data: *const u8, _len: c_int) {}

    #[test]
    fn idl_args_to_text_should_work() {
        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(IDL_ARGS_TEXT, str);
        }

        let idl_args = IDLArgs::new(&IDL_VALUES);

        let idl_args_boxed = Box::new(idl_args);
        let ptr = Box::into_raw(idl_args_boxed);

        idl_args_to_text(ptr, ret_cb);

        idl_args_free(ptr);
    }

    #[test]
    fn idl_args_from_text_should_work() {
        let mut ptr = apply_ptr::<IDLArgs>();

        assert_eq!(
            idl_args_from_text(
                IDL_ARGS_TEXT_C.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
            ),
            StateCode::Ok
        );

        let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };
        assert_eq!(&IDLArgs::new(&IDL_VALUES), boxed.as_ref());
    }

    #[test]
    fn idl_args_to_bytes_should_work() {
        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(IDL_ARGS_BYTES, slice);
        }

        let idl_args = IDLArgs::new(&IDL_VALUES);

        let idl_args_boxed = Box::new(idl_args);
        let ptr = Box::into_raw(idl_args_boxed);

        assert_eq!(idl_args_to_bytes(ptr, ret_cb, empty_err_cb), StateCode::Ok,);

        idl_args_free(ptr);
    }

    #[test]
    fn idl_args_from_bytes_should_work() {
        let mut ptr = apply_ptr::<IDLArgs>();

        assert_eq!(
            idl_args_from_bytes(
                IDL_ARGS_BYTES.as_ptr(),
                IDL_ARGS_BYTES.len() as c_int,
                &mut ptr,
                empty_err_cb
            ),
            StateCode::Ok
        );

        let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };
        assert_eq!(&IDLArgs::new(&IDL_VALUES), boxed.as_ref());
    }

    #[test]
    fn idl_args_ct_vec_should_work() {
        const IDL_VALUE_LIST: &[*const IDLValue] = &[
            &IDLValue::Bool(true),
            &IDLValue::Null,
            &IDLValue::Principal(Principal::anonymous()),
            &IDLValue::Int32(-12),
        ];

        let mut ptr = apply_ptr::<IDLArgs>();
        idl_args_ct_vec(
            IDL_VALUE_LIST.as_ptr(),
            IDL_VALUE_LIST.len() as c_int,
            &mut ptr,
        );

        let boxed = unsafe { Box::from_raw(ptr as *mut IDLArgs) };
        for (i, v) in boxed.args.iter().enumerate() {
            let e = unsafe { &*IDL_VALUE_LIST[i] };
            assert_eq!(e, v);
        }
    }

    #[test]
    fn idl_args_as_vec_should_work() {
        extern "C" fn ret_cb(data: *const *const IDLValue, len: c_int) {
            for i in 0..len as usize {
                unsafe {
                    let val_ptr = *data.offset(i as isize);
                    let idl_value = Box::from_raw(val_ptr as *mut IDLValue);
                    assert_eq!(&IDL_VALUES[i], idl_value.deref());
                }
            }
        }

        let idl_args = IDLArgs::new(&IDL_VALUES);

        let idl_args_boxed = Box::new(idl_args);
        let ptr = Box::into_raw(idl_args_boxed);

        idl_args_as_vec(ptr, ret_cb);

        idl_args_free(ptr);
    }
}
