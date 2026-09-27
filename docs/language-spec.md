# Kobo — language specification

**CSC 312 / SEN 313 · Compiler Construction · 2026/27 Semester 1**

Kobo is the language you will implement this semester. It is small enough to finish and
large enough to be real: it has values, variables, scope, control flow and functions, and
a program written in it runs.

This document is the contract. Your compiler is correct when it agrees with this document,
and the automated marking checks exactly that — nothing else. It never inspects your source
code, your class names or your AST design. It runs your program and reads what it printed.

Source files use the extension `.kobo`.

## 1. Lexical structure

### 1.1 Whitespace and comments

Space, tab, carriage return and newline separate tokens and are otherwise ignored.
Newlines increment the line counter used in error messages.

A comment runs from `//` to the end of the line and is discarded.

```kobo
// this whole line is a comment
print 1;   // and this tail is too
```

There are no block comments.

### 1.2 Tokens

There are 34 token types.

These are the type names, and they are what `--tokens` prints.

| Group | Token types | Written as |
|---|---|---|
| Punctuation | `LPAREN` `RPAREN` `LBRACE` `RBRACE` `COMMA` `SEMICOLON` | `(` `)` `{` `}` `,` `;` |
| Arithmetic | `PLUS` `MINUS` `STAR` `SLASH` | `+` `-` `*` `/` |
| Equality and comparison | `BANG_EQUAL` `EQUAL_EQUAL` `GREATER` `GREATER_EQUAL` `LESS` `LESS_EQUAL` | `!=` `==` `>` `>=` `<` `<=` |
| Negation and assignment | `BANG` `EQUAL` | `!` `=` |
| Literals | `IDENTIFIER` `STRING` `NUMBER` | any name, any `"…"`, any number |
| Keywords | `AND` `ELSE` `FALSE` `FUN` `IF` `NIL` `OR` `PRINT` `RETURN` `TRUE` `VAR` `WHILE` | `and` `else` `false` `fun` `if` `nil` `or` `print` `return` `true` `var` `while` |
| End | `EOF` | — |

**Every keyword has its own token type.** There is no single `KEYWORD` type: `while` scans
as `WHILE`, `var` as `VAR`, and so on for all twelve. Some languages do it the other way,
with one `KEYWORD` type carrying the word as an attribute; Kobo does not, and the reason
is the parser. `statement()` decides what to parse by looking at one token type — seeing
`WHILE` *is* the decision. With a single `KEYWORD` type it would have to look at the type
and then at the attribute, which is two questions where the grammar asks one.

The eight one- and two-character operator tokens — `!` `!=` `=` `==` `>` `>=` `<` `<=` —
are the reason the scanner needs one character of lookahead: on reading `!` it must peek to
decide between `BANG` and `BANG_EQUAL`. `/` needs the same peek, to tell a division from the
start of a comment. Section 1.4's number rule needs **two**: on reading `.` the scanner must
know whether a digit follows before it consumes the dot, which is what `peek_next` is for.

### 1.3 Identifiers

An identifier starts with a letter or `_` and continues with letters, digits or `_`.
Identifiers are case-sensitive. A word that matches a keyword in the table above scans as
**that keyword's own token type** and never as `IDENTIFIER` — the scanner reads the whole
word first, then looks it up.

### 1.4 Numbers

A number is one or more digits, optionally followed by `.` and one or more digits.

```
123      12.75      0.5
```

`.5` is not a number, and it does not scan as anything: **`.` begins no token in Kobo** —
section 1.2's thirty-four types have no `DOT` — so the scanner reports
`Character is not part of any token.` and carries on scanning. As section 5 says, it reports
every bad character in the file, and then the compiler stops before parsing. `5.` is not a number either: `5` is
scanned, then the `.` reports the same error. There are no leading `+` or `-` signs in a
number literal; `-5` is unary minus applied to `5`.

**There is no exponent notation.** `1e308` is not one number: it is `NUMBER 1` followed by
`IDENTIFIER e308`, and `print 1e308;` is a *parse* error — `Expect ';' after value.` A
number is digits and at most one dot, and that is the whole rule.

All numbers are IEEE-754 double-precision. There is no integer type. A literal too large
for a double becomes `inf` rather than an error — see section 2.2.

### 1.5 Strings

A string is any run of characters between two `"`. Strings may span lines, and a newline
inside one increments the line counter. There are no escape sequences — `\n` inside a
string is a backslash followed by an `n`.

An unterminated string is an error reported at the line where the string *started*.

## 2. Values

Kobo has five kinds of value.

| Kind | Written | Notes |
|---|---|---|
| Nil | `nil` | the absence of a value |
| Boolean | `true` `false` | |
| Number | `1`, `12.75` | IEEE-754 double |
| String | `"hello"` | immutable |
| Function | — | produced by `fun`; has no literal form |

Kobo is **dynamically typed**: a variable holds whatever it was last assigned, and type
errors are found when the offending line runs, not before.

### 2.1 Truthiness

`false` and `nil` are false. **Every other value is true**, including `0` and `""`.

### 2.2 How values print

`print` is the only way a program produces output. Getting this exactly right matters more
than it looks — the marking compares your output byte for byte.

| Value | Printed as | Reach it with |
|---|---|---|
| `nil` | `nil` | `print nil;` |
| `true` / `false` | `true` / `false` | `print 1 < 2;` |
| A number with no fractional part | `14`, `-3`, `0` — **no trailing `.0`** | `print 14;` |
| Any other number | the shortest decimal that reads back as the same double: `12.75`, `0.30000000000000004` | `print 0.1 + 0.2;` |
| **Negative zero** | `-0` | `print -0;` · `print 0 * -1;` |
| **Infinity** | `inf` and `-inf` | a literal past the range of a double, or arithmetic that overflows |
| **Not a number** | `NaN` | `inf - inf`, `inf / inf` |
| A string | its characters, with no surrounding quotes | `print "a";` |
| A function | `<fn name>` | `fun f() {} print f;` |

Each `print` emits its text followed by a single `\n`.

**The last three rows are Rust's `f64` `Display`, and they are contractual like every
other row.** They are reachable, so you will meet them:

```kobo
print -0;                            // -0     — and -0 == 0 is true
print <a 1 followed by 309 zeros>;   // inf    — too large for a double
print -<a 1 followed by 309 zeros>;  // -inf
var big = <a 1 followed by 308 zeros>;  // finite, just
print big - big;                     // 0
```

Two consequences worth knowing before a test surprises you. **`-0` prints with its sign
but compares equal to `0`**, because IEEE-754 says so. And **dividing by `-0` is
`Division by zero.`** (section 4.1) — the check is *is this zero*, not *is this positive zero*.
`NaN` is only reachable through infinity, since `0 / 0` is a runtime error rather than
`NaN`.

## 3. Grammar

Written in the notation used throughout this course: `*` is zero or more, `?` is optional,
`|` is alternation, quoted text is a terminal, `UPPERCASE` is a token class.

### 3.1 Declarations and statements

```
program     → declaration* EOF

declaration → funDecl
            | varDecl
            | statement

funDecl     → "fun" IDENTIFIER "(" parameters? ")" block
parameters  → IDENTIFIER ( "," IDENTIFIER )*

varDecl     → "var" IDENTIFIER ( "=" expression )? ";"

statement   → exprStmt
            | printStmt
            | ifStmt
            | whileStmt
            | returnStmt
            | block

exprStmt    → expression ";"
printStmt   → "print" expression ";"
ifStmt      → "if" "(" expression ")" statement ( "else" statement )?
whileStmt   → "while" "(" expression ")" statement
returnStmt  → "return" expression? ";"
block       → "{" declaration* "}"
```

A `var` with no initialiser binds `nil`. A `return` with no expression returns `nil`. A
`return` outside any function ends the program, successfully — the script is the outermost
body, and stopping early is not an error.

`if` and `while` take a *statement* as their body, so both of these are legal and mean the
same thing:

```kobo
if (x > 0) print "positive";
if (x > 0) { print "positive"; }
```

The dangling `else` binds to the **nearest** unmatched `if`. This falls out of
recursive descent for free, which is one of the reasons week 5 uses it.

### 3.2 Expressions

Rules are listed **lowest precedence first**. Each rule calls the one below it, so the
grammar itself encodes the precedence table — this is the whole trick of week 5.

```
expression  → assignment

assignment  → IDENTIFIER "=" assignment
            | logic_or

logic_or    → logic_and ( "or" logic_and )*
logic_and   → equality ( "and" equality )*
equality    → comparison ( ( "!=" | "==" ) comparison )*
comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )*
term        → factor ( ( "-" | "+" ) factor )*
factor      → unary ( ( "/" | "*" ) unary )*
unary       → ( "!" | "-" ) unary
            | call
call        → primary ( "(" arguments? ")" )*
arguments   → expression ( "," expression )*

primary     → NUMBER | STRING
            | "true" | "false" | "nil"
            | IDENTIFIER
            | "(" expression ")"
```

Summarised:

| Precedence | Operators | Associativity |
|---|---|---|
| 1 (lowest) | `=` | right |
| 2 | `or` | left |
| 3 | `and` | left |
| 4 | `==` `!=` | left |
| 5 | `<` `<=` `>` `>=` | left |
| 6 | `+` `-` | left |
| 7 | `*` `/` | left |
| 8 | `!` `-` (unary) | right |
| 9 (highest) | `()` call, grouping | left |

Assignment is an **expression**, not a statement, so `a = b = 0;` is legal and assigns
right to left. Its left side must be a bare identifier; `1 = 2;` and `f() = 3;` are
compile-time errors reported as *Cannot assign to this expression.*

## 4. Semantics

### 4.1 Operators

| Operator | Operands | Result |
|---|---|---|
| `-` unary | number | negation |
| `!` | any | `true` if the operand is falsey |
| `*` `/` `-` | two numbers | arithmetic |
| `+` | two numbers | sum |
| `+` | two strings | concatenation |
| `<` `<=` `>` `>=` | two numbers | boolean |
| `==` `!=` | any two | see below |

For an operator that evaluates both of its operands, the left operand is evaluated first.

Any other combination is a runtime error. `"a" + 1` is an error; Kobo never converts
implicitly.

`/` by zero is a runtime error, not infinity.

**Equality.** Values of different kinds are never equal, so `1 == "1"` is `false` and
never an error. Within a kind, `nil == nil` is `true`, booleans and numbers compare by
value, and strings compare by contents.

**`and` and `or` short-circuit** and evaluate to *one of their operands*, not to a
boolean. `or` returns its left operand if that operand is truthy, otherwise its right.
`and` returns its left operand if that operand is falsey, otherwise its right. So
`nil or "fallback"` is `"fallback"` and `0 and 1` is `1`.

### 4.2 Variables and scope

Kobo is **lexically scoped**. A block introduces a scope; a name declared inside a block
is gone at the closing brace. An inner declaration of a name already in an outer scope
shadows it for the rest of the block.

Declarations at the top level of the program are **global**. A global may be redeclared;
the later `var` replaces the earlier binding.

**A local may be redeclared too, in the same scope, and the later `var` wins.** Kobo does
not reject `var a = 1; var a = 2;` inside a block any more than it rejects it at the top
level — there is no *already declared in this scope* error anywhere in section 5.1, and there is
not meant to be. It is the same rule in both places, which is what makes it one rule:

```kobo
{
  var a = "first";
  var a = "second";
  print a;          // second
}
```

A declaration's initialiser is evaluated **before** the name it declares is bound, so an
initialiser that mentions its own name reads the *previous* binding — the one in an
enclosing scope, or an earlier declaration of the same name in the same scope:

```kobo
var a = 1;
{
  var a = a + 1;    // the `a` on the right is the outer one
  print a;          // 2
  var a = a + 1;    // and this one reads the local just above it
  print a;          // 3
}
print a;            // 1
```

Both lines are the same rule: the name on the left does not exist until its initialiser
has been evaluated. This is the one place where a scope holds two live bindings of one
name at once, and it is why section 7's resolver cannot be a plain map from name to scope.

Assigning to a name that was never declared is a runtime error. Reading one is too.

```kobo
var a = "outer";
{
  var a = "inner";
  print a;          // inner
}
print a;            // outer
```

### 4.3 Functions

A function is declared with `fun`, called with `()`, and returns `nil` if control reaches
its closing brace without a `return`.

Arguments are evaluated left to right, before the call is made. Calling with the wrong
number of arguments is a runtime error. Calling something that is
not a function is a runtime error.

**A function body sees its own parameters and locals, and the globals. It does not see
the caller's locals, and it does not capture the scope it was declared in.** Kobo has no
closures — this is a deliberate simplification and the single largest thing separating it
from a production language. It is what lets the interpreter hold its scopes in one flat
stack instead of a graph of heap-allocated environments, and week 9 explains exactly what
that buys and what it costs.

Recursion works, because a function can always see itself through the global scope.

```kobo
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}
print fib(20);      // 6765
```

**Duplicate parameter names are accepted, and the last one wins.** `fun f(a, a)` is not
an error — section 4.2's redeclaration rule applied to a parameter list, since parameters are
locals of the function's own scope — so `f(1, 2)` returns `2`. It is not useful and no
test requires you to write it; it is written down because it is what falls out of the
rule, and a compiler that rejected it would need an error message that section 5.1 does not
have.

There are no default arguments, no varargs, no anonymous functions, and functions are
values only in the limited sense that a name can hold one.

### 4.4 Not in Kobo

No classes, inheritance or methods. No closures. No `for`. No arrays, maps or any other
compound data. No modules or imports. No standard library beyond `print`. No `break` or
`continue`.

## 5. Errors

Every error message goes to **standard error**, never standard output, and matches one of
four shapes exactly. The automated marking matches these strings, so a message that is
right in spirit and wrong in punctuation fails.

| Phase | Shape |
|---|---|
| Scanning | `[line N] Error: <message>` |
| Parsing | `[line N] Error at '<lexeme>': <message>` |
| Parsing, at end of file | `[line N] Error at end: <message>` — where `N` is the line of the **last real token**, not the last line of the file |
| Runtime | `[line N] Runtime error: <message>` |

Exit codes follow the same split:

| Outcome | Exit code |
|---|---|
| Ran to completion | `0` |
| Any scan or parse error | `65` |
| A runtime error | `70` |

A program with a scan or parse error is **never executed**. Scanning reports every bad
character and unterminated string in the file; if it found any, the compiler stops there
and does not parse, because a broken token stream only produces invented parse errors on
top. Parsing then reports every error it can recover from (section 5.2). A runtime error stops
execution at once, after whatever the program had already printed.

### 5.1 The catalogue

These are every message the compiler produces, and the tests match them exactly.
Reproduce them character for character — a message that is right in spirit and wrong in
punctuation fails.

**Scanning**

```
[line 1] Error: Character is not part of any token.
[line 1] Error: String is never closed.
```

**Parsing**

```
[line 1] Error at '+': Expect expression.
[line 1] Error at end: Expect expression.
[line 1] Error at ';': Expect ')' after expression.
[line 1] Error at '123': Expect variable name.
[line 2] Error at '=': Cannot assign to this expression.
[line 3] Error at '}': Expect ';' after value.
[line 3] Error at '}': Expect ';' after expression.
[line 1] Error at ';': Expect ';' after variable declaration.
[line 4] Error at end: Expect '}' after block.
[line 1] Error at 'x': Expect '(' after 'if'.
[line 1] Error at 'print': Expect ')' after if condition.
[line 1] Error at 'x': Expect '(' after 'while'.
[line 1] Error at 'print': Expect ')' after while condition.
[line 1] Error at '(': Too deeply nested.
[line 1] Error at '(': Expect function name.
[line 1] Error at 'x': Expect '(' after function name.
[line 1] Error at ')': Expect parameter name.
[line 1] Error at 'x': Expect ')' after parameters.
[line 1] Error at 'print': Expect '{' before function body.
[line 1] Error at ';': Expect ')' after arguments.
[line 2] Error at end: Expect ';' after return value.
```

That is the whole parse catalogue. A message not on this list is a message your parser
should not produce.

**Runtime**

```
[line 1] Runtime error: Operands must be numbers.
[line 1] Runtime error: Operands must be two numbers or two strings.
[line 1] Runtime error: Operand must be a number.
[line 1] Runtime error: Division by zero.
[line 1] Runtime error: Variable 'x' is not defined.
[line 1] Runtime error: This value is not a function.
[line 1] Runtime error: Expected 2 arguments but got 3.
[line 1] Runtime error: Stack overflow.
```

That is the whole runtime catalogue. `Stack overflow.` is reported when more than 500
calls are active at once: Kobo's call stack is your compiler's own call stack, and running
off the end of it would kill the process without a message. No sensible program comes near
the limit; runaway recursion reaches it at once.

### 5.2 Error recovery

After a parse error the parser **synchronises**: it always discards at least one token
first, then keeps discarding until it has just consumed a `;` or is positioned at one of
`fun` `var` `if` `while` `print` `return`, and resumes. The first discard is what guarantees
progress: a parser that could stop on the token that caused the error would parse it again,
forever. This is why one missing semicolon produces one error rather than forty. Week 7
builds it.

**Nesting has a limit.** Recursive descent runs on the host language's own call stack, so
a source file nested deeply enough will exhaust it and kill the process. The parser
therefore refuses past 250 levels of nested grammar rules with *Too deeply nested.* No
honest program comes near it, and no test uses it; it is here because a compiler that
crashes on its input is worse than one that complains about it. A table-driven parser has
no such limit, and that difference is one of the real trade-offs between the two
techniques.

## 6. What the tests look like

A test is a Kobo program that carries its own expected output in comments. Nothing else.

```kobo
// a test of expressions and their precedence
print 2 + 3 * 4;        // expect: 14
print (2 + 3) * 4;      // expect: 20
print "a" + "b";        // expect: ab
print !nil;             // expect: true
```

```kobo
// tests/phase-1/invalid/unterminated_string.kobo
var name = "Kobo;
print name;
// [line 1] Error: String is never closed.
```

The runner collects the `// expect:` lines in order and requires standard output to match
them exactly, line for line. It collects the `// [line N] ...` lines and requires each to
appear on standard error. Anything extra on standard output is a failure. Run it with:

```
tool/run-golden <phase>
```

You run precisely the code the marking runs, over the tests you were given. **The tests in
your repository are a subset of the marking tests** — `tool/run-golden` says so on its last
line. Nothing in the hidden tests is outside this document: this is the contract, and a
compiler that satisfies it passes tests nobody has shown you.

### 6.1 Seeing the intermediate forms

A scanner prints nothing and a parser prints nothing, so phases 1 and 2 would have no
observable behaviour to mark. Two flags give them one. Both are part of this contract:
the tests match their output character for character.

```
kobo --tokens program.kobo      # stop after scanning, print the token stream
kobo --ast    program.kobo      # stop after parsing, print the tree
kobo          program.kobo      # scan, parse and run on the tree-walking interpreter
kobo --vm     program.kobo      # scan, parse, compile to bytecode and run on the VM
```

`--vm` is what phase 4 is marked with. It must print exactly what the same program prints
without it, down to the error messages, the line numbers and the exit code — that
requirement is the whole point of building the second engine, and one golden-file suite
checks both because of it.

**`--tokens`** prints one token per line, in order, ending with `EOF`:

```
[line N] KIND 'lexeme'
```

`KIND` is the token type's name from the table in section 1.2, exactly as written there —
`LPAREN`, `BANG_EQUAL`, `IDENTIFIER`, `STRING`, `NUMBER`, `PRINT`, `EOF`. The lexeme
is exactly the source text the token was cut from, quotes included for a string, and
empty for `EOF`.

A string may span lines (section 1.5), so a lexeme has to be rendered before it is printed:
wherever the compiler prints a lexeme — here, in an `--ast` dump and in a parse error —
a newline becomes the two characters `\n`, a carriage return `\r`, and a backslash
`\\`. One token is then always one line, and the rendering reads back unambiguously.

**A token that spans lines is reported at the line it ends on**, because that is where
the line counter stands when the scanner finishes reading it. Only a multi-line string
can be affected; every other token starts and ends on the same line. This is deliberately
*not* the rule for the unterminated-string error, which section 1.5 reports at the line the
string opened on — an unterminated string has no closing line to report.

**`EOF` carries the line of the last real token**, not the line the file happens to end
on. A file ending in three blank lines has the same token stream as one ending in none:
a trailing newline left behind by an editor must not move a line number, which is the
same reason section 5 reports an `at end` parse error at the last real token. A file with no
tokens at all — empty, or nothing but comments — reports `EOF` at line 1.

```kobo
print "hi";
```
```
[line 1] PRINT 'print'
[line 1] STRING '"hi"'
[line 1] SEMICOLON ';'
[line 1] EOF ''
```

**`--ast`** prints one statement per line as parenthesised prefix text. An operator node
is `(lexeme operand…)`; a statement node is `(keyword part…)`. Grouping is kept as
`(group …)` so that a redundant bracket is still visible.

```kobo
print 2 + 3 * 4;
var a = 1;
{ a = 2; }
```
```
(print (+ 2 (* 3 4)))
(var a 1)
(block (expr (= a 2)))
```

Both flags obey section 5: a file with a scan error prints its errors and exits 65 under
`--tokens`, and prints nothing on standard output. The same holds for `--ast` and parse
errors. **A phase that failed never prints its result.**

## 7. Which phase builds what

| Phase | CA | Spec sections | Due |
|---|---|---|---|
| 1 — Scanner | CA1 | 1, 5 (scanning errors) | Sun 4 Oct |
| 2 — Parser | CA2 | 3, 5 (parse errors and synchronisation) | Sun 1 Nov |
| 3 — Interpreter | CA3 | 2, 4, 5 (runtime errors) | Sun 29 Nov |
| 4 — Bytecode and VM | **CA3** | all of the above, compiled | Sun 29 Nov |

### The resolver

Between the parser and both engines there is one more pass, and it is the only part of
this compiler that reads the whole tree without producing output. **Resolution** answers,
for every name in the program, *which declaration is this?* — once, before anything runs.

It can answer it because Kobo is lexically scoped (section 4.2): which declaration a name refers
to is a property of the program text, not of the run, so it cannot change between one
execution of a line and the next. The tree-walking interpreter used to re-derive that
answer on every read by searching its scope stack; the resolver settles it once and
writes two things into the tree.

| What it records | Which engine uses it |
|---|---|
| **depth** — how many scopes out from the innermost the declaration lives | the interpreter, to pick the scope directly instead of searching |
| **slot** — where the binding sits in its call frame's run of stack values | the bytecode compiler, for `GET_LOCAL` and `SET_LOCAL` |

**Globals are deliberately not resolved.** A global may be referred to before its
declaration has run — that is what makes

```kobo
fun even(n) { if (n == 0) return true; return odd(n - 1); }
fun odd(n)  { if (n == 0) return false; return even(n - 1); }
print even(10);     // true
```

legal, and mutual recursion with it. A global's binding genuinely is not known until the
line runs, so it stays a lookup, and a name that resolves to no local *is* a global.

**Resolution reports no errors.** It changes no output, no exit code and no message in
section 5.1: a program means exactly what it meant before this pass existed. That is the property
to check your own resolver against — if adding it changes what any program prints, it is
wrong. Kobo is small enough that everything resolution could complain about is either
legal (section 4.2's redeclaration, section 4.3's duplicate parameters, section 3.1's top-level `return`) or
already a runtime error at the point it matters (an undefined name).

Phase 4 changes nothing in this document. The same programs must produce the same output;
only the machinery underneath changes. That is the point of it — and it is why one golden-file
suite can mark both the interpreter and the VM.
