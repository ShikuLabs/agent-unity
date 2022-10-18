#![feature(unsize)]

use anyhow::Error as AnyErr;

use crate::host::HostKeyStore;
use crate::ic_helper::{get_idl, list_idl, query, register_idl, remove_idl, update};
use anyhow::{anyhow, Context, Error};
use chrono::{DateTime, Utc};
use ic_agent::Identity;
use ic_types::Principal;
use lazy_static::lazy_static;
use libc::{c_char, c_int};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::marker::Unsize;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::runtime;

mod host;
mod ic_helper;

mod agent;
mod identity;
mod principal;

/// NOTE: New Things

/// A callback used to give the unsized value to caller.
type UnsizedCallBack = extern "C" fn(*const u8, c_int);

/// TODO: Use macro to abstract the same parts from the different functions for reducing duplicated code.
fn ret_unsized(unsized_cb: UnsizedCallBack, s: impl AsRef<[u8]>) {
    let arr = s.as_ref();
    let len = arr.len() as c_int;

    unsized_cb(arr.as_ptr(), len);
}

unsafe fn ret_fat_ptr<T, U>(p2p: *mut *const U, t: T)
where
    T: Unsize<U>,
    U: ?Sized,
{
    let boxed = Box::new(t);
    let raw = Box::into_raw(boxed);

    *p2p = raw;
}

/// The state code represented the status of calling ffi functions.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum StateCode {
    /// Ok
    Ok = 0,
    /// Error
    Err = -1,
    /// The data copied to the memory overflows.
    DataOverflow = -2,
    /// The function was terminated by an internal error.
    InternalErr = -3,
    /// The error info copied to the memory overflows.
    ErrInfoOverflow = -4,
}

#[allow(clippy::upper_case_acronyms)]
type LPSTR = *mut c_char;
#[allow(clippy::upper_case_acronyms)]
type LPCSTR = *const c_char;
#[allow(clippy::upper_case_acronyms)]
type JSON = LPCSTR;
#[allow(clippy::upper_case_acronyms)]
type Request = JSON;

type LoggedInfoType = Mutex<HashMap<Principal, (LoggedReceipt, Arc<dyn Identity>)>>;

lazy_static! {
    static ref LOGGED_INFO: LoggedInfoType = Mutex::new(HashMap::new());
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
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn create_keystore(req: Request) -> Response {
    let rsp = CStr::from_ptr(req)
        .to_str()
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
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn login_by_host(req: Request) -> Response {
    let rsp = CStr::from_ptr(req)
        .to_str()
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
            let temp = guard.insert(receipt.principal, (receipt.clone(), Arc::new(identity)));

            match temp {
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
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn get_logged_receipt(req: Request) -> Response {
    let rsp = CStr::from_ptr(req)
        .to_str()
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
        .and_then(|(guard, principal)| {
            let temp = guard.get(&principal);

            match temp {
                Some((receipt, _)) => serde_json::to_string(receipt).map_err(|e| e.into()),
                _ => Err(anyhow!(
                    r#"cannot find the logged info by principal: {}"#,
                    principal
                )),
            }
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
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn logout(req: Request) -> Response {
    let rsp = CStr::from_ptr(req)
        .to_str()
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
        .and_then(|(mut guard, principal)| {
            let temp = guard.remove(&principal);

            match temp {
                Some(_) => Ok("success"),
                _ => Err(anyhow!(r#"cannot logout by principal: {}"#, principal)),
            }
        })
        .into();

    rsp
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn ic_register_idl(canister_id: LPCSTR, candid_file: LPCSTR) -> Response {
    let canister_id_r = CStr::from_ptr(canister_id).to_str();
    let candid_file_r = CStr::from_ptr(candid_file).to_str();

    let args = match canister_id_r {
        Ok(canister_id) => match candid_file_r {
            Ok(candid_file) => Ok((canister_id, candid_file)),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
    .map_err(Error::from);

    args.and_then(|(canister_id, candid_file)| {
        let canister_id = Principal::from_str(canister_id)
            .context(format!("Failed to parse canister_id {}", canister_id))?;

        register_idl(canister_id, candid_file.into())?;

        Ok("()")
    })
    .into()
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn ic_remove_idl(canister_id: LPCSTR) -> Response {
    let canister_id_r = CStr::from_ptr(canister_id).to_str().map_err(Error::from);

    canister_id_r
        .and_then(|canister_id| {
            let canister_id = Principal::from_str(canister_id)
                .context(format!("Failed to parse canister id {}", canister_id))?;

            let candid_file = remove_idl(&canister_id)?.unwrap_or_else(|| "null".into());

            Ok(candid_file)
        })
        .into()
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn ic_get_idl(canister_id: LPCSTR) -> Response {
    let canister_id_r = CStr::from_ptr(canister_id).to_str().map_err(Error::from);

    canister_id_r
        .and_then(|canister_id| {
            let canister_id = Principal::from_str(canister_id)
                .context(format!("Failed to parse canister id {}", canister_id))?;

            let candid_file = get_idl(&canister_id)?.unwrap_or_else(|| "null".into());

            Ok(candid_file)
        })
        .into()
}

#[no_mangle]
pub extern "C" fn ic_list_idl() -> Response {
    list_idl()
        .and_then(|list| serde_json::to_string(&list).map_err(|e| e.into()))
        .into()
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn ic_query_sync(
    caller: LPCSTR,
    canister_id: LPCSTR,
    method_name: LPCSTR,
    args_raw: LPCSTR,
) -> Response {
    let caller_r = CStr::from_ptr(caller).to_str();
    let canister_id_r = CStr::from_ptr(canister_id).to_str();
    let method_name_r = CStr::from_ptr(method_name).to_str();
    let args_raw_r = CStr::from_ptr(args_raw).to_str();

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
    .map_err(Error::from);

    args.and_then(|(caller, canister_id, method_name, args_raw)| {
        let caller =
            Principal::from_str(caller).context(format!("Failed to parse caller {}", caller))?;
        let canister_id = Principal::from_str(canister_id)
            .context(format!("Failed to parse canister_id {}", canister_id))?;

        let runtime = runtime::Runtime::new()?;

        let fut = query(&caller, &canister_id, method_name, args_raw);
        let rst_idl = runtime.block_on(fut)?;

        Ok(rst_idl.to_string())
    })
    .into()
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn ic_update_sync(
    caller: LPCSTR,
    canister_id: LPCSTR,
    method_name: LPCSTR,
    args_raw: LPCSTR,
) -> Response {
    let caller_r = CStr::from_ptr(caller).to_str();
    let canister_id_r = CStr::from_ptr(canister_id).to_str();
    let method_name_r = CStr::from_ptr(method_name).to_str();
    let args_raw_r = CStr::from_ptr(args_raw).to_str();

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
    .map_err(Error::from);

    args.and_then(|(caller, canister_id, method_name, args_raw)| {
        let caller =
            Principal::from_str(caller).context(format!("Failed to parse caller {}", caller))?;
        let canister_id = Principal::from_str(canister_id)
            .context(format!("Failed to parse canister_id {}", canister_id))?;

        let runtime = runtime::Runtime::new()?;

        let fut = update(&caller, &canister_id, method_name, args_raw);
        let rst_idl = runtime.block_on(fut)?;

        Ok(rst_idl.to_string())
    })
    .into()
}

#[no_mangle]
/// # Safety
///
/// Workaround for `cargo clippy`
pub unsafe extern "C" fn free_rsp(rsp: Response) {
    let data = CString::from_raw(rsp.ptr);
    drop(data);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anyhow::{ensure, Result};
//
//     const NAME: &str = "agent-unity";
//     const PASSWORD: &str = "agent-unity-test";
//
//     const II_CANISTER_ID: &'static str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
//     const II_CANDID_FILE: &'static str = include_str!("rdmx6-jaaaa-aaaaa-aaadq-cai.did");
//
//     #[test]
//     fn create_keystore_should_work() -> Result<()> {
//         let args_json = CString::new(r#"{"name": "agent-unity", "password": "agent-unity-test"}"#)?;
//         let req = args_json.as_ptr() as Request;
//
//         let rsp = create_keystore(req);
//         let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
//         ensure!(!rsp.is_err, anyhow!(str));
//
//         let _key_store = serde_json::from_str::<HostKeyStore>(str)?;
//
//         Ok(())
//     }
//
//     #[test]
//     fn login_by_host_should_work() -> Result<()> {
//         let keystore = HostKeyStore::random(NAME, PASSWORD)?;
//         let keystore_json = serde_json::to_string(&keystore)?;
//         let args_json = CString::new(format!(
//             r#"{{"keyStore": {}, "password": "{}"}}"#,
//             keystore_json, PASSWORD
//         ))?;
//         let req = args_json.as_ptr() as Request;
//
//         let rsp = login_by_host(req);
//         let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
//         ensure!(!rsp.is_err, anyhow!(str));
//
//         let _receipt = serde_json::from_str::<LoggedReceipt>(str)?;
//
//         Ok(())
//     }
//
//     #[test]
//     fn query_ii_lookup_should_work() -> Result<()> {
//         let keystore = HostKeyStore::random(NAME, PASSWORD)?;
//         let keystore_json = serde_json::to_string(&keystore)?;
//         let args_json = CString::new(format!(
//             r#"{{"keyStore": {}, "password": "{}"}}"#,
//             keystore_json, PASSWORD
//         ))?;
//         let req = args_json.as_ptr() as Request;
//
//         let rsp = login_by_host(req);
//         let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
//         ensure!(!rsp.is_err, anyhow!(str));
//
//         let receipt = serde_json::from_str::<LoggedReceipt>(str)?;
//
//         // register ii candid file
//         ic_helper::register_idl(Principal::from_str(II_CANISTER_ID)?, II_CANDID_FILE.into())?;
//
//         let caller = format!("{}\0", receipt.principal.to_string());
//         let caller = caller.as_ptr() as LPCSTR;
//         let canister_id = "rdmx6-jaaaa-aaaaa-aaadq-cai\0".as_ptr() as LPCSTR;
//         let method_name = "lookup\0".as_ptr() as LPCSTR;
//         let args_raw = "(1974211: nat64)\0".as_ptr() as LPCSTR;
//
//         let rsp = ic_query_sync(caller, canister_id, method_name, args_raw);
//         let str = unsafe { CStr::from_ptr(rsp.ptr).to_str() }?;
//         ensure!(!rsp.is_err, anyhow!(str));
//
//         let rst_raw = str;
//         println!("{}", rst_raw);
//
//         Ok(())
//     }
// }
