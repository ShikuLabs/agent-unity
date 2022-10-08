use libc::c_char;

pub struct AgentFFI {}

#[no_mangle]
pub extern "C" fn agent_with_transport(ic_net: *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn agent_set_canister(canister_id: *const c_char, candid_file: *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn query_sign() {
    todo!()
}

#[no_mangle]
pub extern "C" fn update_sign() {
    todo!()
}

#[no_mangle]
pub extern "C" fn send() {
    todo!()
}
