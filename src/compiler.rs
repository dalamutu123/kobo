use crate::ast::{Expr, Program, Resolved, Stmt};
use crate::chunk::{Bytecode, Chunk, Op};
use crate::token::TokenType;
use crate::value::Value;

/// AST to bytecode in one pass. Local slots come from `resolver.rs`, which has already
/// written them into the tree.
pub fn compile(program: &Program) -> Bytecode {
    let functions = program
        .functions
        .iter()
        .map(|f| {
            // Parameters are the frame's first locals, so argument `i` is already in
            // slot `i` when the body starts.
            let mut c = Compiler::new(Chunk::new(f.name.clone(), f.params.len()), program);
            c.in_function = true;
            c.statements(&f.body);
            c.finish(f.line)
        })
        .collect();

    let mut script = Compiler::new(Chunk::new("script".to_string(), 0), program);
    script.statements(&program.stmts);
    let line = script.chunk.last_line();
    Bytecode {
        script: script.finish(line),
        functions,
    }
}

struct Compiler<'a> {
    chunk: Chunk,
    program: &'a Program,
    /// A function body's declarations are locals even at its outermost level; the
    /// script's are globals.
    in_function: bool,
}

impl<'a> Compiler<'a> {
    fn new(chunk: Chunk, program: &'a Program) -> Self {
        Compiler {
            chunk,
            program,
            in_function: false,
        }
    }

    /// Every chunk ends by returning `nil`, so falling off the end of a function returns
    /// `nil` (spec section 4.3).
    fn finish(mut self, line: usize) -> Chunk {
        self.chunk.write(Op::Nil, line);
        self.chunk.write(Op::Return, line);
        self.chunk
    }

    fn statements(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.statement(stmt);
        }
    }

    // --- statements -------------------------------------------------------------------

    fn statement(&mut self, stmt: &Stmt) {
        // TODO(you): emit code for one statement.
        todo!("statement")
    }

    // --- expressions ------------------------------------------------------------------

    /// Emit `expr`, and return the line for the caller's next instruction.
    fn expression(&mut self, expr: &Expr) -> usize {
        // TODO(you): emit code for one expression, and return the line it came from.
        todo!("expression")
    }

    // --- names ------------------------------------------------------------------------

    /// A global is written into the globals map. A local is already on the stack in the
    /// slot the resolver gave it, so declaring one emits nothing.
    fn declare(&mut self, name: &str, slot: Option<usize>, line: usize) {
        // TODO(you): a global is written into the globals map; a local is already on the stack
        //            in the slot resolver.rs gave it, so it emits nothing.
        todo!("declare")
    }

    fn name_constant(&mut self, name: &str) -> usize {
        self.chunk.constant(Value::Str(name.to_string()))
    }

    /// Leaving a scope pops its locals, one `Pop` each; the count is on the `Block`.
    fn end_scope(&mut self, locals: usize) {
        // TODO(you): leave a scope: one Pop per local that dies here, and the resolver already
        //            counted them onto the Block.
        todo!("end_scope")
    }

    fn patch(&mut self, at: usize) {
        // TODO(you): fill in a forward jump, whose target was not known when it was emitted.
        todo!("patch")
    }
}
