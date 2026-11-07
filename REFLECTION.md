## Question 1 
                                      Answer
In my if_statement(), the relevant lines are " let then_branch = Box::new(self.statement_inner()?); " and " let else_branch = if self.matches(&[TokenType::Else]) { Some(Box::new(self.statement()?)) } else { None }; ". The dangling else is resolved through recursion because the outer if first parses its then-branch completely before checking for an else. The inner if_statement() gets the opportunity to consume the else first, so it binds to the nearest unmatched if. For the given code, my parser prints (if false (if true (print "a") (print "b"))). If the else were to bind to the outer if instead, it would print (if false (if true (print "a")) (print "b")). 


## Question 2
                                        Answer
self.line tracks the current line as the scanner moves through the source. It is incremented in scan_token() on line 112, where a newline is handled with '\n' => self.line += 1, and also in string() on line 137, where self.line += 1 is executed when a newline occurs inside a string. These updates ensure that tokens are assigned to the correct lines while scanning.

However, self.line can continue increasing because of trailing blank lines after the final token. Therefore, in run(), the EOF line is calculated on line 35 using the last token in self.tokens, rather than using self.line directly. For example, if the last real token is ; on line 3 but the file ends with two blank lines, EOF should still be reported on line 3. Using self.tokens.last() gives the line of the last actual token scanned, while self.line may have moved forward because of whitespace. For an empty file, the fallback line is 1.

## Question 3
                                        Answer

In my synchronise() function, I advance past at least one token before checking for a stopping point. I stop after consuming a semicolon (;) or when the next token is Fun, Var, If, While, Print or Return. The semicolon marks the end of a statement, while these keywords can begin a new statement or declaration, allowing the parser to resume from a valid starting point. One test that demonstrates why synchronisation is necessary is expect_expression.kobo, which contains print 1 + * 2;. With synchronisation, the parser skips the invalid expression and stops at the semicolon, producing one error: Expect expression. If I remove the call to synchronise(), the parser remains at the unexpected * token, repeatedly encounters the same error and gets stuck in an infinite loop instead of finishing.