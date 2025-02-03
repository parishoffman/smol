//! The tiny IR.

use crate::common::*;
use crate::front::ast::BOp;

#[derive(Debug)]
pub struct Program {
    pub decl: Set<Id>,
    pub block: Map<Id, Block>,
}

#[derive(Debug)]
pub struct Block {
    pub insn: Vec<Instruction>,
    pub term: Vec<Terminator>,
}

#[derive(Debug)]
pub enum Instruction {
    Copy {
        dst: Id,
        src: Id,
    },
    Const {
        dst: Id,
        src: i64,
    },
    Arith {
        op: BOp,
        dst: Id,
        lhs: Id,
        rhs: Id,
    },
    Read(Id),
    Print(Id),
}

#[derive(Debug)]
pub enum Terminator {
    Exit,
    Jump(Id),
    Branch {
        guard: Id,
        tt: Id,
        ff: Id,
    },
}
