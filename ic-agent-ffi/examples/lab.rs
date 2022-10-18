use ic_agent::Identity;

fn main() {
    let ptr_len = std::mem::size_of::<*const *mut dyn Identity>();
    let fptr_len = std::mem::size_of::<*mut dyn Identity>();
    println!("ptr_len: {}, fptr_len: {}", ptr_len, fptr_len);
}
