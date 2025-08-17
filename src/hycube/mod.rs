pub mod decode;
pub mod tile;

#[derive(Debug, Clone)]
#[repr(i32)]
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

/// This repersents a HyCube's instruction that is stored in its
/// internal Memory Config.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct HyIns {
    xB: XbarConfig,
    reg_we: [bool; 4],
    /// `true` = get from register, `false` = bypass
    /// `std::map<Regs,uint8_t> regbypass; //1 = get it from reg and 0 = bypass`
    reg_no_bypass: [bool; 4],
    // TODO: what is tregwen
    // uint8_t tregwen;
    treg_we: bool,
    opcode: OpCode,
    constant: i32,
    constValid: bool,
    // TODO: What is NPB
    npb: bool,
}

impl Default for HyIns {
    fn default() -> Self {
        Self {
            xB: Default::default(),
            reg_we: Default::default(),
            reg_no_bypass: Default::default(),
            treg_we: Default::default(),
            opcode: OpCode::NOP,
            constant: Default::default(),
            constValid: Default::default(),
            npb: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum XBarInput {
    EAST_I = 0,
    SOUTH_I = 1,
    WEST_I = 2,
    NORTH_I = 3,
    ALU_I = 4,
    RES_I = 5,
    INV = 7,
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
/// Config in time, see number 0,1,... in the Binary file..
pub type Configs = Vec<Config>;
/// [y][x]
pub type Config = Vec<Vec<HyIns>>;

#[derive(Debug, Clone)]
struct XbarConfig {
    p: XBarInput,
    i1: XBarInput,
    i2: XBarInput,
    north_o: XBarInput,
    east_o: XBarInput,
    west_o: XBarInput,
    south_o: XBarInput,
}

impl Default for XbarConfig {
    fn default() -> Self {
        use XBarInput::INV;
        Self {
            p: INV,
            i1: INV,
            i2: INV,
            north_o: INV,
            east_o: INV,
            west_o: INV,
            south_o: INV,
        }
    }
}
