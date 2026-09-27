# Kobo

Your compiler for Kobo, and the only repository you use all semester. `docs/language-spec.md` is the contract: where this code and that document disagree, the document is right.

Every function you will write between now and the end of November is already here, each one a `todo!()` carrying the rule it implements and the section of the specification that defines it. Run the compiler and it stops at the first one nobody has written, and names it. Nothing else is downloaded: CA1, CA2 and CA3 are three marking windows on this one repository, not three code releases.

**Start with CA1: the five functions in `src/scanner.rs`.** Do not read the other thirty-four today; each has its own week.

```
cargo build
tool/run-golden phase-1   # CA1, the scanner
tool/run-golden phase-2   # CA2, the parser
tool/run-golden phase-3   # CA3, the interpreter
tool/run-golden phase-4   # CA3, the VM
```

Each suite starts at 0 and stays there until its own week: the tests are the specification, not a schedule.

Every CA is marked by running these suites. The tests in this repository are a subset: the rest are hidden and run only when work is marked, and each brief says how many and what they are worth. Every test carries the output it expects in a comment, so a failure tells you what was wanted and what it got.

**Read `docs/ca1.md` before you start.** It is the brief: what the marks are for, what else has to be in your repository, and how to submit.

The documents in `docs/` are the same ones on the portal, kept here so they work offline. In VS Code, open one and press Shift+Cmd+V (Shift+Ctrl+V on Windows) to read it formatted; the portal has them as PDFs if you prefer.

## What you write

**`src/scanner.rs`**, run, scan_token, string, number, identifier

**`src/parser.rs`**, declaration, fun_declaration, var_declaration, statement_inner, return_statement, if_statement, while_statement, block, assignment, or, and, equality, comparison, term, factor, unary, call, primary, synchronise

**`src/resolver.rs`**, resolve, declare, define, lookup, end_scope

**`src/interpreter.rs`**, execute, evaluate, call

**`src/compiler.rs`**, statement, expression, declare, end_scope, patch

**`src/vm.rs`**, Vm::run, Vm::call

Everything else is given complete and must not be changed: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/token.rs`, `src/value.rs`, `src/ast.rs`, `src/printer.rs`, `src/environment.rs`, `src/chunk.rs`, `src/disassembler.rs`. Those files are the compiler you are handed, not the compiler you write.

