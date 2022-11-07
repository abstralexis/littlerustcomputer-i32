mod little_rust_computer;
use little_rust_computer::*;

// Demonstration
fn main() {
    // Make a new CU instance using the new constructor from_memory_length()
    let mut cu: ControlUnit = ControlUnit::new();
    println!("init {:?}", cu.mem.register);
    cu.iset(10, 20);                                // Set a value to address 20 in memory
    cu.iset(20, 21);                                // Set a value to address 21 in memory
    println!("alu {:?}", cu.mem.register);
    cu.ialuset(20, 21);                             // Set the ALU input value 2 from mem addr 21
    cu.alu.iadd();                                  // Add values in ALU and send to accumulator
    cu.iget_acc(23);                                // Gets the accumulator value, sends to mem addr 23
    println!("acc {:?} {:?}", cu.alu.val1, cu.alu.val2);
    cu.ioutput(23);                                 // Output the contents of memory address 23
}
