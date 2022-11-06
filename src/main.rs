mod little_rust_computer;
use little_rust_computer::*;
fn main() {
    let mut cu = ControlUnit::new();
    cu.set(10, 20);
    cu.set(20, 21);
    cu.alu.val1 = cu.mem.register[20];
    cu.alu.val2 = cu.mem.register[21];
    cu.alu.add();
    cu.get_acc(23);
    println!("{:?}", cu.mem.register[23]);
}
