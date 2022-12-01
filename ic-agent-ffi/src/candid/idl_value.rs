use crate::{ret_thin_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use candid::parser::value::IDLValue;
use libc::c_char;
use std::ffi::CStr;
use std::fmt::Display;
use std::str::FromStr;

#[no_mangle]
pub extern "C" fn idl_value_to_text(ptr: *const IDLValue, ret_cb: UnsizedCallBack) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let idl_str = boxed.to_string() + "\0";

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    ret_unsized(ret_cb, idl_str);
}

#[no_mangle]
pub extern "C" fn idl_value_from_text(
    text: *const c_char,
    p2ptr: *mut *const IDLValue,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let text = unsafe { CStr::from_ptr(text).to_str().map_err(AnyErr::from) };

    // Try to recoup brackets
    let text = text.map(|str| {
        if !str.starts_with('(') && !str.ends_with(')') {
            format!("({str})")
        } else {
            str.to_string()
        }
    });

    let idl_value = text.and_then(|text| IDLValue::from_str(&text).map_err(AnyErr::from));

    __todo_replace_this_by_macro(p2ptr, err_cb, idl_value)
}

#[no_mangle]
pub extern "C" fn idl_value_type(ptr: *const IDLValue, ret_cb: UnsizedCallBack) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let type_str = boxed.value_ty().to_string() + "\0";

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    ret_unsized(ret_cb, type_str);
}

#[no_mangle]
pub extern "C" fn idl_value_free(ptr: *const IDLValue) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro(
    p2ptr: *mut *const IDLValue,
    err_cb: UnsizedCallBack,
    r: Result<IDLValue, impl Display>,
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
    use crate::tests_util::apply_ptr;
    use ic_types::Principal;
    use libc::c_int;

    extern "C" fn empty_cb(_data: *const u8, _len: c_int) {}

    #[test]
    fn idl_value_to_text_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Principal(Principal::anonymous());
        const EXPECTED: &str = r#"principal "2vxsx-fae""#;

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(EXPECTED, str);
        }

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        idl_value_to_text(ptr, ret_cb);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_from_text_should_work() {
        const IDL_VALUE_TEXTS: &[&[u8]] = &[
            b"128 : nat64\0",
            b"(128 : nat64)\0",
            b"principal \"2vxsx-fae\"\0",
            b"(principal \"2vxsx-fae\")\0",
        ];
        const EXPECTEDS: [IDLValue; 4] = [
            IDLValue::Nat64(128),
            IDLValue::Nat64(128),
            IDLValue::Principal(Principal::anonymous()),
            IDLValue::Principal(Principal::anonymous()),
        ];

        let mut ptr = apply_ptr::<IDLValue>();

        for (i, idl_value_text) in IDL_VALUE_TEXTS.iter().enumerate() {
            assert_eq!(
                idl_value_from_text(idl_value_text.as_ptr() as *const c_char, &mut ptr, empty_cb),
                StateCode::Ok
            );

            let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };
            assert_eq!(&EXPECTEDS[i], boxed.as_ref());
        }
    }

    #[test]
    fn idl_value_from_text_should_fail() {
        const IDL_VALUE_TEXTS: &[&[u8]] = &[
            b"(128 : nat64\0",
            b"128 : nat64)\0",
            b"(principal \"2vxsx-fae\"\0",
            b"(principal \"2vxsx-fae\"\0",
        ];

        let mut ptr = apply_ptr::<IDLValue>();

        for idl_value_text in IDL_VALUE_TEXTS {
            assert_eq!(
                idl_value_from_text(idl_value_text.as_ptr() as *const c_char, &mut ptr, empty_cb),
                StateCode::Err
            );
        }
    }

    #[test]
    fn idl_value_type_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Principal(Principal::anonymous());
        const EXPECTED: &str = "principal";

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(EXPECTED, str);
        }

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        idl_value_type(ptr, ret_cb);

        idl_value_free(ptr);
    }
}
