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
    register: Vec<i32>,
}
impl MemoryUnit {
    // Construct new memory unit with default size 100
    pub fn new() -> Self { MemoryUnit { register: vec!(0; 100) } }

    // Construct new memory unit from a usize
    pub fn from_length(l: usize) -> Self { MemoryUnit { register: vec!{0; l} } }
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