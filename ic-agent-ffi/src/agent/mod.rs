use crate::identity::IdentityType;
use crate::{ret_thin_ptr, ret_unsized, AnyErr, AnyResult, StateCode, UnsizedCallBack};
use anyhow::{anyhow, bail, Context};
use candid::types::{Function, Type};
use candid::{check_prog, CandidType, Decode, Deserialize, IDLArgs, IDLProg, TypeEnv};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::{AnonymousIdentity, Identity, Secp256k1Identity};
use ic_agent::Agent;
use ic_types::Principal;
use ic_utils::interfaces::management_canister::builders::{CanisterInstall, CanisterSettings};
use ic_utils::interfaces::management_canister::MgmtMethod;
use libc::{c_char, c_int};
use std::ffi::CStr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime;

pub struct AgentWrapper {
    url: String,
    identity: Arc<dyn Identity>,
    canister_id: Principal,
    did_content: String,
}

impl AgentWrapper {
    pub async fn query(&self, func_name: &str, func_args: &str) -> AnyResult<IDLArgs> {
        let (ty_env, actor) = self.parse_candid_file()?;
        let func_sig = Self::get_method_signature(func_name, &ty_env, &actor)?;

        let args_blb = Self::blob_from_raw(func_args, &ty_env, &func_sig)?;

        let effective_canister_id =
            Self::get_effective_canister_id(func_args, args_blb.as_slice(), &self.canister_id)?;

        let agent = self.create_agent()?;

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

    fn parse_candid_file(&self) -> AnyResult<(TypeEnv, Option<Type>)> {
        let ast = self.did_content.parse::<IDLProg>().map_err(AnyErr::from)?;

        let mut env = TypeEnv::new();
        let actor = check_prog(&mut env, &ast).map_err(AnyErr::from)?;

        Ok((env, actor))
    }

    fn create_agent(&self) -> AnyResult<Agent> {
        let transport = ReqwestHttpReplicaV2Transport::create(&self.url).map_err(AnyErr::from)?;

        Agent::builder()
            .with_transport(transport)
            .with_arc_identity(self.identity.clone())
            .build()
            .map_err(AnyErr::from)
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
                    "Attempted to call an unsupported management canister method: {}",
                    method_name
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
    err_cb: UnsizedCallBack,
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

    match once() {
        Ok(agent_w) => {
            unsafe {
                ret_thin_ptr(p2ptr_agent_w, agent_w);
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
pub extern "C" fn agent_query(
    p2ptr_agent_w: *const *const AgentWrapper,
    func_name: *const c_char,
    func_args: *const c_char,
    ret_cb: UnsizedCallBack,
    err_cb: UnsizedCallBack,
) -> StateCode {
    let once = || -> AnyResult<String> {
        let agent_w = unsafe { Box::from_raw(*p2ptr_agent_w as *mut AgentWrapper) };
        let func_name = unsafe { CStr::from_ptr(func_name).to_str().map_err(AnyErr::from) }?;
        let func_args = unsafe { CStr::from_ptr(func_args).to_str().map_err(AnyErr::from) }?;

        let runtime = runtime::Runtime::new()?;
        let rst_idl = runtime.block_on(agent_w.query(func_name, func_args))?;

        Ok(rst_idl.to_string())
    };

    crate::principal::__todo_replace_this_by_macro(ret_cb, err_cb, once())
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
