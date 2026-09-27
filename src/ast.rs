// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use crate::token::Token;
use crate::value::Value;

/// A parsed file: the top-level statements, and every function hoisted into one table.
/// A `Value` names a function by its index here, and the bytecode compiler uses the
/// same index for the function's chunk.
#[derive(Debug, Default)]
pub struct Program {
    pub stmts: Vec<Stmt>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    /// Where `fun` was written: the line of the implicit `return nil` at the end.
    pub line: usize,
}

/// Filled in by `resolver.rs`. The parser writes `Global`, which is also the answer for a
/// name that resolves to no local (spec section 7).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Resolved {
    #[default]
    Global,
    Local {
        /// Scopes out from the innermost. The interpreter indexes with it.
        depth: usize,
        /// Position in the call frame's stack values. The VM indexes with it.
        slot: usize,
    },
}

#[derive(Debug)]
pub enum Expr {
    /// The line is for the bytecode compiler, which gives every instruction one.
    Literal {
        value: Value,
        line: usize,
    },
    Grouping(Box<Expr>),
    /// The token, not just the name: a runtime error needs its line.
    Variable {
        name: Token,
        at: Resolved,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
        at: Resolved,
    },
    Call {
        callee: Box<Expr>,
        /// The closing `)`: arity and not-a-function errors are reported on its line.
        paren: Token,
        args: Vec<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        /// `None` when no initialiser was written; it evaluates to `nil` (spec section 3.1).
        initialiser: Option<Expr>,
        /// The stack slot the resolver gave it, or `None` for a global.
        slot: Option<usize>,
    },
    Block {
        body: Vec<Stmt>,
        /// How many locals the block declares: the `Pop`s the bytecode compiler emits on
        /// leaving it.
        locals: usize,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        /// A dangling `else` binds to the nearest unmatched `if` (spec section 3.1), because
        /// recursive descent takes it as soon as it sees it.
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    /// An index into `Program::functions`. Executing it binds the name in the current
    /// scope.
    Function {
        index: usize,
        /// The name, so the resolver can bind it without reaching into the table.
        name: String,
        slot: Option<usize>,
    },
    Return(Option<Expr>),
}
