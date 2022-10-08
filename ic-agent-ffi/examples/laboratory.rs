use ic_types::Principal;

fn main() {
    let arr = [0u8; 16];
    let principal = Principal::from_slice(arr.as_slice());
    println!("{:?}", principal.as_slice());
    println!("{}, {}", principal.to_text(), Principal::management_canister().to_text());
}
