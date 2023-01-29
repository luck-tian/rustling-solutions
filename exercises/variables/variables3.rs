// variables3.rs
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a hint.


use std::arch::asm;
fn main() {
    let x: i32;
    unsafe {
        asm!("mov {} , 100", out(reg) x);
    }
    println!("Number {}", x);
}
