use crate::hycube::HyIns;
use crate::hycube::OpCode;

use super::DataType;
use super::Memory;

#[derive(Debug)]
struct CgraTile<'a> {
    // Outputs
    result: i64,
    opcode: OpCode,

    // Configuration
    config_mem: &'a Vec<HyIns>,

    x: usize,
    y: usize,
    hash_mem: bool,

    inputs: TileInputs,
    // I don't really know what writing the input means...
    inputs_tobrwritten: TileInputs,

    regs: TileRegisters,
    regs_tobewritten: TileRegisters,
}

impl CgraTile<'_> {
    fn new(x: usize, y: usize, has_mem: bool, data_mem: &Memory) -> Self {
        let _ = (x, y, has_mem, data_mem);
        todo!("TODO: Implement this!!")
    }

    // Note: I split `.tick_alu` for readability...
    fn tick(&mut self) {}
    fn tick_alu(&mut self) {
        let i1_p = self.inputs.i1_p;
        let i2_p = self.inputs.i2_p;
        // TODO: handle `.unwrap()` this safely!!
        let (i1, i2) = (self.inputs.i1.unwrap(), self.inputs.i2.unwrap());

        // For  Select statment, `i1_p` and `i2_p` should never be
        // valid at the same time! Hard crash.
        assert!(
            !(i1_p && i2_p),
            "i1_p && i2_p = true, predicates can't all be true"
        );
        if !(self.inputs.p.unwrap()) {
            return;
        }

        let _tmp_result = match self.opcode {
            OpCode::NOP => todo!(),
            OpCode::ADD => i1 + i2,
            OpCode::SUB => i1 - i2,
            OpCode::MUL => i1 * i2,
            OpCode::SEXT => todo!(),
            OpCode::DIV => i1 / i2,
            OpCode::LS => i1 << i2,
            OpCode::RS => i1 >> i2,
            OpCode::ARS => todo!(),
            OpCode::AND => i1 & i2,
            OpCode::OR => i1 | i2,
            OpCode::XOR => i1 ^ i2,
            OpCode::SELECT => {
                if i1_p {
                    i1
                } else if i2_p {
                    12
                } else {
                    panic!("SELECT instruction issued, yet both branches are false!!!")
                }
            }
            OpCode::CMERGE => todo!(),
            OpCode::CMP => (i1 == i2) as i32,
            OpCode::CLT => (i1 < i2) as i32,
            OpCode::BR => todo!(),
            OpCode::CGT => (i1 > i2) as i32,
            OpCode::LOADCL => todo!(),
            OpCode::MOVCL => todo!(),
            OpCode::LOAD => todo!(),
            OpCode::LOADH => todo!(),
            OpCode::LOADB => todo!(),
            OpCode::STORE => todo!(),
            OpCode::STOREH => todo!(),
            OpCode::STOREB => todo!(),
            OpCode::JUMPL => todo!(),
            OpCode::MOVC => todo!(),
        };
    }

    fn rst(&mut self) {
        todo!()
    }
}

#[derive(Debug)]
pub struct TileRegisters {
    r0: Option<DataType>,
    r1: Option<DataType>,
    r2: Option<DataType>,
    r3: Option<DataType>,
    res: Option<DataType>,
}

#[derive(Debug)]
pub struct TileInputs {
    i1: Option<DataType>,
    i2: Option<DataType>,
    // predicate inputs
    // In hardware, `i{1,2}_p`
    p: Option<bool>,
    i1_p: bool,
    i2_p: bool,
}
