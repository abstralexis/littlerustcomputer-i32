use std::any::Any;

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

    pub fn iset(&mut self, val: i32, address: usize) {                // Sets a value to an address in memory
        self.mdr.val = Box::new(val);                               // Send the value to mdr
        self.mar.val = address;                                     // Send address to mar
        let val: i32 = *self.mdr.val.downcast_mut().expect("Must be int");
        self.mem.register[self.mar.val as usize] = Box::new(val);   // Use mdr and mar to set val in mem
    }

    pub fn ialuset(&mut self, addr1: usize, addr2: usize) {
        let val1: i32 = *self.mem.register[addr1].downcast_mut().expect("Must be int");
        self.alu.val1 = Box::new(val1);
        let val2: i32 = *self.mem.register[addr2].downcast_mut().expect("Must be int");
        self.alu.val2 = Box::new(val2);
    }

    pub fn iget_acc(&mut self, target_addr: usize) {                  // Gets val from accum. to a target addr
        self.mar.val = target_addr;                                 // Send addr to mar
        let acc: i32 = *self.alu.ac.val.downcast_mut().expect("Must be int");
        self.mdr.val = Box::new(acc);                             // Set mdr to accum. val
        let val: i32 = *self.mdr.val.downcast_mut().expect("Must be int");
        self.mem.register[target_addr as usize] = Box::new(val);              // Set val in mem
    }

    pub fn ioutput(&mut self, address: usize) {                        // Prints value from mem to terminal
        self.mar.val = address;
        let val: i32 = *self.mem.register[self.mar.val as usize].downcast_mut().expect("Must be int");
        self.mdr.val = Box::new(val);
        println!("{:?}", self.mdr.val);
    }
}

pub struct ArithmeticLogicUnit {    // The ALU holds 3 values in its register
    pub val1: Box<dyn Any>,                  // 2 input values
    pub val2: Box<dyn Any>,
    pub ac: Accumulator,            // And the accumulator where the output is temporarily stored
}
impl ArithmeticLogicUnit {
    pub fn new() -> Self { ArithmeticLogicUnit { val1: Box::new(0), val2: Box::new(0), ac: Accumulator::new() } }

    // Basic integer arithmetic
    pub fn iadd(&mut self) {
        let a: i32 = *self.val1.downcast_mut().expect("Value should be i32");
        let b: i32 = *self.val2.downcast_mut().expect("Value should be i32");
        self.ac.val = Box::new(a + b);
    }

    pub fn isub(&mut self) {
        let a: i32 = *self.val1.downcast_mut().expect("Value should be i32");
        let b: i32 = *self.val2.downcast_mut().expect("Value should be i32");
        self.ac.val = Box::new(a - b);
    }

    pub fn imul(&mut self) {
        let a: i32 = *self.val1.downcast_mut().expect("Value should be i32");
        let b: i32 = *self.val2.downcast_mut().expect("Value should be i32");
        self.ac.val = Box::new(a * b);
    }

    pub fn idiv(&mut self) {
        let a: i32 = *self.val1.downcast_mut().expect("Value should be i32");
        let b: i32 = *self.val2.downcast_mut().expect("Value should be i32");
        self.ac.val = Box::new(a / b);
    }
}

// TODO PC
pub struct ProgramCounter {

}

// TODO CIR
pub struct CurrentInstructionRegister {

}

pub struct Accumulator {
    val: Box<dyn Any>,
}
impl Accumulator {
    fn new() -> Self { Accumulator { val: Box::new(0) } }
}

// MAR and MDR did not *have* to be seperate registers but I think it
// is most accurate to what it actually represents.
pub struct MemoryAddressRegister {
    pub val: usize,                                   // Register value
}
impl MemoryAddressRegister {
    pub fn new() -> Self { MemoryAddressRegister { val: 0 } }
}

pub struct MemoryDataRegister {
    val: Box<dyn Any>,
}
impl MemoryDataRegister {
    pub fn new() -> Self { MemoryDataRegister { val: Box::new(0) } }
}

// TODO convert the memory vec to use Box<std::Any>
// The memory unit is like a vector register of values that can be indexed.
// Derives debug for debugging purposes such as dumping the memory contents to screen.
#[derive(Debug)]
pub struct MemoryUnit {
    pub register: Vec<Box<dyn Any>>,
}
impl MemoryUnit {
    // Construct new memory unit with default size 100
    pub fn new() -> Self { 
        let mut register: Vec<Box<dyn Any>> = Vec::with_capacity(100);
        for _ in 0..100 { register.push(Box::new(0)) }
        MemoryUnit { register: register } 
    }

    // Construct new memory unit from a usize
    pub fn from_length(l: usize) -> Self {
        let mut register: Vec<Box<dyn Any>> = Vec::with_capacity(l);
        for _ in 0..l { register.push(Box::new(0)) }
        MemoryUnit { register: register } 
    }
}