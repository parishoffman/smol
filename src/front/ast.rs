//! The abstract syntax tree.

use crate::common::Id;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(Id, Expr),
    Print(Expr),
    Read(Expr),
    If {
        guard: Expr,
        tt: Vec<Stmt>,
        ff: Vec<Stmt>,
    },
}

#[derive(Debug)]
pub enum Expr {
    Var(Id),
    Const(i64),
    BOp {
        op: BOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Negate(Box<Expr>),
}

#[derive(Debug)]
pub enum BOp {
    Mul,
    Div,
    Add,
    Sub,
    Lt,
}
