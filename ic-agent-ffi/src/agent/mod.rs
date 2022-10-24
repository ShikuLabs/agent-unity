use crate::identity::IdentityType;
use crate::{ret_thin_ptr, ret_unsized, AnyErr, StateCode, UnsizedCallBack};
use anyhow::anyhow;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::{AnonymousIdentity, Identity, Secp256k1Identity};
use ic_agent::Agent;
use libc::c_char;
use std::ffi::CStr;

/// NOTE: Forbid inputting [`BasicIdentity`] because:
///
/// 1. `p2fptr` will be dangling because the creation of [`Agent`] will consume it.
/// 2. `[BasicIdentity]` doesn't implement [`Clone`] trait because it's inner field.
#[no_mangle]
pub extern "C" fn agent_create(
    url: *const c_char,
    p2fptr_iden: *const *const dyn Identity,
    iden_type: IdentityType,
    p2ptr_agent: *mut *const Agent,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let once = || -> Result<Agent, AnyErr> {
        let url = unsafe { CStr::from_ptr(url).to_str().map_err(AnyErr::from) }?;

        let transport = ReqwestHttpReplicaV2Transport::create(url).map_err(AnyErr::from)?;

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

        let agent = Agent::builder()
            .with_transport(transport)
            .with_boxed_identity(identity)
            .build()
            .map_err(AnyErr::from)?;

        Ok(agent)
    };

    match once() {
        Ok(agent) => {
            unsafe {
                ret_thin_ptr(p2ptr_agent, agent);
            }

            StateCode::Ok
        }
        Err(e) => {
            ret_unsized(err_cb, e.to_string());

            StateCode::Err
        }
    }
}

#[cfg(test)]
mod tests {
    use ic_agent::identity::BasicIdentity;
    use super::*;
    use crate::identity::{identity_anonymous, identity_secp256k1_random};
    use crate::tests_util::{apply_fptr, apply_ptr};
    use ic_types::Principal;
    use libc::c_int;

    const MAIN_NET: &[u8; 16] = b"https://ic0.app\0";

    extern "C" fn empty_err_cb(_data: *const u8, _len: c_int) {}

    #[test]
    fn agent_create_with_anonymous_should_work() {
        let url = b"https://ic0.app\0";
        let mut fptr = apply_fptr::<AnonymousIdentity, _>();
        identity_anonymous(&mut fptr);
        let mut ptr = apply_ptr();

        assert_eq!(
            agent_create(
                url.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Anonymous,
                &mut ptr,
                empty_err_cb
            ),
            StateCode::Ok
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_boxed = Box::from_raw(ptr as *mut Agent);
            assert_eq!(agent_boxed.get_principal(), Ok(Principal::anonymous()));
        }
    }

    #[test]
    fn agent_create_with_secp256k1_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();
        identity_secp256k1_random(&mut fptr);

        let mut ptr = apply_ptr::<Agent>();

        assert_eq!(
            agent_create(
                MAIN_NET.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Secp256K1,
                &mut ptr,
                empty_err_cb
            ),
            StateCode::Ok
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());
            //
            let agent_boxed = Box::from_raw(ptr as *mut Agent);
            assert_eq!(agent_boxed.get_principal(), identity_boxed.sender());
        }
    }

    #[test]
    fn agent_create_with_basic_should_fail() {
        extern "C" fn err_cb(data: *const u8, _len: c_int) {
            let c_str = unsafe {CStr::from_ptr(data as *const i8)};
            let str = c_str.to_str().unwrap();
            assert_eq!(str, "Forbid using [`BasicIdentity`]");
        }


        let mut fptr = apply_fptr::<BasicIdentity, _>();
        identity_secp256k1_random(&mut fptr);

        let mut ptr = apply_ptr::<Agent>();

        assert_eq!(
            agent_create(
                MAIN_NET.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Basic,
                &mut ptr,
                err_cb
            ),
            StateCode::Err
        );

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            assert_eq!(ptr as u64, 0);
        }
    }
}
