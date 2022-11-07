/// This is a Rust version of the Little Man Computer basically.
/// I decided to abstract it to hell and back so the terminology
/// is thorough and verbose. I like typing.

pub struct ControlUnit {                // This is the main CU struct used to interface with cpu parts
    pub alu: ArithmeticLogicUnit,
    pub mem: MemoryUnit,
    pub mar: MemoryAddressRegister,
    pub mdr: MemoryDataRegister,
}
impl ControlUnit {
    pub fn new() -> Self {              // Creates new instance using default values
        ControlUnit { 
            alu: ArithmeticLogicUnit::new(),
            mem: MemoryUnit::new(),
            mar: MemoryAddressRegister::new(),
            mdr: MemoryDataRegister::new(),
        }
    }

    pub fn from_memory_length(mem_size: usize) -> Self {
        ControlUnit {
            alu: ArithmeticLogicUnit::new(),
            mem: MemoryUnit::from_length(mem_size),
            mar: MemoryAddressRegister::new(),
            mdr: MemoryDataRegister::new(),
        }
    }

    pub fn set(&mut self, val: i32, address: u32) {                 // Sets a value to an address in memory
        self.mdr.val = val;                                         // Send the value to mdr
        self.mar.val = address;                                     // Send address to mar
        self.mem.register[self.mar.val as usize] = self.mdr.val;    // Use mdr and mar to set val in mem
    }   

    pub fn get_acc(&mut self, target_addr: u32) {                   // Gets val from accum. to a target addr
        self.mar.val = target_addr;                                 // Send addr to mar
        self.mdr.val = self.alu.ac.val;                             // Set mdr to accum. val
        self.mem.register[target_addr as usize] = self.mdr.val;     // Set val in mem
    }

    pub fn output(&mut self, address: u32) {                        // Prints value from mem to terminal
        self.mar.val = address;
        self.mdr.val = self.mem.register[self.mar.val as usize];
        println!("{:?}", self.mdr.val);
    }
}

pub struct ArithmeticLogicUnit {    // The ALU holds 3 values in its register
    pub val1: i32,                  // 2 input values
    pub val2: i32,
    pub ac: Accumulator,            // And the accumulator where the output is temporarily stored
}
impl ArithmeticLogicUnit {
    pub fn new() -> Self { ArithmeticLogicUnit { val1: 0, val2: 0, ac: Accumulator::new() } }

    // Basic integer arithmetic
    pub fn add(&mut self) {
        self.ac.val = self.val1 + self.val2;
    }

    pub fn sub(&mut self) {
        self.ac.val = self.val1 - self.val2;
    }

    pub fn mul(&mut self) {
        self.ac.val = self.val1 * self.val2;
    }

    pub fn div(&mut self) {
        self.ac.val = self.val1 / self.val2
    }
}

// TODO PC
pub struct ProgramCounter {

}

// TODO CIR
pub struct CurrentInstructionRegister {

}

pub struct Accumulator {
    val: i32,
}
impl Accumulator {
    fn new() -> Self { Accumulator { val: 0 } }
}

// MAR and MDR did not *have* to be seperate registers but I think it
// is most accurate to what it actually represents.
pub struct MemoryAddressRegister {
    pub val: u32,                                   // Register value
}
impl MemoryAddressRegister {
    pub fn new() -> Self { MemoryAddressRegister { val: 0 } }
}

pub struct MemoryDataRegister {
    val: i32,
}
impl MemoryDataRegister {
    pub fn new() -> Self { MemoryDataRegister { val: 0 } }
}

// The memory unit is like a vector register of values that can be indexed.
// Derives debug for debugging purposes such as dumping the memory contents to screen.
#[derive(Debug)]
pub struct MemoryUnit {
    pub register: Vec<i32>,
}
impl MemoryUnit {
    // Construct new memory unit with default size 100
    pub fn new() -> Self { MemoryUnit { register: vec!(0; 100) } }

    // Construct new memory unit from a usize
    pub fn from_length(l: usize) -> Self { MemoryUnit { register: vec!{0; l} } }
}