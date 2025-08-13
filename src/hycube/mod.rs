pub mod tile;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
/// This is represented by HyCube as a byte.
pub enum OpCode {
    NOP = 0,
    ADD = 1,
    SUB = 2,
    MUL = 3,
    SEXT = 4,
    DIV = 5,

    LS = 8,
    RS = 9,
    ARS = 10,
    AND = 11,
    OR = 12,
    XOR = 13,

    SELECT = 16,
    CMERGE = 17,
    CMP = 18,
    CLT = 19,
    BR = 20,
    CGT = 21,
    LOADCL = 22,
    MOVCL = 23,
    LOAD = 24,
    LOADH = 25,
    LOADB = 26,
    STORE = 27,
    STOREH = 28,
    STOREB = 29,
    JUMPL = 30,
    MOVC = 31,
}

#[derive(Debug)]
/// This repersents a HyCube's instruction that is stored in its
/// internal Memory Config.
#[allow(non_snake_case)]
pub struct HyIns {
    xB: XbarConfig,
    registers: [i8; 4],
    /// `true` = get from register, `false` = bypass
    /// `std::map<Regs,uint8_t> regbypass; //1 = get it from reg and 0 = bypass`
    registers_no_bypass: [bool; 4],
    // TODO: what is tregwen
    // uint8_t tregwen;
    opcode: OpCode,
    constant: i32,
    constValid: bool,
    // TODO: What is NPB
    // bool NPB;
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum XBarInput {
    NORTH_I,
    EAST_I,
    WEST_I,
    SOUTH_I,
    RES_I,
    ALU_I,
    INV,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum Dir {
    NORTH = 0,
    EAST = 1,
    WEST = 2,
    SOUTH = 3,
}

type DataType = i32;
type Memory = Vec<i8>;

#[derive(Debug)]
struct XbarConfig {
    p: XBarInput,
    i1: XBarInput,
    i2: XBarInput,
    north_o: XBarInput,
    east_o: XBarInput,
    west_o: XBarInput,
    south_o: XBarInput,
}
