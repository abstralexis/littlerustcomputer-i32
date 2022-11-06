mod little_rust_computer;
use little_rust_computer::*;
fn main() {
    let mut mar: MemoryAddressRegister = MemoryAddressRegister::new();
    mar.val = 100;
    println!("{:?}", mar.val);
}
