use anyhow::{anyhow, bail, Context};
use candid::parser::value::IDLValue;
use candid::types::{Function, Type};
use candid::{check_prog, CandidType, Decode, Deserialize, IDLArgs, IDLProg, Principal, TypeEnv};
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::AnonymousIdentity;
use ic_agent::{Agent, Identity};
use ic_utils::interfaces::management_canister::builders::{CanisterInstall, CanisterSettings};
use ic_utils::interfaces::management_canister::MgmtMethod;
use std::str::FromStr;
use std::sync::Arc;

pub const IC_MAIN_NET: &str = "https://ic0.app";

const II_CANISTER_ID: &'static str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
const II_CANDID_FILE: &'static str = include_str!("rdmx6-jaaaa-aaaaa-aaadq-cai.did");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Init variable
    let canister_id = Principal::from_str(II_CANISTER_ID)?;
    let caller = Arc::new(AnonymousIdentity {});

    let fut = query_sync(
        &canister_id,
        II_CANDID_FILE,
        "lookup",
        "(1974211: nat64)",
        caller.clone(),
    );
    let rst_idl = tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(fut))?;
    println!("{}", rst_idl);

    let fut = update_sync(
        &canister_id,
        II_CANDID_FILE,
        "create_challenge",
        "()",
        caller,
    );
    let rst_idl = tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(fut))?;
    println!("{}", rst_idl);

    Ok(())
}

async fn query_sync(
    canister_id: &Principal,
    candid_file: &str,
    method_name: &str,
    args_raw: &str,
    caller: Arc<dyn Identity>,
) -> anyhow::Result<IDLArgs> {
    let (ty_env, actor) = check_candid_file(candid_file)?;

    let method_sig = get_method_signature(&ty_env, &actor, method_name)?;
    let args_blb = blob_from_args(args_raw, &ty_env, &method_sig)?;

    let effective_canister_id =
        get_effective_canister_id(method_name, args_blb.as_slice(), &canister_id)?;

    let agent = Agent::builder()
        .with_transport(ReqwestHttpReplicaV2Transport::create(IC_MAIN_NET)?)
        .with_arc_identity(caller)
        .build()?;

    let mut query_builder = agent.query(&canister_id, method_name);
    let fut = query_builder
        .with_arg(args_blb)
        .with_effective_canister_id(effective_canister_id)
        .call();

    let rst_blb = fut.await?;

    let rst_idl = args_from_blob(rst_blb.as_slice(), &ty_env, &method_sig)?;

    Ok(rst_idl)
}

async fn update_sync(
    canister_id: &Principal,
    candid_file: &str,
    method_name: &str,
    args_raw: &str,
    caller: Arc<dyn Identity>,
) -> anyhow::Result<IDLArgs> {
    let (ty_env, actor) = check_candid_file(candid_file)?;

    let method_sig = get_method_signature(&ty_env, &actor, method_name)?;
    let args_blb = blob_from_args(args_raw, &ty_env, &method_sig)?;

    let effective_canister_id =
        get_effective_canister_id(method_name, args_blb.as_slice(), &canister_id)?;

    let agent = Agent::builder()
        .with_transport(ReqwestHttpReplicaV2Transport::create(IC_MAIN_NET)?)
        .with_arc_identity(caller)
        .build()?;

    let mut update_builder = agent.update(&canister_id, method_name);
    let fut = update_builder
        .with_arg(args_blb)
        .with_effective_canister_id(effective_canister_id)
        .call_and_wait(
            garcon::Delay::builder()
                .timeout(std::time::Duration::from_secs(60 * 5))
                .build(),
        );

    let rst_blb = fut.await?;

    let rst_idl = args_from_blob(rst_blb.as_slice(), &ty_env, &method_sig)?;

    Ok(rst_idl)
}

fn check_candid_file(idl_file: &str) -> anyhow::Result<(TypeEnv, Option<Type>)> {
    let ast = idl_file
        .parse::<IDLProg>()
        .with_context(|| format!("Failed to parse the Candid file: {}", idl_file))?;
    let mut env = TypeEnv::new();
    let actor = check_prog(&mut env, &ast)
        .with_context(|| format!("Failed to type check the Candid file: {}", idl_file))?;
    Ok((env, actor))
}

fn get_method_signature(
    ty_env: &TypeEnv,
    actor: &Option<Type>,
    method_name: &str,
) -> anyhow::Result<Function> {
    match actor {
        Some(actor) => Ok(ty_env.get_method(&actor, method_name)?.clone()),
        None => Err(anyhow!("Failed to get method: {}", method_name)),
    }
}

fn blob_from_args(args: &str, ty_env: &TypeEnv, meth_sig: &Function) -> anyhow::Result<Vec<u8>> {
    let first_char = args.chars().next();
    let is_candid_format = first_char.map_or(false, |c| c == '(');

    let args_idl = args
        .parse::<IDLArgs>()
        .or_else(|e| {
            if meth_sig.args.len() == 1 && !is_candid_format {
                let is_quote = first_char.map_or(false, |c| c == '"');
                if Type::Text == meth_sig.args[0] && !is_quote {
                    Ok(IDLValue::Text(args.into()))
                } else {
                    args.parse::<IDLValue>()
                }
                .map(|v| IDLArgs::new(&[v]))
            } else {
                Err(e)
            }
        })
        .context(format!(
            "Failed to parse arguments \"{}\" with method signature {}",
            args, meth_sig
        ))?;

    let args_blob = args_idl
        .to_bytes_with_types(ty_env, &meth_sig.args)
        .context(format!(
            "Failed to serialize candid values from \"{}\" to {}",
            args, meth_sig
        ))?;

    Ok(args_blob)
}

fn args_from_blob(blob: &[u8], ty_env: &TypeEnv, meth_sig: &Function) -> anyhow::Result<IDLArgs> {
    let args_idl = IDLArgs::from_bytes_with_types(blob, ty_env, &meth_sig.rets).context(format!(
        "Failed to parse blob \"{}\" with method signature {}",
        hex::encode(blob),
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
