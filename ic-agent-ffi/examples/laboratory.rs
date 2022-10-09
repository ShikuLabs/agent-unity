use ic_types::Principal;

fn main() {
    let anonymous = Principal::anonymous();
    let anonymous_text = anonymous.to_text();
    let anonymous_bytes = anonymous.as_slice();

    println!("{}, {:?}", anonymous_text, anonymous_bytes);
}
