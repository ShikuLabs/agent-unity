use crate::{ret_fat_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use ic_agent::identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity};
use ic_agent::{Identity, Signature};
use k256::elliptic_curve::rand_core::OsRng;
use k256::SecretKey;
use libc::{c_char, c_int};
use ring::rand::SystemRandom;
use ring::signature::Ed25519KeyPair;
use std::ffi::CStr;
use std::fmt::Display;

#[allow(dead_code)]
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdentityType {
    Anonymous = 0,
    Basic = 1,
    Secp256K1 = 2,
}

#[no_mangle]
pub extern "C" fn identity_anonymous(p2fptr: *mut *const dyn Identity) {
    unsafe {
        ret_fat_ptr(p2fptr, AnonymousIdentity {});
    };
}

#[no_mangle]
pub extern "C" fn identity_basic_random(
    p2fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let rng = SystemRandom::new();

    let identity = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(AnyErr::from)
        .and_then(|pkcs8| Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).map_err(AnyErr::from))
        .map(BasicIdentity::from_key_pair);

    __todo_replace_this_by_macro(p2fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_basic_from_pem(
    pem: *const c_char,
    p2fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let identity =
        pem.and_then(|pem| BasicIdentity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    __todo_replace_this_by_macro(p2fptr, err_cb, identity)
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_random(p2fptr: *mut *const dyn Identity) {
    let secret_key = SecretKey::random(OsRng);
    let identity = Secp256k1Identity::from_private_key(secret_key);

    unsafe {
        ret_fat_ptr(p2fptr, identity);
    };
}

#[no_mangle]
pub extern "C" fn identity_secp256k1_from_pem(
    pem: *const c_char,
    p2fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let pem = unsafe { CStr::from_ptr(pem).to_str().map_err(AnyErr::from) };

    let identity =
        pem.and_then(|pem| Secp256k1Identity::from_pem(pem.as_bytes()).map_err(AnyErr::from));

    __todo_replace_this_by_macro(p2fptr, err_cb, identity)
}

// TODO: Wrap Secp256k1Identity::from_private_key

#[no_mangle]
pub extern "C" fn identity_sender(
    p2fptr: *const *const dyn Identity,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(*p2fptr as *mut dyn Identity) };

    let principal = boxed.sender();

    // keep available the fat pointer to the [`Identity`]
    let _ = Box::into_raw(boxed);

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, principal)
}

#[no_mangle]
pub extern "C" fn identity_sign(
    bytes: *const u8,
    bytes_len: c_int,
    p2fptr: *const *const dyn Identity,
    pub_key_cb: UnsizedCallBack,
    sig_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let boxed = unsafe { Box::from_raw(*p2fptr as *mut dyn Identity) };
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
pub extern "C" fn identity_free(p2fptr: *const *const dyn Identity) {
    let boxed = unsafe { Box::from_raw(*p2fptr as *mut dyn Identity) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro(
    p2fptr: *mut *const dyn Identity,
    err_cb: UnsizedCallBack,
    r: Result<impl Identity + 'static, impl Display>,
) -> StateCode {
    match r {
        Ok(t) => {
            unsafe {
                ret_fat_ptr(p2fptr, t);
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
    use crate::tests_util::{apply_fptr, empty_err_cb};
    use ic_types::Principal;

    const BASIC_IDENTITY_FILE: &'static str = "-----BEGIN PRIVATE KEY-----
MFMCAQEwBQYDK2VwBCIEIL9r4XBKsg4pquYBHY6rgfzuBsvCy89tgqDfDpofXRBP
oSMDIQBCkE1NL4X43clXS1LFauiceiiKW9NhjVTEpU6LpH9Qcw==
-----END PRIVATE KEY-----\0";

    const SECP256K1_IDENTITY_FILE: &str = "-----BEGIN EC PARAMETERS-----
BgUrgQQACg==
-----END EC PARAMETERS-----
-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIAgy7nZEcVHkQ4Z1Kdqby8SwyAiyKDQmtbEHTIM+WNeBoAcGBSuBBAAK
oUQDQgAEgO87rJ1ozzdMvJyZQ+GABDqUxGLvgnAnTlcInV3NuhuPv4O3VGzMGzeB
N3d26cRxD99TPtm8uo2OuzKhSiq6EQ==
-----END EC PRIVATE KEY-----\0";

    #[test]
    fn identity_anonymous_should_work() {
        let mut fptr = apply_fptr::<AnonymousIdentity, _>();

        identity_anonymous(&mut fptr);

        unsafe {
            // Free here!
            let boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert_eq!(boxed.sender(), Ok(Principal::anonymous()));
        }
    }

    #[test]
    fn identity_basic_random_should_work() {
        let mut fptr = apply_fptr::<BasicIdentity, _>();

        assert_eq!(
            identity_basic_random(&mut fptr, empty_err_cb),
            StateCode::Ok,
        );

        unsafe {
            // Free here!
            let boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(boxed.sender().is_ok());
        }
    }

    #[test]
    fn identity_basic_from_pem_should_work() {
        let mut fptr = apply_fptr::<BasicIdentity, _>();

        assert_eq!(
            identity_basic_from_pem(
                BASIC_IDENTITY_FILE.as_ptr() as *const c_char,
                &mut fptr,
                empty_err_cb
            ),
            StateCode::Ok,
        );

        unsafe {
            // Free here!
            let boxed = Box::from_raw(fptr as *mut dyn Identity);
            let basic = BasicIdentity::from_pem(BASIC_IDENTITY_FILE.as_bytes()).unwrap();
            assert_eq!(boxed.sender(), basic.sender());
        }
    }

    #[test]
    fn identity_secp256k1_random_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();

        identity_secp256k1_random(&mut fptr);

        unsafe {
            // Free here!
            let boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(boxed.sender().is_ok());
        }
    }

    #[test]
    fn identity_secp256k1_from_pem_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();

        assert_eq!(
            identity_secp256k1_from_pem(
                SECP256K1_IDENTITY_FILE.as_ptr() as *const c_char,
                &mut fptr,
                empty_err_cb
            ),
            StateCode::Ok,
        );

        unsafe {
            // Free here!
            let boxed = Box::from_raw(fptr as *mut dyn Identity);
            let secp256k1 =
                Secp256k1Identity::from_pem(SECP256K1_IDENTITY_FILE.as_bytes()).unwrap();
            assert_eq!(boxed.sender(), secp256k1.sender());
        }
    }

    #[test]
    fn identity_sender_should_work() {
        const ANONYMOUS_BYTES: [u8; 1] = [4u8];

        let mut fptr = apply_fptr::<AnonymousIdentity, _>();

        identity_anonymous(&mut fptr);

        extern "C" fn ret_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, ANONYMOUS_BYTES);
        }

        assert_eq!(identity_sender(&fptr, ret_cb, empty_err_cb), StateCode::Ok);

        // Free here!
        identity_free(&fptr);
    }

    #[test]
    fn identity_sign_should_work() {
        const EMPTY_BYTES: [u8; 0] = [];
        const PUB_KEY_EXPECTED: [u8; 44] = [
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00, 0x42, 0x90,
            0x4d, 0x4d, 0x2f, 0x85, 0xf8, 0xdd, 0xc9, 0x57, 0x4b, 0x52, 0xc5, 0x6a, 0xe8, 0x9c,
            0x7a, 0x28, 0x8a, 0x5b, 0xd3, 0x61, 0x8d, 0x54, 0xc4, 0xa5, 0x4e, 0x8b, 0xa4, 0x7f,
            0x50, 0x73,
        ];
        const SIGNATURE_EXPECTED: [u8; 64] = [
            0x6d, 0x7a, 0x2f, 0x85, 0xeb, 0x6c, 0xc2, 0x18, 0x80, 0xc8, 0x3d, 0x9b, 0xb1, 0x70,
            0xe2, 0x4b, 0xf5, 0xd8, 0x9a, 0xa9, 0x96, 0x92, 0xb6, 0x89, 0xac, 0x9d, 0xe9, 0x5c,
            0x1e, 0x3e, 0x50, 0xdc, 0x98, 0x12, 0x2f, 0x94, 0x11, 0x2f, 0x6c, 0xc6, 0x6a, 0x0b,
            0xbf, 0xc0, 0x56, 0x5b, 0xdb, 0x87, 0xa9, 0xe2, 0x2c, 0x8e, 0x56, 0x94, 0x56, 0x12,
            0xde, 0xbf, 0x22, 0x4a, 0x3f, 0xdb, 0xf1, 0x03,
        ];

        let basic = BasicIdentity::from_pem(BASIC_IDENTITY_FILE.as_bytes()).unwrap();
        let mut fptr = Box::into_raw(Box::new(basic)) as *const dyn Identity;

        extern "C" fn pub_key_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, PUB_KEY_EXPECTED);
        }

        extern "C" fn sig_cb(data: *const u8, len: c_int) {
            let slice = unsafe { std::slice::from_raw_parts(data, len as usize) };

            assert_eq!(slice, SIGNATURE_EXPECTED);
        }

        assert_eq!(
            identity_sign(
                EMPTY_BYTES.as_ptr(),
                EMPTY_BYTES.len() as c_int,
                &mut fptr,
                pub_key_cb,
                sig_cb,
                empty_err_cb
            ),
            StateCode::Ok
        );

        // Free here!
        identity_free(&fptr);
    }

    #[test]
    fn identity_free_should_work() {
        let mut fptr = apply_fptr::<AnonymousIdentity, _>();

        identity_anonymous(&mut fptr);
        identity_free(&fptr);
    }
}
