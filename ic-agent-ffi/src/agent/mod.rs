use crate::identity::IdentityType;
use crate::{ret_thin_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use anyhow::anyhow;
use ic_agent::identity::{AnonymousIdentity, Identity, Secp256k1Identity};
use ic_types::Principal;
use libc::{c_char, c_int};
use std::ffi::CStr;

pub struct AgentWrapper {
    url: String,
    identity: Box<dyn Identity>,
    canister_id: Principal,
    did_content: String,
}

/// NOTE: Forbid inputting [`BasicIdentity`] because:
///
/// 1. `[BasicIdentity]` doesn't implement [`Clone`] trait because it's inner field.
#[no_mangle]
pub extern "C" fn agent_create(
    // Url points to the ic net
    url: *const c_char,
    // The pointer points to the [`Identity`]
    p2fptr_iden: *const *const dyn Identity,
    // The type of [`Identity`]
    iden_type: IdentityType,
    // The data of [`Principal`]
    canister_id_bytes: *const u8,
    // The length of data of [`Principal`]
    canister_id_bytes_len: c_int,
    // The content of candid of that canister
    did_content: *const c_char,
    // out: A pointer points to the `AgentWrapper`
    p2ptr_agent_wrapped: *mut *const AgentWrapper,
    // The callback used report error information
    err_cb: UnsizedCallBack,
) -> StateCode {
    let once = || -> Result<AgentWrapper, AnyErr> {
        let url = unsafe { CStr::from_ptr(url).to_str().map_err(AnyErr::from) }?.to_string();

        let identity = unsafe {
            match iden_type {
                IdentityType::Anonymous => Box::new(AnonymousIdentity {}) as Box<dyn Identity>,
                IdentityType::Basic => {
                    return Err(anyhow!("Forbid using [`BasicIdentity`]"));
                }
                IdentityType::Secp256K1 => {
                    // downcast
                    let ptr = *p2fptr_iden as *const Secp256k1Identity;
                    let boxed = Box::from_raw(ptr as *mut Secp256k1Identity);
                    let cloned = boxed.clone();

                    // Make pointer to [`Identity`] usable
                    Box::into_raw(boxed);

                    cloned
                }
            }
        };

        let slice = unsafe {
            std::slice::from_raw_parts(canister_id_bytes, canister_id_bytes_len as usize)
        };
        let canister_id = Principal::from_slice(slice);

        let did_content =
            unsafe { CStr::from_ptr(did_content).to_str().map_err(AnyErr::from) }?.to_string();

        Ok(AgentWrapper {
            url,
            identity,
            canister_id,
            did_content,
        })
    };

    match once() {
        Ok(agent_w) => {
            unsafe {
                ret_thin_ptr(p2ptr_agent_wrapped, agent_w);
            }

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string());

            StateCode::Err
        }
    }
}

#[no_mangle]
pub extern "C" fn agent_query() -> StateCode {
    todo!()
}

#[no_mangle]
pub extern "C" fn agent_update() -> StateCode {
    todo!()
}

#[no_mangle]
pub extern "C" fn agent_status() -> StateCode {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::{identity_anonymous, identity_basic_random, identity_secp256k1_random};
    use crate::tests_util::{apply_fptr, apply_ptr};
    use ic_agent::identity::BasicIdentity;
    use ic_types::Principal;
    use libc::c_int;

    const IC_NET_BYTES: &[u8; 16] = b"https://ic0.app\0";
    const CANISTER_ID_BYTES: &[u8; 1] = &[4u8];
    const DID_CONTENT_BYTES: &[u8; 5] = b"todo\0";

    fn cbytes_to_str(cbytes: &[u8]) -> &str {
        let cstr = CStr::from_bytes_with_nul(cbytes).unwrap();
        cstr.to_str().unwrap()
    }

    extern "C" fn empty_err_cb(_data: *const u8, _len: c_int) {}

    #[test]
    fn agent_create_with_anonymous_should_work() {
        let mut fptr = apply_fptr::<AnonymousIdentity, _>();
        identity_anonymous(&mut fptr);
        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Anonymous,
                CANISTER_ID_BYTES.as_ptr(),
                CANISTER_ID_BYTES.len() as c_int,
                DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_err_cb
            ),
            StateCode::Ok
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_w_wrap = Box::from_raw(ptr as *mut AgentWrapper);
            assert_eq!(agent_w_wrap.url, cbytes_to_str(IC_NET_BYTES));
            assert_eq!(agent_w_wrap.identity.sender(), identity_boxed.sender());
            assert_eq!(
                agent_w_wrap.canister_id,
                Principal::from_slice(CANISTER_ID_BYTES)
            );
            assert_eq!(agent_w_wrap.did_content, cbytes_to_str(DID_CONTENT_BYTES));
        }
    }

    #[test]
    fn agent_create_with_secp256k1_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();
        identity_secp256k1_random(&mut fptr);
        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Secp256K1,
                CANISTER_ID_BYTES.as_ptr(),
                CANISTER_ID_BYTES.len() as c_int,
                DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_err_cb
            ),
            StateCode::Ok
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_w_wrap = Box::from_raw(ptr as *mut AgentWrapper);
            assert_eq!(agent_w_wrap.url, cbytes_to_str(IC_NET_BYTES));
            assert_eq!(agent_w_wrap.identity.sender(), identity_boxed.sender());
            assert_eq!(
                agent_w_wrap.canister_id,
                Principal::from_slice(CANISTER_ID_BYTES)
            );
            assert_eq!(agent_w_wrap.did_content, cbytes_to_str(DID_CONTENT_BYTES));
        }
    }

    #[test]
    fn agent_create_with_basic_should_fail() {
        extern "C" fn err_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe { CStr::from_ptr(data as *const i8) };
            let str = c_str.to_str().unwrap();
            assert_eq!(str, "Forbid using [`BasicIdentity`]");
        }

        let mut fptr = apply_fptr::<BasicIdentity, _>();
        identity_basic_random(&mut fptr, empty_err_cb);
        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Basic,
                CANISTER_ID_BYTES.as_ptr(),
                CANISTER_ID_BYTES.len() as c_int,
                DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                err_cb
            ),
            StateCode::Err
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            assert_eq!(ptr as u64, 0u64);
        }
    }
}
