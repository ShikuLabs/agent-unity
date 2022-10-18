use crate::{ret_fat_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use ic_agent::identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity};
use ic_agent::{Identity, Signature};
use k256::elliptic_curve::rand_core::OsRng;
use k256::SecretKey;
use libc::c_char;
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use std::ffi::CStr;
use std::fmt::Display;

#[no_mangle]
pub extern "C" fn identity_anonymous(out_fptr: *mut *const dyn Identity) {
    unsafe {
        ret_fat_ptr(out_fptr, AnonymousIdentity {});
    };
}

#[no_mangle]
pub extern "C" fn identity_basic_random(
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let rng = SystemRandom::new();

    let identity = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(AnyErr::from)
        .and_then(|pkcs8| Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).map_err(AnyErr::from))
        .map(BasicIdentity::from_key_pair);

    __todo_replace_this_by_macro(out_fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem_file(
    path: *const c_char,
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let path = unsafe { CStr::from_ptr(path).to_str().map_err(AnyErr::from) };

    let identity = path.and_then(|path| BasicIdentity::from_pem_file(path).map_err(AnyErr::from));

    __todo_replace_this_by_macro(out_fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem(
    pem: *const c_char,
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let identity =
        pem.and_then(|pem| BasicIdentity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    __todo_replace_this_by_macro(out_fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_random(out_fptr: *mut *const dyn Identity) {
    let secret_key = SecretKey::random(OsRng);
    let identity = Secp256k1Identity::from_private_key(secret_key);

    unsafe {
        ret_fat_ptr(out_fptr, identity);
    };
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_from_pem_file(
    path: *const c_char,
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let path = unsafe { CStr::from_ptr(path).to_str().map_err(AnyErr::from) };

    let identity =
        path.and_then(|path| Secp256k1Identity::from_pem_file(path).map_err(AnyErr::from));

    __todo_replace_this_by_macro(out_fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_from_pem(
    pem: *const c_char,
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let identity =
        pem.and_then(|pem| Secp256k1Identity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    __todo_replace_this_by_macro(out_fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_sender(
    fptr: *const *mut dyn Identity,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(*fptr) };

    let principal = boxed.sender();

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, principal)
}

#[no_mangle]
pub extern "C" fn identity_sign(
    fptr: *const *mut dyn Identity,
    bytes: *const u8,
    bytes_len: u32,
    pub_key_cb: UnsizedCallBack,
    sig_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(*fptr) };
    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_len as usize) };

    let signature = boxed.sign(bytes);

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    match signature {
        Ok(Signature {
            public_key,
            signature,
        }) => {
            let public_key = public_key.unwrap_or_default();
            let signature = signature.unwrap_or_default();

            ret_unsized(pub_key_cb, public_key);
            ret_unsized(sig_cb, signature);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, &err);

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn identity_free(fptr: *const *mut dyn Identity) {
    let boxed = unsafe { Box::from_raw(*fptr) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro(
    p2p: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
    r: Result<impl Identity + 'static, impl Display>,
) -> StateCode {
    match r {
        Ok(t) => {
            unsafe {
                ret_fat_ptr(p2p, t);
            }

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string());

            StateCode::Err
        }
    }
}
