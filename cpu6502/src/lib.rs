mod regs;
mod addr_modes;

use regs::Regs;
use addr_modes::AddrMode;
use addr_modes::{get_addr_mode, get_operand_size};

#[derive(Debug)]
pub struct Cpu {
    arch: String,
    regs: Regs,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            arch: "NES-6502".to_string(),
            regs: Regs::default(),
        }
    }

    fn run(&mut self, mem: &mut [u8]) {
        /// TODO: cpu power up
        /// TODO: loop
        unimplemented!()
    }

    /// execute an instruction
    fn execute(&mut self, mem: &mut [u8]) {
        let pc = &mut self.regs.pc;
        let code = mem[*pc as usize];
        let addr_mode = get_addr_mode(code);
        let operand_length = get_operand_size(addr_mode);
        //let instr = get_instr(code);
        *pc += operand_length as u16 + 1;
        //instr.execute(self, addr_mode);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
