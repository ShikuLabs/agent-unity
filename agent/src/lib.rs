use crate::host::HostKeyStore;
use crate::ic_helper::{get_idl, list_idl, query, register_idl, remove_idl};
use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use ic_agent::Identity;
use ic_types::Principal;
use lazy_static::lazy_static;
use libc::c_char;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::{runtime, task};

mod host;
mod ic_helper;

type LPSTR = *mut c_char;
type LPCSTR = *const c_char;
type JSON = LPCSTR;
type Request = JSON;

lazy_static! {
    static ref LOGGED_INFO: Mutex<HashMap::<Principal, (LoggedReceipt, Arc<dyn Identity>)>> =
        Mutex::new(HashMap::new());
}

#[repr(C)]
pub struct Response {
    pub ptr: LPSTR,
    pub is_err: bool,
}

impl Response {
    pub fn new<T: Into<String>>(data: T, is_err: bool) -> Self {
        // NOTE: Should be panic if is err!
        let data = CString::new(data.into()).unwrap();
        let ptr = data.into_raw();

        Self { ptr, is_err }
    }
}

impl<T, E> From<Result<T, E>> for Response
where
    T: Into<String>,
    E: Display,
{
    fn from(result: Result<T, E>) -> Self {
        result.map_or_else(
            |e| Response::new(e.to_string(), true),
            |str| Response::new(str, false),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LoggedType {
    II = 0,
    Host = 1,
    Ext = 2,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedReceipt {
    pub principal: Principal,
    pub deadline: DateTime<Utc>,
    pub logged_type: LoggedType,
}

impl LoggedReceipt {
    pub fn new(principal: Principal, deadline: DateTime<Utc>, logged_type: LoggedType) -> Self {
        Self {
            deadline,
            principal,
            logged_type,
        }
    }
}

#[no_mangle]
pub extern "C" fn create_keystore(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| match (&val["name"], &val["password"]) {
            (Value::String(name), Value::String(password)) => Ok((name.clone(), password.clone())),
            _ => Err(anyhow!(
                r#"please input {{"name": .., "password": ..}} as arguments rather than {}"#,
                val
            )),
        })
        .and_then(|(name, password)| HostKeyStore::random(&name, &password))
        .and_then(|keystore| serde_json::to_string(&keystore).map_err(|e| e.into()))
        .into();

    rsp
}

/// Login by host;
///
/// Request Json:
///
/// {
///     "keystore": ..,
///     "password": ..
/// }
#[no_mangle]
pub extern "C" fn login_by_host(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| match (&val["keyStore"], &val["password"]) {
            (keystore, Value::String(password)) => {
                serde_json::from_value::<HostKeyStore>(keystore.clone())
                    .map(|keystore| (keystore, password.clone()))
                    .map_err(|e| e.into())
            }
            _ => Err(anyhow!(
                r#"please input {{"keystore": .., "password": ..}} as aguments rahter than {}"#,
                val
            )),
        })
        .and_then(|(keystore, password)| keystore.to_identity(password.as_str()))
        .and_then(|identity| {
            // Well, dead right :)
            let principal = identity.sender().unwrap();
            let receipt = LoggedReceipt::new(principal, Utc::now(), LoggedType::Host);

            LOGGED_INFO
                .lock()
                .map(|logged_info| (logged_info, receipt, identity))
                .map_err(|e| anyhow!(e.to_string()))
        })
        .and_then(|(mut guard, receipt, identity)| {
            match guard.insert(
                receipt.principal.clone(),
                (receipt.clone(), Arc::new(identity)),
            ) {
                Some(_) => Err(anyhow!(
                    "the account {} has been logged already",
                    receipt.principal
                )),
                _ => serde_json::to_string(&receipt).map_err(|e| e.into()),
            }
        })
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn get_logged_receipt(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| {
            serde_json::from_value::<Principal>(val["principal"].clone()).map_err(|e| e.into())
        })
        .and_then(|principal| {
            LOGGED_INFO
                .lock()
                .map(|logged_info| (logged_info, principal))
                .map_err(|e| anyhow!(e.to_string()))
        })
        .and_then(|(guard, principal)| match guard.get(&principal) {
            Some((receipt, _)) => serde_json::to_string(receipt).map_err(|e| e.into()),
            _ => Err(anyhow!(
                r#"cannot find the logged info by principal: {}"#,
                principal
            )),
        })
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn list_logged_receipt() -> Response {
    let rsp = LOGGED_INFO
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|guard| {
            let list: Vec<_> = guard.iter().map(|(_, (receipt, _))| receipt).collect();
            serde_json::to_string(list.as_slice()).map_err(|e| e.into())
        })
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn logout(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| {
            serde_json::from_value::<Principal>(val["principal"].clone()).map_err(|e| e.into())
        })
        .and_then(|principal| {
            LOGGED_INFO
                .lock()
                .map(|logged_info| (logged_info, principal))
                .map_err(|e| anyhow!(e.to_string()))
        })
        .and_then(|(mut guard, principal)| match guard.remove(&principal) {
            Some(_) => Ok("success"),
            _ => Err(anyhow!(r#"cannot logout by principal: {}"#, principal)),
        })
        .into();

    rsp
}

/// Register idl file
///
/// Request Json:
///
/// {
///     "canisterId": ..,
///     "idlContent": ..
/// }
#[no_mangle]
pub extern "C" fn ic_register_idl(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| {
            let canister_id = serde_json::from_value::<Principal>(val["canisterId"].clone());
            let idl_content = serde_json::from_value::<String>(val["idlContent"].clone());

            match canister_id {
                Ok(cid) => match idl_content {
                    Ok(idl) => Ok((cid, idl)),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
            .map_err(|e| e.into())
        })
        .and_then(|(cid, idl)| register_idl(cid, idl).map(|_| "()"))
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn ic_remove_idl(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| {
            serde_json::from_value::<Principal>(val["canisterId"].clone()).map_err(|e| e.into())
        })
        .and_then(|cid| remove_idl(&cid))
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn ic_get_idl(req: Request) -> Response {
    let rsp = unsafe { CStr::from_ptr(req).to_str() }
        .map_err(|e| e.into())
        .and_then(|str| serde_json::from_str::<Value>(str).map_err(|e| e.into()))
        .and_then(|val| {
            serde_json::from_value::<Principal>(val["canisterId"].clone()).map_err(|e| e.into())
        })
        .and_then(|cid| get_idl(&cid))
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn ic_list_idl() -> Response {
    let rsp = list_idl()
        .and_then(|list| serde_json::to_string(&list).map_err(|e| e.into()))
        .into();

    rsp
}

/// Query canister from ic main-net
///
/// Request Json:
///
/// {
///     "canisterId": ..,
///     "funcName": ..,
///     "funcArgs": [..],
///     "caller": ..
/// }
#[no_mangle]
#[tokio::main]
pub async extern "C" fn ic_query_sync(
    caller: LPCSTR,
    canister_id: LPCSTR,
    method_name: LPCSTR,
    args_raw: LPCSTR,
) -> Response {
    let caller_r = unsafe { CStr::from_ptr(caller).to_str() };
    let canister_id_r = unsafe { CStr::from_ptr(canister_id).to_str() };
    let method_name_r = unsafe { CStr::from_ptr(method_name).to_str() };
    let args_raw_r = unsafe { CStr::from_ptr(args_raw).to_str() };

    let args = match caller_r {
        Ok(caller) => match canister_id_r {
            Ok(canister_id) => match method_name_r {
                Ok(method_name) => match args_raw_r {
                    Ok(args_raw) => Ok((caller, canister_id, method_name, args_raw)),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
    .map_err(|e| anyhow::Error::from(e));

    let rsp = args
        .and_then(|(caller, canister_id, method_name, args_raw)| {
            let caller = Principal::from_str(caller)
                .context(format!("Failed to parse caller {}", caller))?;
            let canister_id = Principal::from_str(canister_id)
                .context(format!("Failed to parse canister_id {}", canister_id))?;

            let fut = query(&caller, &canister_id, method_name, args_raw);
            let rst_idl = task::block_in_place(move || runtime::Handle::current().block_on(fut))?;

            Ok(rst_idl.to_string())
        })
        .into();

    rsp
}

// #[no_mangle]
// pub extern "C" fn ic_update_sync(req: Request) -> Response {
//     todo!()
// }

#[no_mangle]
pub extern "C" fn free_rsp(rsp: Response) {
    let data = unsafe { CString::from_raw(rsp.ptr) };
    drop(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{ensure, Result};

    const NAME: &str = "agent-unity";
    const PASSWORD: &str = "agent-unity-test";

    const II_CANISTER_ID: &'static str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
    const II_CANDID_FILE: &'static str = include_str!("rdmx6-jaaaa-aaaaa-aaadq-cai.did");

    #[test]
    fn create_keystore_should_work() -> Result<()> {
        let args_json = CString::new(r#"{"name": "agent-unity", "password": "agent-unity-test"}"#)?;
        let req = args_json.as_ptr() as Request;

        let rsp = create_keystore(req);
        let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
        ensure!(!rsp.is_err, anyhow!(str));

        let _key_store = serde_json::from_str::<HostKeyStore>(str)?;

        Ok(())
    }

    #[test]
    fn login_by_host_should_work() -> Result<()> {
        let keystore = HostKeyStore::random(NAME, PASSWORD)?;
        let keystore_json = serde_json::to_string(&keystore)?;
        let args_json = CString::new(format!(
            r#"{{"keyStore": {}, "password": "{}"}}"#,
            keystore_json, PASSWORD
        ))?;
        let req = args_json.as_ptr() as Request;

        let rsp = login_by_host(req);
        let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
        ensure!(!rsp.is_err, anyhow!(str));

        let _receipt = serde_json::from_str::<LoggedReceipt>(str)?;

        Ok(())
    }

    #[test]
    fn query_ii_lookup_should_work() -> Result<()> {
        let keystore = HostKeyStore::random(NAME, PASSWORD)?;
        let keystore_json = serde_json::to_string(&keystore)?;
        let args_json = CString::new(format!(
            r#"{{"keyStore": {}, "password": "{}"}}"#,
            keystore_json, PASSWORD
        ))?;
        let req = args_json.as_ptr() as Request;

        let rsp = login_by_host(req);
        let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
        ensure!(!rsp.is_err, anyhow!(str));

        let receipt = serde_json::from_str::<LoggedReceipt>(str)?;

        // register ii candid file
        ic_helper::register_idl(Principal::from_str(II_CANISTER_ID)?, II_CANDID_FILE.into())?;

        let caller = CString::new(receipt.principal.to_string())?.into_raw() as LPCSTR;
        let canister_id = "rdmx6-jaaaa-aaaaa-aaadq-cai\0".as_ptr() as LPCSTR;
        let method_name = "lookup\0".as_ptr() as LPCSTR;
        let args_raw = "(1974211: nat64)\0".as_ptr() as LPCSTR;

        let rsp = ic_query_sync(caller, canister_id, method_name, args_raw);
        let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
        ensure!(!rsp.is_err, anyhow!(str));

        let rst_raw = str;
        println!("{}", rst_raw);

        Ok(())
    }
}
