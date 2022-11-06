/// This is a Rust version of the Little Man Computer basically.
/// I decided to abstract it to hell and back so the terminology
/// is thorough and verbose. I like typing.

pub struct ControlUnit { 
    pub alu: ArithmeticLogicUnit,
    pub mem: MemoryUnit,
    pub mar: MemoryAddressRegister,
    pub mdr: MemoryDataRegister,
}
impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit { 
            alu: ArithmeticLogicUnit::new(),
            mem: MemoryUnit::new(),
            mar: MemoryAddressRegister::new(),
            mdr: MemoryDataRegister::new(),
        }
    }

    pub fn set(&mut self, val: i32, address: u32) {
        self.mdr.val = val;
        self.mar.val = address;
        self.mem.register[self.mar.val as usize] = self.mdr.val;
    }

    pub fn get_acc(&mut self, target_addr: u32) {
        self.mar.val = target_addr;
        self.mdr.val = self.alu.ac.val;
        self.mem.register[target_addr as usize] = self.mdr.val;
    }

    pub fn output(&mut self, address: u32) {
        self.mar.val = address;
        self.mdr.val = self.mem.register[self.mar.val as usize];
        println!("{:?}", self.mdr.val);
    }
}

pub struct ArithmeticLogicUnit {
    pub val1: i32,
    pub val2: i32,
    pub ac: Accumulator,
}
impl ArithmeticLogicUnit {
    pub fn new() -> Self { ArithmeticLogicUnit { val1: 0, val2: 0, ac: Accumulator::new() } }

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