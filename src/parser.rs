use crate::ast::{Expr, Function, Program, Resolved, Stmt};
use crate::token::{one_line, Token, TokenType};
use crate::value::Value;

/// Recursive descent: one function per grammar rule, and their call order is the
/// precedence table.
pub fn parse(tokens: Vec<Token>) -> (Program, Vec<String>) {
    let mut p = Parser {
        tokens,
        current: 0,
        depth: 0,
        program: Program::default(),
        errors: Vec::new(),
    };
    while !p.at_end() {
        match p.declaration() {
            Ok(s) => p.program.stmts.push(s),
            Err(()) => p.synchronise(),
        }
    }
    (p.program, p.errors)
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    /// Grammar rules currently on the Rust call stack, capped at `MAX_NESTING` because
    /// recursive descent uses the host's stack (spec section 5.2).
    depth: usize,
    /// `funDecl` appends to `program.functions` as the parse runs; `parse` fills in
    /// `stmts` at the end.
    program: Program,
    errors: Vec<String>,
}

/// No honest program reaches it; the measured stack overflow is past a thousand levels.
const MAX_NESTING: usize = 250;

type Parsed<T> = Result<T, ()>;

impl Parser {
    // --- declarations and statements ------------------------------------------------

    fn declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: a declaration is a fun, a var, or a statement. This is also
        //            where an error is caught and synchronise() runs.
        // TEMPORARY: lets the rules below run until you write this one.
        self.statement()
    }

    fn fun_declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: a function declaration, its parameter list, and its body.
        //            Functions are hoisted into Program::functions.
        todo!("fun_declaration")
    }

    fn var_declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'var' name ( '=' expression )? ';'
        todo!("var_declaration")
    }

    fn statement(&mut self) -> Parsed<Stmt> {
        self.depth += 1;
        let result = self.statement_inner();
        self.depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: pick the statement by its leading token.
        // TEMPORARY: lets the rules below run until you write this one.
        // Named, so an `if` is not misreported as a broken expression.
        if self.check(TokenType::LBrace) || self.check(TokenType::If)
            || self.check(TokenType::While) || self.check(TokenType::Return)
        {
            self.error("This statement form is not written yet.");
            return Err(());
        }
        if self.matches(&[TokenType::Print]) {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
            Ok(Stmt::Print(value))
        } else {
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
            Ok(Stmt::Expression(expr))
        }
    }

    fn return_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'return' expression? ';'
        todo!("return_statement")
    }

    fn if_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1. The dangling else binds to the nearest unmatched if , the
        //            recursion decides that, not the grammar.
        todo!("if_statement")
    }

    fn while_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'while' '(' expression ')' statement
        todo!("while_statement")
    }

    /// The `{` is already consumed. A block holds declarations, not statements (spec
    /// section 3.1).
    fn block(&mut self) -> Parsed<Vec<Stmt>> {
        // TODO(you): spec 3.1: '{' declaration* '}'
        todo!("block")
    }

    // --- expressions, lowest precedence first --------------------------------------

    fn expression(&mut self) -> Parsed<Expr> {
        self.depth += 1;
        let result = if self.depth > MAX_NESTING {
            self.error("Too deeply nested.");
            Err(())
        } else {
            self.assignment()
        };
        self.depth -= 1;
        result
    }

    /// `a = 1` and `a + 1` start the same way, so the left side is parsed as an ordinary
    /// expression and checked when the `=` arrives (spec section 3.2).
    fn assignment(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2. The left side parses as an ordinary expression and is checked
        //            afterwards, because the rule is not LL(1).
        // TEMPORARY: lets the rules below run until you write this one.
        self.or()
    }

    fn or(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: lets the rules below run until you write this one.
        self.and()
    }

    fn and(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: lets the rules below run until you write this one.
        self.equality()
    }

    fn equality(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: lets the rules below run until you write this one.
        self.comparison()
    }

    fn comparison(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: lets the rules below run until you write this one.
        self.term()
    }

    fn term(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        todo!("term")
    }

    fn factor(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        todo!("factor")
    }

    fn unary(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: unary operators are right-associative.
        // TEMPORARY: lets the rules below run until you write this one.
        self.call()
    }

    /// A loop, not recursion: `f(1)(2)` is one callee called twice.
    fn call(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: call -> primary ( '(' arguments? ')' )*. That is a loop, not
        //            recursion; f(1)(2) is one callee called twice.
        // TEMPORARY: lets the rules below run until you write this one.
        self.primary()
    }

    fn primary(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: literals, identifiers and grouping.
        // TEMPORARY: lets the rules below run until you write this one.
        if self.matches(&[TokenType::Number]) {
            let n = self.previous().lexeme.parse::<f64>().unwrap();
            return Ok(self.literal(Value::Number(n)));
        }
        self.error("Expect expression.");
        Err(())
    }

    // --- error recovery ---------------------------------------------------------------

    /// Discard tokens until just past a `;`, or on a keyword that can only begin a
    /// statement (spec section 5.2). It always steps over at least one token first: a
    /// recovery that could stand still would hang on the token that caused the error.
    fn synchronise(&mut self) {
        // TODO(you): spec 5.2: after an error, discard tokens until just past a ';' or on a
        //            statement keyword, so one mistake costs one message.
        // TEMPORARY: lets the rules below run until you write this one.
        self.depth = 0;
        while !self.at_end() {
            self.current += 1;
            if self.previous().kind == TokenType::Semicolon {
                return;
            }
        }
    }

    /// Wraps a value as a literal node carrying the line of the token just consumed.
    fn literal(&self, value: Value) -> Expr {
        Expr::Literal {
            value,
            line: self.previous().line,
        }
    }

    // --- primitives ---------------------------------------------------------------

    fn matches(&mut self, kinds: &[TokenType]) -> bool {
        for k in kinds {
            if self.check(*k) {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.at_end() && self.peek().kind == kind
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, kind: TokenType, message: &str) -> Parsed<Token> {
        if self.check(kind) {
            self.current += 1;
            return Ok(self.previous());
        }
        self.error(message);
        Err(())
    }

    fn error(&mut self, message: &str) {
        let t = self.peek().clone();
        self.error_at(&t, message);
    }

    fn error_at(&mut self, t: &Token, message: &str) {
        let at = if t.kind == TokenType::Eof {
            "end".to_string()
        } else {
            format!("'{}'", one_line(&t.lexeme))
        };
        self.errors
            .push(format!("[line {}] Error at {}: {}", t.line, at, message));
    }
}
