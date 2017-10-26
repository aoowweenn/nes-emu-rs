#[derive(Debug, Clone, Copy, Default)]
pub struct Regs {
    pub a: i8, // Accumulator
    pub x: i8, // addressing mode index X
    pub y: i8, // addressing mode index Y
    pub pc: u16, // Program Counter
    pub s: u8, // Stack Pointer
    pub p: u8, // Status Register ( 6 bits )
}
