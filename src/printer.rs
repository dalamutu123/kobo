// GIVEN: do not edit. Read it anyway; what you write has to fit it.

use crate::ast::{Expr, Program, Stmt};
use crate::token::one_line;
use crate::value::Value;

/// What `--ast` prints (spec section 6.1): one statement per line, as prefix text.
///
///     print 2 + 3 * 4;   →   (print (+ 2 (* 3 4)))
pub fn statement(stmt: &Stmt, program: &Program) -> String {
    match stmt {
        Stmt::Expression(e) => format!("(expr {})", expression(e)),
        Stmt::Print(e) => format!("(print {})", expression(e)),
        Stmt::Var {
            name, initialiser, ..
        } => match initialiser {
            Some(e) => format!("(var {} {})", name.lexeme, expression(e)),
            None => format!("(var {})", name.lexeme),
        },
        Stmt::Block { body, .. } => format!("(block{})", parts(body, program)),
        Stmt::Function { index, .. } => {
            let f = &program.functions[*index];
            format!(
                "(fun {} ({}){})",
                f.name,
                f.params.join(" "),
                parts(&f.body, program)
            )
        }
        Stmt::Return(value) => match value {
            Some(e) => format!("(return {})", expression(e)),
            None => "(return)".to_string(),
        },
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => match else_branch {
            Some(alt) => format!(
                "(if {} {} {})",
                expression(condition),
                statement(then_branch, program),
                statement(alt, program)
            ),
            None => format!(
                "(if {} {})",
                expression(condition),
                statement(then_branch, program)
            ),
        },
        Stmt::While { condition, body } => {
            format!(
                "(while {} {})",
                expression(condition),
                statement(body, program)
            )
        }
    }
}

/// Each part preceded by a space, so an empty block prints `(block)` and not `(block )`.
fn parts(stmts: &[Stmt], program: &Program) -> String {
    stmts
        .iter()
        .map(|s| format!(" {}", statement(s, program)))
        .collect()
}

fn expression(expr: &Expr) -> String {
    match expr {
        Expr::Literal { value, .. } => literal(value),
        Expr::Grouping(inner) => format!("(group {})", expression(inner)),
        Expr::Variable { name, .. } => name.lexeme.clone(),
        Expr::Assign { name, value, .. } => {
            format!("(= {} {})", name.lexeme, expression(value))
        }
        Expr::Call { callee, args, .. } => {
            let rendered: String = args.iter().map(|a| format!(" {}", expression(a))).collect();
            format!("(call {}{})", expression(callee), rendered)
        }
        Expr::Unary { op, right } => format!("({} {})", op.lexeme, expression(right)),
        Expr::Binary { left, op, right } | Expr::Logical { left, op, right } => {
            format!("({} {} {})", op.lexeme, expression(left), expression(right))
        }
    }
}

fn literal(value: &Value) -> String {
    match value {
        // Quoted, so `"1"` in the source is never confused with `1` in the dump.
        Value::Str(s) => format!("\"{}\"", one_line(s)),
        other => other.to_string(),
    }
}
