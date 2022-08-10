use ic_types::Principal;

#[no_mangle]
pub unsafe extern "C" fn Principal_management_canister() -> *const Principal {
    let principal = Principal::management_canister();

    let box_principal = Box::new(principal);
    let box_ptr = Box::into_raw(box_principal);
    std::mem::forget(box_ptr);

    box_ptr
}

#[no_mangle]
pub unsafe extern "C" fn Principal_free(slf: *mut Principal) {
    drop(Box::from_raw(slf))
}