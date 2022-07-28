use crate::{Response, LPCSTR};
use libc::c_void;

#[no_mangle]
pub unsafe extern "C" fn Agent_builder() -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_new(config: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_set_transport(slf: c_void, transport: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_fetch_root_key(slf: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_set_root_key(slf: c_void, root_key: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_get_principal(slf: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_query_signed(
    slf: c_void,
    effective_canister_id: c_void,
    signed_query: c_void,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_update_signed(
    slf: c_void,
    effective_canister_id: c_void,
    signed_update: c_void,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_pool(
    slf: c_void,
    request_id: c_void,
    effective_canister_id: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_wait(
    slf: c_void,
    request_id: c_void,
    effective_canister_id: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_read_state_raw(
    slf: c_void,
    paths: c_void,
    effective_canister_id: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_verify(
    slf: c_void,
    cert: c_void,
    effective_canister_id: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_read_state_canister_id(
    slf: c_void,
    canister_id: c_void,
    path: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_read_state_canister_metadata(
    slf: c_void,
    canister_id: c_void,
    path: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_request_status_raw(
    slf: c_void,
    request_id: c_void,
    path: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_request_status_signed(
    slf: c_void,
    request_id: c_void,
    effective_canister_id: c_void,
    signed_request_status: c_void,
    disable_range_check: bool,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_update(
    slf: c_void,
    canister_id: c_void,
    method_name: LPCSTR,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_status(slf: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_query(
    slf: c_void,
    canister_id: c_void,
    method_name: LPCSTR,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn Agent_sign_request_status(
    slf: c_void,
    effective_canister_id: c_void,
    request_id: c_void,
) -> Response {
    todo!()
}
