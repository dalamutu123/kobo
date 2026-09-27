// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use crate::chunk::{Bytecode, Chunk, Op};

/// What `--disasm` prints. Each chunk is headed `== name/arity ==`; each line is
/// `offset line opcode operand`, with `|` when the line repeats.
pub fn disassemble(bytecode: &Bytecode) -> String {
    let mut out = chunk(&bytecode.script);
    for c in &bytecode.functions {
        out.push('\n');
        out.push_str(&chunk(c));
    }
    out
}

fn chunk(c: &Chunk) -> String {
    let mut out = format!("== {}/{} ==\n", c.name, c.arity);
    for (offset, op) in c.code.iter().enumerate() {
        let line = if offset > 0 && c.lines[offset] == c.lines[offset - 1] {
            "   |".to_string()
        } else {
            format!("{:>4}", c.lines[offset])
        };
        out.push_str(&format!("{:04} {} {}\n", offset, line, instruction(c, *op)));
    }
    out
}

fn instruction(c: &Chunk, op: Op) -> String {
    match op {
        Op::Constant(i) => format!("{:<14} {:>4} '{}'", "CONSTANT", i, c.constants[i]),
        Op::DefineGlobal(i) => format!("{:<14} {:>4} '{}'", "DEFINE_GLOBAL", i, c.constants[i]),
        Op::GetGlobal(i) => format!("{:<14} {:>4} '{}'", "GET_GLOBAL", i, c.constants[i]),
        Op::SetGlobal(i) => format!("{:<14} {:>4} '{}'", "SET_GLOBAL", i, c.constants[i]),
        Op::GetLocal(i) => format!("{:<14} {:>4}", "GET_LOCAL", i),
        Op::SetLocal(i) => format!("{:<14} {:>4}", "SET_LOCAL", i),
        Op::Jump(t) => format!("{:<14} {:>4}", "JUMP", t),
        Op::JumpIfFalse(t) => format!("{:<14} {:>4}", "JUMP_IF_FALSE", t),
        Op::Call(n) => format!("{:<14} {:>4}", "CALL", n),
        Op::Nil => "NIL".to_string(),
        Op::True => "TRUE".to_string(),
        Op::False => "FALSE".to_string(),
        Op::Pop => "POP".to_string(),
        Op::Equal => "EQUAL".to_string(),
        Op::NotEqual => "NOT_EQUAL".to_string(),
        Op::Greater => "GREATER".to_string(),
        Op::GreaterEqual => "GREATER_EQUAL".to_string(),
        Op::Less => "LESS".to_string(),
        Op::LessEqual => "LESS_EQUAL".to_string(),
        Op::Add => "ADD".to_string(),
        Op::Subtract => "SUBTRACT".to_string(),
        Op::Multiply => "MULTIPLY".to_string(),
        Op::Divide => "DIVIDE".to_string(),
        Op::Not => "NOT".to_string(),
        Op::Negate => "NEGATE".to_string(),
        Op::Print => "PRINT".to_string(),
        Op::Return => "RETURN".to_string(),
    }
}
