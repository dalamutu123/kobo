use crate::ast::{Expr, Function, Program, Resolved, Stmt};

/// Bind every name to its declaration once, before either engine runs (spec section 7):
/// a depth for the interpreter, a stack slot for the VM. A name that matches no local is a
/// global, and globals stay unresolved.
pub fn resolve(program: &mut Program) {
    // TODO(you): spec 7, 'The resolver': walk every function body in its own frame (4.3), then
    //            the script. A function's body sees its own scopes and the globals and nothing
    //            between.
    todo!("resolve")
}

struct Local {
    name: String,
    scope: usize,
    /// False between `declare` and `define`, so a binding is invisible while its own
    /// initialiser is resolved and `var a = a + 1;` reads the enclosing `a` (section 4.2).
    initialised: bool,
}

/// A flat list, not a map per scope as in *Crafting Interpreters*: spec section 4.2 allows
/// `{ var a = 1; var a = a + 1; }`, two live bindings of `a` in one scope. A binding's
/// index in the list is its slot.
#[derive(Default)]
struct Resolver {
    locals: Vec<Local>,
    depth: usize,
}

impl Resolver {
    /// Returns how many locals the scope held: the number of `Pop`s the bytecode
    /// compiler owes for leaving it.
    fn end_scope(&mut self) -> usize {
        // TODO(you): close a scope and report how many locals died in it, which is how many
        //            Pops the bytecode compiler owes.
        todo!("end_scope")
    }

    fn declare(&mut self, name: &str) {
        // TODO(you): introduce a binding that is not usable yet; its own initialiser must not
        //            see it (4.2).
        todo!("declare")
    }

    /// `None` at the top level of the script, where a declaration is a global.
    fn define(&mut self, name: &str) -> Option<usize> {
        // TODO(you): make it usable, and hand back its stack slot.
        todo!("define")
    }

    /// Innermost first, so the nearest declaration wins and an outer one stays shadowed.
    fn lookup(&self, name: &str) -> Resolved {
        // TODO(you): innermost binding first, skipping any still initialising. A name that
        //            matches no local is a global (spec 7).
        todo!("lookup")
    }

    fn statements(&mut self, stmts: &mut [Stmt]) {
        for stmt in stmts {
            self.statement(stmt);
        }
    }

    fn statement(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Expression(e) | Stmt::Print(e) => self.expression(e),

            Stmt::Var {
                name,
                initialiser,
                slot,
            } => {
                // Declare, resolve, define: in that order (spec section 4.2).
                self.declare(&name.lexeme);
                if let Some(e) = initialiser {
                    self.expression(e);
                }
                *slot = self.define(&name.lexeme);
            }

            Stmt::Block { body, locals } => {
                self.depth += 1;
                self.statements(body);
                *locals = self.end_scope();
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expression(condition);
                self.statement(then_branch);
                if let Some(alt) = else_branch {
                    self.statement(alt);
                }
            }

            Stmt::While { condition, body } => {
                self.expression(condition);
                self.statement(body);
            }

            // The body was resolved in its own frame above; only the name binds here.
            Stmt::Function { name, slot, .. } => {
                self.declare(name);
                *slot = self.define(name);
            }

            Stmt::Return(value) => {
                if let Some(e) = value {
                    self.expression(e);
                }
            }
        }
    }

    fn expression(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Literal { .. } => {}
            Expr::Grouping(inner) => self.expression(inner),

            Expr::Variable { name, at } => *at = self.lookup(&name.lexeme),

            Expr::Assign { name, value, at } => {
                self.expression(value);
                *at = self.lookup(&name.lexeme);
            }

            Expr::Call { callee, args, .. } => {
                self.expression(callee);
                for arg in args {
                    self.expression(arg);
                }
            }

            Expr::Unary { right, .. } => self.expression(right),

            Expr::Binary { left, right, .. } | Expr::Logical { left, right, .. } => {
                self.expression(left);
                self.expression(right);
            }
        }
    }
}
