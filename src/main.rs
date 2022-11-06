mod little_rust_computer;
use little_rust_computer::*;
fn main() {
    let mut mar: MemoryAddressRegister = MemoryAddressRegister::new();
    mar.set(100);
    let curr = mar.get();
    println!("{curr:?}");
}
