use crate::{Response, LPCSTR};
use libc::c_void;

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_build(slf: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_url(slf: c_void, url: LPCSTR) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_transport(slf: c_void, transport: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_nonce_factory(
    slf: c_void,
    nonce_factory: c_void,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_nonce_generator(
    slf: c_void,
    nonce_factory: c_void,
) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_identity(slf: c_void, identity: c_void) -> Response {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn AgentBuilder_with_ingress_expiry(
    slf: c_void,
    duration: c_void,
) -> Response {
    todo!()
}
