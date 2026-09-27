// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use crate::value::Value;

/// One instruction of the Kobo virtual machine. The operand lives inside the variant
/// rather than in packed bytes; nothing in this course turns on the encoding.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    /// Push `constants[i]`.
    Constant(usize),
    Nil,
    True,
    False,
    /// Discard the top of the stack. Every expression statement ends with one.
    Pop,

    /// The operand indexes the constant holding the variable's name: a global is found
    /// by name at run time.
    DefineGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    /// A slot in the current frame's window of the stack, fixed at compile time.
    GetLocal(usize),
    SetLocal(usize),

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,

    /// An absolute index into `code`, not a relative offset, so a disassembly shows
    /// the target directly.
    Jump(usize),
    /// Jumps if the top of the stack is falsey, and leaves it there: `and` and `or` need
    /// the value (spec section 4.1), so `if` and `while` emit their own `Pop`.
    JumpIfFalse(usize),
    /// The operand is the argument count. The callee sits under the arguments.
    Call(usize),
    Return,
}

/// A compiled body: the script, or one function.
pub struct Chunk {
    pub name: String,
    pub arity: usize,
    pub code: Vec<Op>,
    /// `lines[i]` is the source line of `code[i]`: where a VM runtime error gets its
    /// `[line N]`.
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new(name: String, arity: usize) -> Self {
        Chunk {
            name,
            arity,
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Append an instruction; returns its offset, which is what jump patching needs.
    pub fn write(&mut self, op: Op, line: usize) -> usize {
        self.code.push(op);
        self.lines.push(line);
        self.code.len() - 1
    }

    /// Add a constant, reusing an equal one to keep the table small.
    pub fn constant(&mut self, value: Value) -> usize {
        if let Some(i) = self.constants.iter().position(|c| *c == value) {
            return i;
        }
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn last_line(&self) -> usize {
        *self.lines.last().unwrap_or(&1)
    }
}

/// Everything a program compiles to. `functions[i]` is the chunk for
/// `Program::functions[i]`, so a function value means the same thing in both engines.
pub struct Bytecode {
    pub script: Chunk,
    pub functions: Vec<Chunk>,
}
