enum Instruction {
    Add(u32),
    Sub(u32, u32),
    Halt
}

struct RegisterMachine {
    registers: Vec<i32>,
    program:   Vec<Instruction>,
}

fn main() {
    
}
