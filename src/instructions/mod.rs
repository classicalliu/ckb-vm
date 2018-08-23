mod utils;

pub mod c;
pub mod rv32i;
pub mod rv32m;

use super::machine::Machine;
use super::memory::Memory;
use super::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    RV32I(rv32i::Instruction),
    RV32M(rv32m::Instruction),
    C(c::Instruction),
}

impl Instruction {
    pub fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<(), Error> {
        match self {
            Instruction::RV32I(instruction) => instruction.execute(machine),
            Instruction::RV32M(instruction) => instruction.execute(machine),
            Instruction::C(instruction) => instruction.execute(machine),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: change to real disasm feature instead of simply delegating
        // to std::fmt::Debug
        write!(f, "{:?}", self)
    }
}

pub type InstructionFactory = fn(instruction_bits: u32) -> Option<Instruction>;

// Instruction execution trait
pub trait Execute {
    fn execute<M: Memory>(&self, machine: &mut Machine<M>) -> Result<Option<NextPC>, Error>;
}

type RegisterIndex = usize;
type Immediate = i32;
type UImmediate = u32;
type NextPC = u32;

//
//  31       27 26 25 24     20 19    15 14    12 11          7 6      0
// ======================================================================
// | funct7          |   rs2   |   rs1  | funct3 |  rd         | opcode | R-type
// +--------------------------------------------------------------------+
// |          imm[11:0]        |   rs1  | funct3 |  rd         | opcode | I-type
// +--------------------------------------------------------------------+
// |   imm[11:5]     |   rs2   |   rs1  | funct3 | imm[4:0]    | opcode | S-type
// +--------------------------------------------------------------------+
// |   imm[12|10:5]  |   rs2   |   rs1  | funct3 | imm[4:1|11] | opcode | B-type
// +--------------------------------------------------------------------+
// |             imm[31:12]                      |  rd         | opcode | U-type
// +--------------------------------------------------------------------+
// |             imm[20|10:1|11|19:12]           |  rd         | opcode | J-type
// ======================================================================
//

#[derive(Debug)]
pub struct Rtype<I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    rd: RegisterIndex,
    inst: I,
}

#[derive(Debug)]
pub struct Itype<I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    imm: Immediate,
    inst: I,
}

#[derive(Debug)]
pub struct ItypeShift<I> {
    rs1: RegisterIndex,
    rd: RegisterIndex,
    shamt: Immediate,
    inst: I,
}

#[derive(Debug)]
pub struct Stype<I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    inst: I,
}

#[derive(Debug)]
pub struct Btype<I> {
    rs2: RegisterIndex,
    rs1: RegisterIndex,
    imm: Immediate,
    inst: I,
}

#[derive(Debug)]
pub struct Utype<I> {
    rd: RegisterIndex,
    imm: Immediate,
    inst: I,
}

#[derive(Debug)]
pub struct Jtype<I> {
    rd: usize,
    imm: i32,
    inst: I,
}

