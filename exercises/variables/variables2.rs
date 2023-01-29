// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

use std::arch::asm;
fn main() {
    let x:i32;
    // unsafe{
    //     asm!("mov {}, 10", out(reg) x);
    // }
    x = 10i32;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
