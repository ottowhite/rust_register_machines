enum InstructionType {
    Add,
    Sub,
    Halt
}

fn main() {
    println!("Hello, world!");
    let a = encode((5, 15));
    println!("{}", a);
}

fn encode((a, b): (i32, i32)) -> i32 {
    return a + b;
}
