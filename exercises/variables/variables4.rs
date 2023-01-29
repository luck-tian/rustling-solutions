// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.


use std::arch::asm;
fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    unsafe{
        asm!("add {} ,2", inout(reg) x);
    }
    println!("Number {}", x);
}
