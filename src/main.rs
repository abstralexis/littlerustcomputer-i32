mod little_rust_computer;
use little_rust_computer::*;

// Demonstration
fn main() {
    let mut cu: ControlUnit = ControlUnit::new();   // New CU instance
    cu.set(10, 20);                                 // Set a value to address 20 in memory
    cu.set(20, 21);                                 // Set a value to address 21 in memory
    cu.alu.val1 = cu.mem.register[20];              // Set the ALU input value 1 from mem addr 20
    cu.alu.val2 = cu.mem.register[21];              // Set the ALU input value 2 from mem addr 21
    cu.alu.add();                                   // Add values in ALU and send to accumulator
    cu.get_acc(23);                                 // Gets the accumulator value, sends to mem addr 23
    cu.output(23);                                  // Output the contents of memory address 23
}
