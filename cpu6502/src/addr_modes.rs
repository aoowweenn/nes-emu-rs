// Ref: http://wiki.nesdev.com/w/index.php/CPU_addressing_modes
// Ref: http://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes

pub fn get_addr_mode(opcode: u8) -> AddrMode {
    let h = (opcode & 0xE0u8) >> 5; // h = opcode[7:5]
    let l = opcode & 0x1Fu8; // l = opcode[4:0]

    if opcode == 0x6Cu8 {
        // We check filter out 0x6C first, and then check 0x0C column
        // 0x6C jmp
        // (a)
        AddrMode::Ind
    }
    else if (l >> 2) == 0b001u8 {
        // h = *, l = 0x04
        // d
        AddrMode::ZeroPage
    }
    else if (h >> 1) == 0b10u8 && (l >> 1) == 0b1011u8 {
        // We check d,y addr mode first, and then check d,x
        // h = 0b10?, l = 0b1011?
        // d,y
        AddrMode::ZeroPageY
    }
    else if (l >> 2) == 0b101u8 {
        // h = *, l = 0b101??
        // d,x
        AddrMode::ZeroPageX
    }
    else if (l >> 2) == 0b011u8  || opcode == 0x20 {
        // h = *, l = 0x0C or JSR
        // a
        AddrMode::Abs
    }
    else if ( (h >> 2) != 0 && (l & !0b00010u8) == 0 ) || (l & !0b00010u8) ^ 0b01001u8 == 0 {
        // h = 0b1??, l = 0b000?0
        // h = *, l = 0b010?1
        // #i
        AddrMode::Imm
    }
    else if l == 0x10u8 {
        // h = *, l = 0x10
        // *+d
        AddrMode::Rel
    }
    else if (l & !0b00010u8) ^ 0b00001u8 == 0 {
        // h = *, l = 0b0001?
        // (d,x)
        AddrMode::IndIdxX
    }
    else if (l & !0b00010u8) ^ 0b10001u8 == 0 {
        // h = *, l = 0b10001
        // (d),y
        AddrMode::IndIdxY
    }
    else if l == 0x19u8 || ( (h >> 1) == 0b10u8 && (l >> 1) == 0b1111u8 ) {
        // h = *, l = 0x19
        // h = 0b10?, l = 0b1111?
        // We check a,y addr mode first, and then check a,x
        // a,y
        AddrMode::AbsIdxY
    }
    else if (l >> 2) == 0b111u8 {
        // h = *, l = 0b111??
        // a,x
        AddrMode::AbsIdxX
    }
    else {
        AddrMode::Implicit
    }
}

// TODO: const fn?
pub fn get_operand_size(mode: AddrMode) -> usize {
    match mode {
        AddrMode::ZeroPage | AddrMode::ZeroPageX | AddrMode::ZeroPageY |
        AddrMode::IndIdxX | AddrMode::IndIdxY | AddrMode::Imm | AddrMode::Rel => 1,
        AddrMode::Abs | AddrMode::AbsIdxX | AddrMode::AbsIdxY | AddrMode::Ind => 2,
        _ => 0,
    }
}

/// Addressing value type | Abbreviation | Mode Name
#[derive(Debug, PartialEq)]
pub enum AddrMode {
    /// u8  | d,x   | Zero page indexed X
    ZeroPageX,
    /// u8  | d,y   | Zero page indexed Y
    ZeroPageY,
    /// u16 | a,x   | Absolute indexed X
    AbsIdxX,
    /// u16 | a,y   | Absolute indexed Y
    AbsIdxY,
    /// u16 | (d,x) | Indirect indexed X
    IndIdxX,
    /// u16 | (d),y | Indirect indexed Y
    IndIdxY,
    /// ()  |       | Implicit
    Implicit,
    /// u8  | #v    | Immediate
    Imm,
    /// u8  | d     | Zero page
    ZeroPage,
    /// u16 | a     | Absolute
    Abs,
    /// i8  | label | Relative
    Rel,
    /// u16 | (a)   | Indirect
    Ind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_works() {
        assert_eq!(get_addr_mode(0x6Cu8), AddrMode::Ind);
        assert_eq!(get_addr_mode(0xB6u8), AddrMode::ZeroPageY);
        assert_eq!(get_addr_mode(0x4Bu8), AddrMode::Imm);
    }
}
