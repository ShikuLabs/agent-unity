use crate::UnsizedCallBack;
use crate::{ret_identity, ret_unsized, AnyErr, StateCode};
use ic_agent::identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity};
use ic_agent::Identity;
use k256::elliptic_curve::rand_core::OsRng;
use k256::SecretKey;
use libc::c_char;
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn identity_anonymous(out_fptr: *mut *const dyn Identity) {
    ret_identity(out_fptr, AnonymousIdentity {});
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

    match identity {
        Ok(iden) => {
            ret_identity(out_fptr, iden);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, err.to_string());

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem_file(
    path: *const c_char,
    out_fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let path = unsafe { CStr::from_ptr(path).to_str().map_err(AnyErr::from) };

    let identity = path.and_then(|path| BasicIdentity::from_pem_file(path).map_err(AnyErr::from));

    match identity {
        Ok(iden) => {
            ret_identity(out_fptr, iden);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, err.to_string());

            StateCode::Err
        }
    }
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

    match identity {
        Ok(iden) => {
            ret_identity(out_fptr, iden);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, err.to_string());

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_random(out_fptr: *mut *const dyn Identity) {
    let secret_key = SecretKey::random(OsRng);
    let identity = Secp256k1Identity::from_private_key(secret_key);

    ret_identity(out_fptr, identity);
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

    match identity {
        Ok(iden) => {
            ret_identity(out_fptr, iden);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, err.to_string());

            StateCode::Err
        }
    }
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

    match identity {
        Ok(iden) => {
            ret_identity(out_fptr, iden);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, err.to_string());

            StateCode::Err
        }
    }
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

    match principal {
        Ok(principal) => {
            ret_unsized(ret_cb, principal);

            StateCode::Ok
        }
        Err(err) => {
            ret_unsized(err_cb, &err);

            StateCode::Err
        }
    }
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
        Ok(sig) => {
            let public_key = sig.public_key.unwrap_or_default();
            let signature = sig.signature.unwrap_or_default();

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
