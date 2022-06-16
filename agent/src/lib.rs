use crate::host::HostKeyStore;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use ic_agent::{identity::BasicIdentity, Identity};
use ic_types::Principal;
use lazy_static::lazy_static;
use libc::c_char;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::sync::Mutex;

pub mod host;

type LPSTR = *mut c_char;
type LPCSTR = *const c_char;
type JSON = LPCSTR;
type Request = JSON;

lazy_static! {
    static ref LOGGED_INFO: Mutex<HashMap::<Principal, (LoggedReceipt, BasicIdentity)>> =
        Mutex::new(HashMap::new());
}

#[repr(C)]
pub struct Response {
    pub ptr: LPSTR,
    pub is_err: bool,
}

impl Response {
    pub fn new<T: Into<Vec<u8>>>(data: T, is_err: bool) -> Self {
        // NOTE: Should be panic if is err!
        let data = CString::new(data).unwrap();
        let ptr = data.into_raw();

        Self { ptr, is_err }
    }
}

impl<T, E> From<Result<T, E>> for Response
where
    T: Into<Vec<u8>>,
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
            match guard.insert(receipt.principal.clone(), (receipt.clone(), identity)) {
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
            _ => Err(anyhow!(
                r#"cannot logout by principal: {}"#,
                principal
            )),
        })
        .into();

    rsp
}

#[no_mangle]
pub extern "C" fn ic_query(req: Request) -> Response {
    todo!()
}

#[no_mangle]
pub extern "C" fn ic_update(req: Request) -> Response {
    todo!()
}

#[no_mangle]
pub extern "C" fn free_rsp(rsp: Response) {
    let data = unsafe { CString::from_raw(rsp.ptr) };
    drop(data);
}
