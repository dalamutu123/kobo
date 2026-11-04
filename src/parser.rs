use crate::ast::{Expr, Function, Program, Resolved, Stmt};
use crate::token::{one_line, Token, TokenType};
use crate::value::{self, Value};

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
        
        if self.matches(&[TokenType::Fun]) {
            return self.fun_declaration();
        }
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }
        
        self.statement()
    }

    fn fun_declaration(&mut self) -> Parsed<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect function name.")?;
        self.consume(TokenType::LParen, "Expect '(' after function name.")?;

        let mut parameters = Vec::new();

        if !self.check(TokenType::RParen) {
            loop {
                let param = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(param.lexeme);

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LBrace, "Expect '{' before function body.")?;

        let body = self.block()?;

        let function = Function {
            name: name.lexeme.clone(),
            params: parameters,
            body,
            line: name.line,
        };

        let index = self.program.functions.len();
        self.program.functions.push(function);

        Ok(Stmt::Function {
            index,
            name: name.lexeme,
            slot: None,
        })
}

    fn var_declaration(&mut self) -> Parsed<Stmt> {
        
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initialiser = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var {
            name,
            initialiser,
            slot: None,
        })
    }

    fn statement(&mut self) -> Parsed<Stmt> {
        self.depth += 1;
        let result = self.statement_inner();
        self.depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Parsed<Stmt> {
        if self.matches(&[TokenType::Print]) {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
            return Ok(Stmt::Print(value));
        }

        if self.matches(&[TokenType::LBrace]) {
            let body = self.block()?;
            return Ok(Stmt::Block {
            body,
            locals: 0,
            });
        }

        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matches(&[TokenType::Return]) {
            return self.return_statement();
        }

        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
}


    fn return_statement(&mut self) -> Parsed<Stmt> {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(value))
    }

    fn if_statement(&mut self) -> Parsed<Stmt> {
        self.consume(TokenType::LParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement_inner()?);

        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement_inner()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Parsed<Stmt> {
       self.consume(TokenType::LParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(TokenType::RParen, "Expect ')' after while condition.")?;

        let body = Box::new(self.statement_inner()?);

        Ok(Stmt::While { 
            condition,
            body,
        })
    }

    /// The `{` is already consumed. A block holds declarations, not statements (spec
    /// section 3.1).
    fn block(&mut self) -> Parsed<Vec<Stmt>> {
        let mut statements = Vec::new();

    while !self.check(TokenType::RBrace) && !self.at_end() {        
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        self.consume(TokenType::RBrace, "Expect '}' after block.")?;

        Ok(statements)
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
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name, at } => {
                    Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                        at,
                    })
                }
                _ => Err(self.error( "Invalid assignment target.")),
            }
        } else {
            Ok(expr)
        }
}

    fn or(&mut self) -> Parsed<Expr> {
        let mut expr = self.and()?;
        
        while self.matches(&[TokenType::Or]) {
            let op = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Parsed<Expr> {
        let mut expr = self.equality()?;
        
        while self.matches(&[TokenType::And]) {
            let op = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Parsed<Expr> {
        let mut expr = self.comparison()?;

        while self.matches(&[
        TokenType::BangEqual,
        TokenType::EqualEqual,
        ]) {
            let op = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Parsed<Expr> {
        let mut expr = self.term()?;
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Parsed<Expr> {
        let mut expr = self.factor()?;

        while self.matches(&[
            TokenType::Minus,
            TokenType::Plus,
        ]) {
            let op = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Parsed<Expr> {
        let mut expr = self.unary()?;

        while self.matches(&[
            TokenType::Slash,
            TokenType::Star,
        ]) {
            let op = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Parsed<Expr> {
        if self.matches(&[
            TokenType::Bang,
            TokenType::Minus,
        ]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op,
                right: Box::new(right),
            });
        }
        self.call()
    }

    /// A loop, not recursion: `f(1)(2)` is one callee called twice.
    fn call(&mut self) -> Parsed<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LParen]) {
                let mut arguments = Vec::new();
                if !self.check(TokenType::RParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.matches(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }
                let paren = self.consume(TokenType::RParen, "Expect ')' after arguments.")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    paren,
                    args: arguments,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Parsed<Expr> {
        if self.matches(&[TokenType::False]) {
            return Ok(self.literal(Value::Bool(false)));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(self.literal(Value::Bool(true)));
        }

        if self.matches(&[TokenType::Nil]) {
            return Ok(self.literal(Value::Nil));
        }

        if self.matches(&[TokenType::Number]) {
            let token = self.previous();
            let value = Value::Number(token.lexeme.parse().unwrap());
            return Ok(self.literal(value));
        }

        if self.matches(&[TokenType::Str]) {
            let token = self.previous();
            let value = Value::Str(token.lexeme[1..token.lexeme.len() - 1].to_string());
            return Ok(self.literal(value));
        }

        if self.matches(&[TokenType::Identifier]) {
            let token = self.previous();
            return Ok(Expr::Variable {
                name: token,
                at: Resolved::Global,
            });
        }

        if self.matches(&[TokenType::LParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
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
