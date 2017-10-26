extern crate cpu6502;

use cpu6502::Cpu;

fn main() {
    println!("Hello, world!");

    let cpu = Cpu::new();

    println!("{:?}", cpu);
}
