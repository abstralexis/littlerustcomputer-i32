/// This is a Rust version of the Little Man Computer basically.
/// I decided to abstract it to hell and back so the terminology
/// is thorough and verbose. I like typing.

pub struct ControlUnit { 

}
impl ControlUnit {

}

pub struct ArithmeticLogicUnit {
}
impl ArithmeticLogicUnit {
}

pub struct ProgramCounter {

}

pub struct CurrentInstructionRegister {

}

pub struct Accumulator {

}

pub struct MemoryAddressRegister {
    val: u32,                                   // Register value
}
impl MemoryAddressRegister {
    pub fn new() -> Self {                      // Construct new MAR
        MemoryAddressRegister { val: 0 }   
    }

    pub fn set(&mut self, arg: u32) {           // Set the register to a value
        self.val = arg;
    }

    pub fn get(&self) -> u32 {                  // Get the register value
        return self.val;
    }
}

pub struct MemoryDataRegister {
    val: i32,
}
impl MemoryDataRegister {
    pub fn new() -> Self {
        MemoryDataRegister { val: 0 }
    }

    pub fn set(&mut self, arg: i32) {
        self.val = arg;
    }

    pub fn get(&self) -> i32 {
        return self.val;
    }
}

#[derive(Debug)]
pub struct MemoryUnit {
    register: Vec<i32>,
}
impl MemoryUnit {
    pub fn from_length(l: u8) -> Self {
        MemoryUnit { register: vec!{0; usize::from(l)} }
    }
}

pub struct CPU {
    cu: ControlUnit,
    alu: ArithmeticLogicUnit,
    pc: ProgramCounter,
    cir: CurrentInstructionRegister,
    ac: Accumulator,
    mar: MemoryAddressRegister,
    mdr: MemoryDataRegister,
    mem: MemoryUnit,
}