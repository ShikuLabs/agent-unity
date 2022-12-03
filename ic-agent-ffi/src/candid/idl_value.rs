use crate::{ret_thin_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use anyhow::anyhow;
use candid::parser::value::{IDLField, IDLValue};
use libc::c_char;
use std::ffi::CStr;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

// const NOT_MATCH_TYPE: AnyErr

#[no_mangle]
pub extern "C" fn idl_value_to_text(ptr: *const IDLValue, ret_cb: UnsizedCallBack<u8>) {
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
    err_cb: UnsizedCallBack<u8>,
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

    __todo_replace_this_by_macro_unsized(p2ptr, err_cb, idl_value)
}

#[no_mangle]
pub extern "C" fn idl_value_type(ptr: *const IDLValue, ret_cb: UnsizedCallBack<u8>) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let type_str = boxed.value_ty().to_string() + "\0";

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    ret_unsized(ret_cb, type_str);
}

#[no_mangle]
pub extern "C" fn idl_value_as_bool(
    ptr: *const IDLValue,
    ptr_bool: *mut bool,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Bool(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_bool), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_is_null(
    ptr: *const IDLValue,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Null => Ok(()),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(None, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_text(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Text(v) => Ok(v.clone() + "\0"),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_number(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Number(v) => Ok(v.clone() + "\0"),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_float64(
    ptr: *const IDLValue,
    ptr_f64: *mut f64,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Float64(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_f64), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_opt(
    ptr: *const IDLValue,
    p2ptr_opt: *mut *const IDLValue,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Opt(v) => Ok(v.deref().clone()),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_unsized(p2ptr_opt, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_vec(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<*const IDLValue>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Vec(v) => Ok(v.clone()),
        _ => Err(anyhow!("Not match the actual type of value")),
    }
    .map(|mut vec| {
        let mut ptrs = Vec::new();

        for idl_value in vec.drain(..) {
            let boxed = Box::new(idl_value);
            let ptr = Box::into_raw(boxed);

            ptrs.push(ptr as *const IDLValue);
        }

        ptrs
    });

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_record(
    ptr: *const IDLValue,
    ret_cb_01: UnsizedCallBack<*const u8>,
    ret_cb_02: UnsizedCallBack<*const IDLValue>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Record(v) => {
            let mut fields = v.clone();

            let mut ids = Vec::new();
            let mut vals = Vec::new();

            for IDLField { id, val } in fields.drain(..) {
                let id = id.to_string() + "\0";
                ids.push(id);
                vals.push(val);
            }

            Ok((ids, vals))
        }
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    match r {
        Ok((ids, mut vals)) => {
            // Catch all pointers from ids
            let id_ptrs: Vec<*const u8> = ids
                .iter()
                .map(|id| {
                    let slice: &[u8] = id.as_ref();
                    slice.as_ptr()
                })
                .collect();

            let val_ptrs: Vec<*const IDLValue> = vals
                .drain(..)
                .map(|val| {
                    let boxed = Box::new(val);
                    let ptr = Box::into_raw(boxed);

                    ptr as *const IDLValue
                })
                .collect();

            ret_unsized(ret_cb_01, id_ptrs);
            ret_unsized(ret_cb_02, val_ptrs);

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string() + "\0");

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn idl_value_as_variant(
    ptr: *const IDLValue,
    id_cb: UnsizedCallBack<u8>,
    p2ptr_val: *mut *const IDLValue,
    ptr_u64: *mut u64,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Variant(v) => {
            let id = v.0.id.to_string() + "\0";
            let val = v.0.val.clone();
            let code = v.1;

            Ok((id, val, code))
        }
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    match r {
        Ok((id, val, code)) => {
            ret_unsized(id_cb, id);
            unsafe {
                ret_thin_ptr(p2ptr_val, val);
            }
            unsafe {
                *ptr_u64 = code;
            }

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string() + "\0");

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn idl_value_as_principal(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Principal(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_service(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Service(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_func(
    ptr: *const IDLValue,
    ret_cb_01: UnsizedCallBack<u8>,
    ret_cb_02: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Func(p, s) => Ok((*p, s.clone() + "\0")),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    match r {
        Ok((p, s)) => {
            ret_unsized(ret_cb_01, p);
            ret_unsized(ret_cb_02, s);

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string() + "\0");

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn idl_value_is_none(
    ptr: *const IDLValue,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::None => Ok(()),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(None, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_int(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Int(v) => Ok(v.to_string() + "\0"),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_nat(
    ptr: *const IDLValue,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        IDLValue::Nat(v) => Ok(v.to_string() + "\0"),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_nat8(
    ptr: *const IDLValue,
    ptr_u8: *mut u8,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Nat8(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_u8), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_nat16(
    ptr: *const IDLValue,
    ptr_u16: *mut u16,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Nat16(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_u16), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_nat32(
    ptr: *const IDLValue,
    ptr_u32: *mut u32,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Nat32(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_u32), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_nat64(
    ptr: *const IDLValue,
    ptr_u64: *mut u64,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Nat64(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_u64), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_int8(
    ptr: *const IDLValue,
    ptr_i8: *mut i8,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Int8(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_i8), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_int16(
    ptr: *const IDLValue,
    ptr_i16: *mut i16,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Int16(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_i16), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_int32(
    ptr: *const IDLValue,
    ptr_i32: *mut i32,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Int32(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_i32), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_int64(
    ptr: *const IDLValue,
    ptr_i64: *mut i64,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Int64(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_i64), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_as_float32(
    ptr: *const IDLValue,
    ptr_f32: *mut f32,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Float32(v) => Ok(v),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(Some(ptr_f32), err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_is_reserved(
    ptr: *const IDLValue,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    let r = match boxed.as_ref() {
        &IDLValue::Reserved => Ok(()),
        _ => Err(anyhow!("Not match the actual type of value")),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    __todo_replace_this_by_macro_primitive(None, err_cb, r)
}

#[no_mangle]
pub extern "C" fn idl_value_free(ptr: *const IDLValue) {
    let boxed = unsafe { Box::from_raw(ptr as *mut IDLValue) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro_unsized(
    p2ptr: *mut *const IDLValue,
    err_cb: UnsizedCallBack<u8>,
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

pub(crate) fn __todo_replace_this_by_macro_primitive<T>(
    ptr_opt: Option<*mut T>,
    err_cb: UnsizedCallBack<u8>,
    r: Result<T, impl Display>,
) -> StateCode {
    match r {
        Ok(v) => {
            if let Some(ptr) = ptr_opt {
                unsafe {
                    *ptr = v;
                }
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
    use candid::parser::value::VariantValue;
    use candid::types::Label;
    use candid::{Int, Nat};
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

    #[test]
    fn idl_value_as_bool_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Bool(true);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_bool = false;

        assert_eq!(
            idl_value_as_bool(ptr, &mut out_bool, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(true, out_bool);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_is_null_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Null;

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_is_null(ptr, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_text_should_work() {
        const TEXT: &str = "IDL_VALUE_AS_TEXT_SHOULD_WORK";

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(TEXT, str);
        }

        let idl_value = IDLValue::Text(TEXT.to_string());
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_as_text(ptr, ret_cb, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_number_should_work() {
        const NUMBER: &str = "1234567890";

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(NUMBER, str);
        }

        let idl_value = IDLValue::Number(NUMBER.to_string());
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(
            idl_value_as_number(ptr, ret_cb, empty_err_cb),
            StateCode::Ok
        );

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_float64_should_work() {
        const EXPECTED: f64 = 1.0f64;
        const IDL_VALUE: IDLValue = IDLValue::Float64(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_f64 = 0f64;

        assert_eq!(
            idl_value_as_float64(ptr, &mut out_f64, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_f64);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_opt_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Bool(true);

        let idl_value = IDLValue::Opt(Box::new(IDL_VALUE));
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut p2ptr_opt = apply_ptr::<IDLValue>();

        assert_eq!(
            idl_value_as_opt(ptr, &mut p2ptr_opt, empty_err_cb),
            StateCode::Ok
        );

        let idl_value = unsafe { Box::from_raw(p2ptr_opt as *mut IDLValue) };
        assert_eq!(&IDL_VALUE, idl_value.deref());

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_vec_should_work() {
        const IDL_VALUE_LIST: [IDLValue; 3] = [
            IDLValue::Bool(true),
            IDLValue::Principal(Principal::anonymous()),
            IDLValue::Int64(-12),
        ];

        extern "C" fn ret_cb(data: *const *const IDLValue, len: c_int) {
            for i in 0..len as usize {
                unsafe {
                    let val_ptr = *data.offset(i as isize);
                    let idl_value = Box::from_raw(val_ptr as *mut IDLValue);
                    assert_eq!(&IDL_VALUE_LIST[i], idl_value.deref());
                }
            }
        }

        let idl_value_boxed = Box::new(IDLValue::Vec(IDL_VALUE_LIST.to_vec()));
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_as_vec(ptr, ret_cb, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_record_should_work() {
        const KEYS: [&str; 3] = ["Key01", "123", "Key03"];
        const VALS: [IDLValue; 3] = [
            IDLValue::Bool(true),
            IDLValue::Principal(Principal::anonymous()),
            IDLValue::Int64(-12),
        ];

        extern "C" fn ret_cb_01(data: *const *const u8, len: c_int) {
            for i in 0..len as usize {
                unsafe {
                    let id_ptr = *data.offset(i as isize);

                    let c_str = CStr::from_ptr(id_ptr as *const c_char);
                    let str = c_str.to_str().unwrap();
                    assert_eq!(KEYS[i], str);
                }
            }
        }

        extern "C" fn ret_cb_02(data: *const *const IDLValue, len: c_int) {
            for i in 0..len as usize {
                unsafe {
                    let val_ptr = *data.offset(i as isize);
                    let idl_value = Box::from_raw(val_ptr as *mut IDLValue);
                    assert_eq!(&VALS[i], idl_value.deref());
                }
            }
        }

        let idl_value = IDLValue::Record(vec![
            IDLField {
                id: Label::Named("Key01".to_string()),
                val: IDLValue::Bool(true),
            },
            IDLField {
                id: Label::Id(123),
                val: IDLValue::Principal(Principal::anonymous()),
            },
            IDLField {
                id: Label::Named("Key03".to_string()),
                val: IDLValue::Int64(-12),
            },
        ]);
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(
            idl_value_as_record(ptr, ret_cb_01, ret_cb_02, empty_err_cb),
            StateCode::Ok
        );

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_variant_should_work() {
        const ID: &str = "Key";
        const IDL_VALUE: IDLValue = IDLValue::Bool(true);
        const CODE: u64 = 0;

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let id = c_str.to_str().unwrap();

            assert_eq!(ID, id);
        }

        let idl_field = IDLField {
            id: Label::Named(ID.to_string()),
            val: IDL_VALUE,
        };
        let variant_value = VariantValue(Box::new(idl_field), CODE);

        let idl_value = IDLValue::Variant(variant_value);
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut ptr_val = apply_ptr::<IDLValue>();
        let mut out_u64 = 1u64;

        assert_eq!(
            idl_value_as_variant(ptr, ret_cb, &mut ptr_val, &mut out_u64, empty_err_cb),
            StateCode::Ok
        );

        let idl_value = unsafe { Box::from_raw(ptr_val as *mut IDLValue) };
        assert_eq!(&IDL_VALUE, idl_value.deref());
        assert_eq!(CODE, out_u64);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_principal_should_work() {
        const EXPECTED: Principal = Principal::anonymous();
        const IDL_VALUE: IDLValue = IDLValue::Principal(EXPECTED);

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            let principal = Principal::from_slice(slice);
            assert_eq!(EXPECTED, principal);
        }

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(
            idl_value_as_principal(ptr, ret_cb, empty_err_cb),
            StateCode::Ok
        );

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_service_should_work() {
        const EXPECTED: Principal = Principal::anonymous();
        const IDL_VALUE: IDLValue = IDLValue::Service(EXPECTED);

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            let principal = Principal::from_slice(slice);
            assert_eq!(EXPECTED, principal);
        }

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(
            idl_value_as_service(ptr, ret_cb, empty_err_cb),
            StateCode::Ok
        );

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_func_should_work() {
        const EXPECTED_01: Principal = Principal::anonymous();
        const EXPECTED_02: &str = "IDL_VALUE_AS_FUNC_SHOULD_WORK";

        extern "C" fn ret_cb_01(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            let principal = Principal::from_slice(slice);
            assert_eq!(EXPECTED_01, principal);
        }

        extern "C" fn ret_cb_02(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();

            assert_eq!(EXPECTED_02, str);
        }

        let idl_value = IDLValue::Func(EXPECTED_01, EXPECTED_02.to_string());
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(
            idl_value_as_func(ptr, ret_cb_01, ret_cb_02, empty_err_cb),
            StateCode::Ok
        );

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_is_none_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::None;

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_is_none(ptr, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_int_should_work() {
        static EXPECTED: &str = "12345678901234567890";

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let int_str = c_str.to_str().unwrap();
            let rst_int = Int::from_str(int_str).unwrap();

            let expected_int = Int::from_str(EXPECTED).unwrap();

            assert_eq!(expected_int, rst_int);
        }

        let int = Int::from_str(EXPECTED).unwrap();
        let idl_value = IDLValue::Int(int);
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_as_int(ptr, ret_cb, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_nat_should_work() {
        static EXPECTED: &str = "12345678901234567890";

        extern "C" fn ret_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let nat_str = c_str.to_str().unwrap();
            let rst_nat = Nat::from_str(nat_str).unwrap();

            let expected_nat = Nat::from_str(EXPECTED).unwrap();

            assert_eq!(expected_nat, rst_nat);
        }

        let nat = Nat::from_str(EXPECTED).unwrap();
        let idl_value = IDLValue::Nat(nat);
        let idl_value_boxed = Box::new(idl_value);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_as_nat(ptr, ret_cb, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_nat8_should_work() {
        const EXPECTED: u8 = 1u8;
        const IDL_VALUE: IDLValue = IDLValue::Nat8(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_u8 = 0u8;

        assert_eq!(
            idl_value_as_nat8(ptr, &mut out_u8, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_u8);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_nat16_should_work() {
        const EXPECTED: u16 = 1u16;
        const IDL_VALUE: IDLValue = IDLValue::Nat16(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_u16 = 0u16;

        assert_eq!(
            idl_value_as_nat16(ptr, &mut out_u16, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_u16);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_nat32_should_work() {
        const EXPECTED: u32 = 1u32;
        const IDL_VALUE: IDLValue = IDLValue::Nat32(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_u32 = 0u32;

        assert_eq!(
            idl_value_as_nat32(ptr, &mut out_u32, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_u32);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_nat64_should_work() {
        const EXPECTED: u64 = 1u64;
        const IDL_VALUE: IDLValue = IDLValue::Nat64(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_u64 = 0u64;

        assert_eq!(
            idl_value_as_nat64(ptr, &mut out_u64, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_u64);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_int8_should_work() {
        const EXPECTED: i8 = -1i8;
        const IDL_VALUE: IDLValue = IDLValue::Int8(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_i8 = 0;

        assert_eq!(
            idl_value_as_int8(ptr, &mut out_i8, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_i8);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_int16_should_work() {
        const EXPECTED: i16 = -1i16;
        const IDL_VALUE: IDLValue = IDLValue::Int16(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_i16 = 0;

        assert_eq!(
            idl_value_as_int16(ptr, &mut out_i16, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_i16);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_int32_should_work() {
        const EXPECTED: i32 = -1i32;
        const IDL_VALUE: IDLValue = IDLValue::Int32(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_i32 = 0;

        assert_eq!(
            idl_value_as_int32(ptr, &mut out_i32, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_i32);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_int64_should_work() {
        const EXPECTED: i64 = -1i64;
        const IDL_VALUE: IDLValue = IDLValue::Int64(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_i64 = 0;

        assert_eq!(
            idl_value_as_int64(ptr, &mut out_i64, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_i64);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_as_float32_should_work() {
        const EXPECTED: f32 = 1.0f32;
        const IDL_VALUE: IDLValue = IDLValue::Float32(EXPECTED);

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        let mut out_f32 = 0f32;

        assert_eq!(
            idl_value_as_float32(ptr, &mut out_f32, empty_err_cb),
            StateCode::Ok
        );

        assert_eq!(EXPECTED, out_f32);

        idl_value_free(ptr);
    }

    #[test]
    fn idl_value_is_reserved_should_work() {
        const IDL_VALUE: IDLValue = IDLValue::Reserved;

        let idl_value_boxed = Box::new(IDL_VALUE);
        let ptr = Box::into_raw(idl_value_boxed);

        assert_eq!(idl_value_is_reserved(ptr, empty_err_cb), StateCode::Ok);

        idl_value_free(ptr);
    }
}
