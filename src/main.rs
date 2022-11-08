mod little_rust_computer;
use little_rust_computer::*;

// Demonstration
fn main() {
    // Make a new CU instance using the new constructor from_memory_length()
    let mut cu: ControlUnit = ControlUnit::from_memory_length(1000);
    cu.set(20, 20);                                 // Set a value to address 20 in memory
    cu.set(10, 21);                                 // Set a value to address 21 in memory
    cu.alu.val1 = cu.mem.register[20];              // Set the ALU input value 1 from mem addr 20
    cu.alu.val2 = cu.mem.register[21];              // Set the ALU input value 2 from mem addr 21
    
    cu.alu.add();                                   // Add values in ALU and send to accumulator
    cu.get_acc(23);                                 // Gets the accumulator value, sends to mem addr 23
    cu.output(23);                                  // Output the contents of memory address 23

    cu.alu.sub();                                   // Subtract example
    cu.get_acc(24);
    cu.output(24);

    cu.alu.mul();                                   // Multiply example
    cu.get_acc(25);
    cu.output(25);

    cu.alu.div();                                   // Divide example
    cu.get_acc(26);
    cu.output(26);
}