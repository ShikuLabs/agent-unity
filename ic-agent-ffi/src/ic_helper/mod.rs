use anyhow::{anyhow, bail, Context, Result};
use candid::types::{Function, Type};
use candid::{check_prog, CandidType, Decode, Deserialize, IDLArgs, IDLProg, TypeEnv};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;
use ic_types::Principal;
use ic_utils::interfaces::management_canister::builders::{CanisterInstall, CanisterSettings};
use ic_utils::interfaces::management_canister::MgmtMethod;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;

pub const IC_MAIN_NET: &str = "https://ic0.app";
pub const ONE_HALF_MINUS: Duration = Duration::new(90, 0);

lazy_static! {
    static ref IDL_LOOKUP: Mutex<HashMap<Principal, String>> = Mutex::new(HashMap::new());
}

pub fn register_idl(canister_id: Principal, idl_file: String) -> Result<()> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|mut lookup| {
            if lookup.contains_key(&canister_id) != true {
                lookup.insert(canister_id, idl_file);

                Ok(())
            } else {
                Err(anyhow!("{} has been added already.", canister_id))
            }
        })
}

pub fn remove_idl(canister_id: &Principal) -> Result<Option<String>> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|mut lookup| {
            let candid_file = lookup.remove(canister_id);

            Ok(candid_file)
        })
}

pub fn get_idl(canister_id: &Principal) -> Result<Option<String>> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|lookup| {
            let candid_file = lookup.get(canister_id).cloned();

            Ok(candid_file)
        })
}

pub fn list_idl() -> Result<Vec<Principal>> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|lookup| Ok(lookup.keys().map(|cid| cid.clone()).collect()))
}

pub async fn query(
    caller: &Principal,
    canister_id: &Principal,
    method_name: &str,
    args_raw: &str,
) -> Result<IDLArgs> {
    // 1. parse did file
    let (ty_env, actor) = check_candid_file(canister_id)?;
    // 2. get method signature by method name
    let method_sig = get_method_signature(method_name, &ty_env, &actor)?;
    // 3. parse input arguments to blob
    let args_blb = blob_from_raw(args_raw, &ty_env, &method_sig)?;
    // 4. get effective canister id
    let effective_canister_id =
        get_effective_canister_id(method_name, args_blb.as_slice(), &canister_id)?;
    // 5. create agent
    let agent = create_agent(caller, IC_MAIN_NET)?;
    // 6. construct transaction then call it
    let rst_blb = agent
        .query(&canister_id, method_name)
        .expire_after(ONE_HALF_MINUS)
        .with_arg(args_blb)
        .with_effective_canister_id(effective_canister_id)
        .call()
        .await
        .context(format!(
            "Failed to call {} from canister {}",
            method_name, canister_id
        ))?;
    // 7. deserialize from args_blb to args_idl
    let rst_idl = idl_from_blob(rst_blb.as_slice(), &ty_env, &method_sig)?;

    Ok(rst_idl)
}

pub async fn update() -> Result<IDLArgs> {
    todo!()
}

fn check_candid_file(canister_id: &Principal) -> Result<(TypeEnv, Option<Type>)> {
    let idl_file = IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|guard| match guard.get(canister_id) {
            Some(idl_file) => Ok(idl_file.clone()),
            None => bail!(
                "Failed to find Candid file by {}, did you register before?",
                canister_id
            ),
        })?;

    let ast = idl_file
        .parse::<IDLProg>()
        .with_context(|| format!("Failed to parse the Candid file: {}", canister_id))?;

    let mut env = TypeEnv::new();
    let actor = check_prog(&mut env, &ast)
        .with_context(|| format!("Failed to type check the Candid file: {}", canister_id))?;

    Ok((env, actor))
}

fn get_method_signature(
    method_name: &str,
    ty_env: &TypeEnv,
    actor: &Option<Type>,
) -> Result<Function> {
    match actor {
        Some(actor) => {
            let method_sig = ty_env
                .get_method(&actor, method_name)
                .with_context(|| format!("Failed to get method: {}", method_name))?
                .clone();

            Ok(method_sig)
        }
        None => bail!("Failed to get method: {}", method_name),
    }
}

fn blob_from_raw(args_raw: &str, ty_env: &TypeEnv, meth_sig: &Function) -> Result<Vec<u8>> {
    let args_idl = args_raw.parse::<IDLArgs>().context(format!(
        "Failed to parse args_raw \"{}\" to args_idl",
        args_raw
    ))?;

    let args_blob = args_idl
        .to_bytes_with_types(ty_env, &meth_sig.args)
        .context(format!("Failed to parse args_idl {} to args_blb", args_idl))?;

    Ok(args_blob)
}

fn idl_from_blob(args_blb: &[u8], ty_env: &TypeEnv, meth_sig: &Function) -> Result<IDLArgs> {
    let args_idl =
        IDLArgs::from_bytes_with_types(args_blb, ty_env, &meth_sig.rets).context(format!(
            "Failed to parse blob \"{}\" with method signature {}",
            hex::encode(args_blb),
            meth_sig
        ));

    args_idl
}

fn get_effective_canister_id(
    method_name: &str,
    args_blob: &[u8],
    canister_id: &Principal,
) -> anyhow::Result<Principal> {
    let is_management_canister = Principal::management_canister() == *canister_id;

    if !is_management_canister {
        Ok(canister_id.clone())
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
            MgmtMethod::ProvisionalCreateCanisterWithCycles => Ok(Principal::management_canister()),
            MgmtMethod::UpdateSettings => {
                #[derive(CandidType, Deserialize)]
                struct In {
                    canister_id: Principal,
                    settings: CanisterSettings,
                }
                let in_args =
                    Decode!(args_blob, In).context("Argument is not valid for UpdateSettings")?;
                Ok(in_args.canister_id)
            }
        }
    }
}

// TODO: Maybe there is opportunity to use another place?
fn create_agent(principal: &Principal, ic_net: &str) -> Result<Agent> {
    let identity = super::LOGGED_INFO
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|guard| match guard.get(principal) {
            Some((_, identity)) => Ok(identity.clone()),
            None => bail!(
                "Failed to find login info about {}, did you logged?",
                principal
            ),
        })?;

    let transport = ReqwestHttpReplicaV2Transport::create(ic_net)
        .with_context(|| format!("Failed to create transport from \"{}\"", ic_net))?;

    let agent = Agent::builder()
        .with_transport(transport)
        .with_arc_identity(identity)
        .build()
        .context(format!("Failed to create agent from {}", principal));

    agent
}
