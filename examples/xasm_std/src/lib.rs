extern crate xmachine;
use xmachine::{Machine, Value};


pub fn xasm_println(xasm: &mut Machine) {
    println!("{}", xasm.pop());
}

pub fn xasm_print(xasm: &mut Machine) {
    print!("{}", xasm.pop());
}
