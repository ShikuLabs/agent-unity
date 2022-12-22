use crate::identity::IdentityType;
use crate::{ret_thin_ptr, ret_unsized, AnyErr, AnyResult, StateCode, UnsizedCallBack};
use anyhow::{anyhow, bail, Context};
use candid::types::{Function, Type};
use candid::{check_prog, CandidType, Decode, Deserialize, IDLArgs, IDLProg, TypeEnv};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::agent::status::Status;
use ic_agent::identity::{AnonymousIdentity, Identity, Secp256k1Identity};
use ic_agent::Agent;
use ic_types::Principal;
use ic_utils::interfaces::management_canister::builders::{CanisterInstall, CanisterSettings};
use ic_utils::interfaces::management_canister::MgmtMethod;
use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime;

#[derive(Clone, Debug)]
pub struct AgentWrapper {
    url: String,
    identity: Arc<dyn Identity>,
    canister_id: Principal,
    did_content: String,
}

impl AgentWrapper {
    pub unsafe fn from_raw(ptr: *mut Self) -> Self {
        let boxed = Box::from_raw(ptr);
        let cloned = boxed.clone();

        // Don't drop the [`AgentWrapper`]
        Box::into_raw(boxed);

        *cloned
    }

    pub async fn query(&self, func_name: &str, func_args: &str) -> AnyResult<IDLArgs> {
        let (ty_env, actor) = self.parse_candid_file()?;
        let func_sig = Self::get_method_signature(func_name, &ty_env, &actor)?;

        let args_blb = Self::blob_from_raw(func_args, &ty_env, &func_sig)?;

        let effective_canister_id =
            Self::get_effective_canister_id(func_args, args_blb.as_slice(), &self.canister_id)?;

        let agent = self.create_agent().await?;

        let rst_blb = agent
            .query(&self.canister_id, func_name)
            .with_arg(args_blb)
            .with_effective_canister_id(effective_canister_id)
            .call()
            .await
            .map_err(AnyErr::from)?;

        let rst_idl = Self::idl_from_blob(rst_blb.as_slice(), &ty_env, &func_sig)?;

        Ok(rst_idl)
    }

    pub async fn update(&self, func_name: &str, func_args: &str) -> AnyResult<IDLArgs> {
        let (ty_env, actor) = self.parse_candid_file()?;
        let func_sig = Self::get_method_signature(func_name, &ty_env, &actor)?;
        let args_blb = Self::blob_from_raw(func_args, &ty_env, &func_sig)?;

        let effective_canister_id =
            Self::get_effective_canister_id(func_args, args_blb.as_slice(), &self.canister_id)?;

        let agent = self.create_agent().await?;

        let rst_blb = agent
            .update(&self.canister_id, func_name)
            .with_arg(args_blb)
            .with_effective_canister_id(effective_canister_id)
            .call_and_wait(
                garcon::Delay::builder()
                    .timeout(std::time::Duration::from_secs(60 * 5))
                    .build(),
            )
            .await
            .map_err(AnyErr::from)?;

        let rst_idl = Self::idl_from_blob(rst_blb.as_slice(), &ty_env, &func_sig)?;

        Ok(rst_idl)
    }

    pub async fn status(&self) -> AnyResult<Status> {
        let agent = self.create_agent().await?;

        agent.status().await.map_err(AnyErr::from)
    }

    fn parse_candid_file(&self) -> AnyResult<(TypeEnv, Option<Type>)> {
        let ast = self.did_content.parse::<IDLProg>().map_err(AnyErr::from)?;

        let mut env = TypeEnv::new();
        let actor = check_prog(&mut env, &ast).map_err(AnyErr::from)?;

        Ok((env, actor))
    }

    async fn create_agent(&self) -> AnyResult<Agent> {
        let transport = ReqwestHttpReplicaV2Transport::create(&self.url).map_err(AnyErr::from)?;

        let agent = Agent::builder()
            .with_transport(transport)
            .with_arc_identity(self.identity.clone())
            .build()
            .map_err(AnyErr::from)?;

        agent.fetch_root_key().await.map_err(AnyErr::from)?;

        Ok(agent)
    }

    fn get_method_signature(
        method_name: &str,
        ty_env: &TypeEnv,
        actor: &Option<Type>,
    ) -> AnyResult<Function> {
        match actor {
            Some(actor) => {
                let method_sig = ty_env
                    .get_method(actor, method_name)
                    .map_err(AnyErr::from)?
                    .clone();

                Ok(method_sig)
            }
            None => bail!("Failed to get method: {}", method_name),
        }
    }

    fn blob_from_raw(args_raw: &str, ty_env: &TypeEnv, meth_sig: &Function) -> AnyResult<Vec<u8>> {
        let args_idl = args_raw.parse::<IDLArgs>().map_err(AnyErr::from)?;

        let args_blob = args_idl
            .to_bytes_with_types(ty_env, &meth_sig.args)
            .map_err(AnyErr::from)?;

        Ok(args_blob)
    }

    fn get_effective_canister_id(
        method_name: &str,
        args_blob: &[u8],
        canister_id: &Principal,
    ) -> anyhow::Result<Principal> {
        let is_management_canister = Principal::management_canister() == *canister_id;

        if !is_management_canister {
            Ok(*canister_id)
        } else {
            let method_name = MgmtMethod::from_str(method_name).with_context(|| {
                format!(
                    "Attempted to call an unsupported management canister method: {method_name}",
                )
            })?;

            match method_name {
                MgmtMethod::CreateCanister | MgmtMethod::RawRand => bail!(
                    "{} can only be called via an inter-canister call.",
                    method_name.as_ref()
                ),
                MgmtMethod::InstallCode => {
                    let install_args = Decode!(args_blob, CanisterInstall)
                        .context("Argument is not valid for CanisterInstall")?;
                    Ok(install_args.canister_id)
                }
                MgmtMethod::StartCanister
                | MgmtMethod::StopCanister
                | MgmtMethod::CanisterStatus
                | MgmtMethod::DeleteCanister
                | MgmtMethod::DepositCycles
                | MgmtMethod::UninstallCode
                | MgmtMethod::ProvisionalTopUpCanister => {
                    #[derive(CandidType, Deserialize)]
                    struct In {
                        canister_id: Principal,
                    }
                    let in_args =
                        Decode!(args_blob, In).context("Argument is not a valid Principal")?;
                    Ok(in_args.canister_id)
                }
                MgmtMethod::ProvisionalCreateCanisterWithCycles => {
                    Ok(Principal::management_canister())
                }
                MgmtMethod::UpdateSettings => {
                    #[derive(CandidType, Deserialize)]
                    struct In {
                        canister_id: Principal,
                        settings: CanisterSettings,
                    }
                    let in_args = Decode!(args_blob, In)
                        .context("Argument is not valid for UpdateSettings")?;
                    Ok(in_args.canister_id)
                }
            }
        }
    }

    fn idl_from_blob(args_blb: &[u8], ty_env: &TypeEnv, meth_sig: &Function) -> AnyResult<IDLArgs> {
        IDLArgs::from_bytes_with_types(args_blb, ty_env, &meth_sig.rets).map_err(AnyErr::from)
    }
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
    p2ptr_agent_w: *mut *const AgentWrapper,
    // The callback used report error information
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let once = || -> AnyResult<AgentWrapper> {
        let url = unsafe { CStr::from_ptr(url).to_str().map_err(AnyErr::from) }?.to_string();

        let identity = unsafe {
            match iden_type {
                IdentityType::Anonymous => Arc::new(AnonymousIdentity {}) as Arc<dyn Identity>,
                IdentityType::Basic => {
                    return Err(anyhow!("Forbid using [`BasicIdentity`]"));
                }
                IdentityType::Secp256K1 => {
                    // downcast
                    let ptr = *p2fptr_iden as *const Secp256k1Identity;
                    let boxed = Box::from_raw(ptr as *mut Secp256k1Identity);
                    let cloned = Arc::<Secp256k1Identity>::from(boxed.clone()) as Arc<dyn Identity>;

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

    __todo_replace_this_by_macro(p2ptr_agent_w, err_cb, once())
}

#[no_mangle]
pub extern "C" fn agent_query(
    ptr_agent_w: *const AgentWrapper,
    func_name: *const c_char,
    func_args: *const c_char,
    p2ptr: *mut *const IDLArgs,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let once = || -> AnyResult<_> {
        let agent_w = unsafe { AgentWrapper::from_raw(ptr_agent_w as *mut AgentWrapper) };
        let func_name = unsafe { CStr::from_ptr(func_name).to_str().map_err(AnyErr::from) }?;
        let func_args = unsafe { CStr::from_ptr(func_args).to_str().map_err(AnyErr::from) }?;

        let runtime = runtime::Runtime::new()?;
        let rst_idl = runtime.block_on(agent_w.query(func_name, func_args))?;

        Ok(rst_idl)
    };

    crate::candid::idl_args::__todo_replace_this_by_macro(p2ptr, err_cb, once())
}

#[no_mangle]
pub extern "C" fn agent_update(
    ptr_agent_w: *const AgentWrapper,
    func_name: *const c_char,
    func_args: *const c_char,
    p2ptr: *mut *const IDLArgs,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let once = || -> AnyResult<_> {
        let agent_w = unsafe { AgentWrapper::from_raw(ptr_agent_w as *mut AgentWrapper) };
        let func_name = unsafe { CStr::from_ptr(func_name).to_str().map_err(AnyErr::from) }?;
        let func_args = unsafe { CStr::from_ptr(func_args).to_str().map_err(AnyErr::from) }?;

        let runtime = runtime::Runtime::new()?;
        let rst_idl = runtime.block_on(agent_w.update(func_name, func_args))?;

        Ok(rst_idl)
    };

    crate::candid::idl_args::__todo_replace_this_by_macro(p2ptr, err_cb, once())
}

#[no_mangle]
pub extern "C" fn agent_status(
    ptr_agent_w: *const AgentWrapper,
    ret_cb: UnsizedCallBack<u8>,
    err_cb: UnsizedCallBack<u8>,
) -> StateCode {
    let once = || -> AnyResult<_> {
        let agent_w = unsafe { Box::from_raw(ptr_agent_w as *mut AgentWrapper) };

        let runtime = runtime::Runtime::new()?;
        let status = runtime.block_on(agent_w.status())?;

        // Don't drop the [`AgentWrapper`]
        Box::into_raw(agent_w);

        let status_cstr = CString::new(status.to_string())
            .map_err(AnyErr::from)?
            .into_bytes_with_nul();

        Ok(status_cstr)
    };

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, once())
}

#[no_mangle]
pub extern "C" fn agent_free(ptr_agent_w: *const AgentWrapper) {
    let boxed = unsafe { Box::from_raw(ptr_agent_w as *mut AgentWrapper) };

    drop(boxed);
}

pub(crate) fn __todo_replace_this_by_macro(
    p2ptr: *mut *const AgentWrapper,
    err_cb: UnsizedCallBack<u8>,
    r: Result<AgentWrapper, impl Display>,
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
    use crate::identity::{identity_anonymous, identity_basic_random, identity_secp256k1_random};
    use crate::tests_util::{apply_fptr, apply_ptr, panic_err_cb};
    use ic_agent::identity::BasicIdentity;
    use ic_types::Principal;
    use libc::c_int;

    const IC_NET_BYTES: &[u8] = b"https://ic0.app\0";

    const II_CANISTER_ID_BYTES: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 7, 1, 1];
    const II_DID_CONTENT_BYTES: &[u8] =
        concat_bytes!(include_bytes!("rdmx6-jaaaa-aaaaa-aaadq-cai.did"), b"\0");

    fn cbytes_to_str(cbytes: &[u8]) -> &str {
        let cstr = CStr::from_bytes_with_nul(cbytes).unwrap();
        cstr.to_str().unwrap()
    }

    extern "C" fn empty_cb(_data: *const u8, _len: c_int) {}

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
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
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
                Principal::from_slice(II_CANISTER_ID_BYTES)
            );
            assert_eq!(
                agent_w_wrap.did_content,
                cbytes_to_str(II_DID_CONTENT_BYTES)
            );
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
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
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
                Principal::from_slice(II_CANISTER_ID_BYTES)
            );
            assert_eq!(
                agent_w_wrap.did_content,
                cbytes_to_str(II_DID_CONTENT_BYTES)
            );
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
        identity_basic_random(&mut fptr, empty_cb);
        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Basic,
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
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

    #[test]
    fn agent_query_should_work() {
        const EXPECTED: &str = r#"(
  vec {
    record {
      alias = "macbook-2021";
      pubkey = blob "0^0\0c\06\0a+\06\01\04\01\83\b8C\01\01\03N\00\a5\01\02\03& \01!X Q\bf\c1O\11\feX\a1\1d\1a\1a|$\be\15>\12\dc/|v\bc)\db#\14\a0pM!\fdf\22X V\ac\d0t\02c\15\e7\fd\edS\ed?K\a7r\86\86K\f9\06\9a\c7\04I\15\a3\f4\00-\a6\93";
      key_type = variant { unknown };
      purpose = variant { authentication };
      credential_id = opt blob "\0c\d6\e3\cd\8a\ad\07\e6\95\e9\08j\90\c6.\0d\b0\d8\cc\db\f6\c7\18l\ba\1aM\c9\8b\a8\12\c8%\d2\af\12\bc\0a\cd\b1\08\9d\af\e6\f1\9c\a0Lq\b0\a2\e9-\12\cc\8a\c1\ad%\b1P\b6\f8@+_\a9\223\af\07\0d\1d\cfv\9b\0a\80\fd\8a\abE\c5";
    };
  },
)"#;

        let mut fptr = apply_fptr::<Secp256k1Identity, _>();
        identity_secp256k1_random(&mut fptr);

        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Secp256K1,
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
            ),
            StateCode::Ok
        );

        let mut idl_ptr = apply_ptr::<IDLArgs>();

        assert_eq!(
            agent_query(
                ptr,
                b"lookup\0".as_ptr() as *const c_char,
                b"(1974211: nat64)\0".as_ptr() as *const c_char,
                &mut idl_ptr,
                panic_err_cb,
            ),
            StateCode::Ok
        );

        unsafe {
            let idl_boxed = Box::from_raw(idl_ptr as *mut IDLArgs);
            assert_eq!(EXPECTED, idl_boxed.to_string());

            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_w_wrap = Box::from_raw(ptr as *mut AgentWrapper);
            assert_eq!(agent_w_wrap.url, cbytes_to_str(IC_NET_BYTES));
            assert_eq!(agent_w_wrap.identity.sender(), identity_boxed.sender());
            assert_eq!(
                agent_w_wrap.canister_id,
                Principal::from_slice(II_CANISTER_ID_BYTES)
            );
            assert_eq!(
                agent_w_wrap.did_content,
                cbytes_to_str(II_DID_CONTENT_BYTES)
            );
        }
    }

    #[test]
    fn agent_update_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();
        identity_secp256k1_random(&mut fptr);

        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Secp256K1,
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
            ),
            StateCode::Ok
        );

        let mut idl_ptr = apply_ptr::<IDLArgs>();

        assert_eq!(
            agent_update(
                ptr,
                b"create_challenge\0".as_ptr() as *const c_char,
                b"()\0".as_ptr() as *const c_char,
                &mut idl_ptr,
                panic_err_cb,
            ),
            StateCode::Ok
        );

        unsafe {
            let idl_boxed = Box::from_raw(idl_ptr as *mut IDLArgs);
            assert!(idl_boxed.to_string().contains("png_base64"));

            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_w_wrap = Box::from_raw(ptr as *mut AgentWrapper);
            assert_eq!(agent_w_wrap.url, cbytes_to_str(IC_NET_BYTES));
            assert_eq!(agent_w_wrap.identity.sender(), identity_boxed.sender());
            assert_eq!(
                agent_w_wrap.canister_id,
                Principal::from_slice(II_CANISTER_ID_BYTES)
            );
            assert_eq!(
                agent_w_wrap.did_content,
                cbytes_to_str(II_DID_CONTENT_BYTES)
            );
        }
    }

    #[test]
    fn agent_status_should_work() {
        let mut fptr = apply_fptr::<Secp256k1Identity, _>();
        identity_secp256k1_random(&mut fptr);

        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Secp256K1,
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
            ),
            StateCode::Ok
        );

        assert_eq!(agent_status(ptr, empty_cb, panic_err_cb), StateCode::Ok);

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());

            let agent_w_wrap = Box::from_raw(ptr as *mut AgentWrapper);
            assert_eq!(agent_w_wrap.url, cbytes_to_str(IC_NET_BYTES));
            assert_eq!(agent_w_wrap.identity.sender(), identity_boxed.sender());
            assert_eq!(
                agent_w_wrap.canister_id,
                Principal::from_slice(II_CANISTER_ID_BYTES)
            );
            assert_eq!(
                agent_w_wrap.did_content,
                cbytes_to_str(II_DID_CONTENT_BYTES)
            );
        }
    }

    #[test]
    fn agent_free_should_work() {
        let mut fptr = apply_fptr::<AnonymousIdentity, _>();
        identity_anonymous(&mut fptr);
        let mut ptr = apply_ptr::<AgentWrapper>();

        assert_eq!(
            agent_create(
                IC_NET_BYTES.as_ptr() as *const c_char,
                &mut fptr,
                IdentityType::Anonymous,
                II_CANISTER_ID_BYTES.as_ptr(),
                II_CANISTER_ID_BYTES.len() as c_int,
                II_DID_CONTENT_BYTES.as_ptr() as *const c_char,
                &mut ptr,
                empty_cb
            ),
            StateCode::Ok
        );

        agent_free(ptr);

        unsafe {
            let identity_boxed = Box::from_raw(fptr as *mut dyn Identity);
            assert!(identity_boxed.sender().is_ok());
        }
    }
}
