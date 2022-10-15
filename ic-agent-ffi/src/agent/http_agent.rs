use crate::AnyErr;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn agent_create(url: *const c_char, identity: u32) {
    let url = unsafe { CStr::from_ptr(url).to_str().map_err(AnyErr::from) };

    println!("{}, {}", url.unwrap(), identity);
}
