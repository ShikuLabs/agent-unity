use crate::CopyTo;
use crate::{AnyErr, StateCode};
use ic_agent::identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity};
use ic_agent::Identity as TIdentity;
use k256::elliptic_curve::rand_core::OsRng;
use k256::SecretKey;
use libc::c_char;
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use std::ffi::CStr;

impl<I: TIdentity + 'static> CopyTo<*mut *const dyn TIdentity> for I {
    fn copy_to(self, out_fptr: *mut *const dyn TIdentity) -> StateCode {
        let boxed: Box<dyn TIdentity> = Box::new(self);
        let raw = Box::into_raw(boxed);

        unsafe {
            *out_fptr = raw;
        }

        StateCode::Ok
    }
}

#[no_mangle]
pub extern "C" fn identity_anonymous(out_fptr: *mut *const dyn TIdentity) -> StateCode {
    AnonymousIdentity {}.copy_to(out_fptr)
}

#[no_mangle]
pub extern "C" fn identity_basic_random(
    out_fptr: *mut *const dyn TIdentity,
    out_err_info: *mut c_char,
    err_info_size: u32,
) -> StateCode {
    let rng = SystemRandom::new();

    let basic = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(AnyErr::from)
        .and_then(|pkcs8| Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).map_err(AnyErr::from))
        .map(BasicIdentity::from_key_pair);

    match basic {
        Ok(basic) => basic.copy_to(out_fptr),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem_file(
    path: *const c_char,
    out_fptr: *mut *const dyn TIdentity,
    out_err_info: &mut c_char,
    err_info_size: u32,
) -> StateCode {
    let path = unsafe { CStr::from_ptr(path).to_str().map_err(AnyErr::from) };

    let basic = path.and_then(|path| BasicIdentity::from_pem_file(path).map_err(AnyErr::from));

    match basic {
        Ok(basic) => basic.copy_to(out_fptr),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem(
    pem: *const c_char,
    out_fptr: *mut *const dyn TIdentity,
    out_err_info: &mut c_char,
    err_info_size: u32,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let basic = pem.and_then(|pem| BasicIdentity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    match basic {
        Ok(basic) => basic.copy_to(out_fptr),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_random(out_fptr: *mut *const dyn TIdentity) -> StateCode {
    let secret_key = SecretKey::random(OsRng);
    let secp256k1 = Secp256k1Identity::from_private_key(secret_key);

    secp256k1.copy_to(out_fptr)
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_from_pem_file(
    path: *const c_char,
    out_fptr: *mut *const dyn TIdentity,
    out_err_info: &mut c_char,
    err_info_size: u32,
) -> StateCode {
    let path = unsafe { CStr::from_ptr(path).to_str().map_err(AnyErr::from) };

    let secp256k1 =
        path.and_then(|path| Secp256k1Identity::from_pem_file(path).map_err(AnyErr::from));

    match secp256k1 {
        Ok(secp256k1) => secp256k1.copy_to(out_fptr),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_from_pem(
    pem: *const c_char,
    out_fptr: *mut *const dyn TIdentity,
    out_err_info: &mut c_char,
    err_info_size: u32,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let basic =
        pem.and_then(|pem| Secp256k1Identity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    match basic {
        Ok(secp256k1) => secp256k1.copy_to(out_fptr),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_sender(
    fptr: *const *mut dyn TIdentity,
    out_arr: *mut u8,
    out_arr_len: *mut u32,
    arr_size: u32,
    out_err_info: *mut c_char,
    err_info_size: u32,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(*fptr) };

    let sc = match boxed.sender() {
        Ok(principal) => principal.copy_to((out_arr, out_arr_len, arr_size)),
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    };

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    sc
}

#[no_mangle]
pub extern "C" fn identity_sign(
    fptr: *const *mut dyn TIdentity,
    bytes: *const u8,
    bytes_len: u32,
    out_public_key: *mut u8,
    out_public_key_len: *mut u32,
    public_key_size: u32,
    out_signature: *mut u8,
    out_signature_len: *mut u32,
    signature_size: u32,
    out_err_info: *mut c_char,
    err_info_size: u32,
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

            let sc_a = public_key.copy_to((out_public_key, out_public_key_len, public_key_size));
            let sc_b = signature.copy_to((out_signature, out_signature_len, signature_size));

            if sc_a == StateCode::Ok && sc_b == StateCode::Ok {
                StateCode::Ok
            } else if sc_a != StateCode::Ok && sc_b == StateCode::Ok {
                "The data of `public-key` copied to the memory allocated outside overflows."
                    .copy_to((out_err_info, err_info_size))
            } else if sc_a == StateCode::Ok && sc_b != StateCode::Ok {
                "The data of `signature` copied to the memory allocated outside overflows."
                    .copy_to((out_err_info, err_info_size))
            } else {
                "Both `public-key` and `signature` copied to the memory allocated outside are overflowed."
                    .copy_to((out_err_info, err_info_size))
            }
        }
        Err(err) => err.copy_to((out_err_info, err_info_size)),
    }
}

#[no_mangle]
pub extern "C" fn identity_free(fptr: *const *mut dyn TIdentity) {
    let boxed = unsafe { Box::from_raw(*fptr) };

    drop(boxed);
}
