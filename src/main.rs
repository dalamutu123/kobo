// Silences unused-code warnings while the todo!()s are unwritten. It is not in the
// published solution.
#![allow(unused_variables, dead_code, unused_imports)]

// GIVEN: do not edit. Read it anyway; what you write has to fit it.

mod ast;
mod chunk;
mod compiler;
mod disassembler;
mod environment;
mod interpreter;
mod parser;
mod printer;
mod resolver;
mod scanner;
mod token;
mod value;
mod vm;

use std::env;
use std::fs;
use std::process;

const EXIT_USAGE: i32 = 64;
const EXIT_COMPILE_ERROR: i32 = 65;
const EXIT_RUNTIME_ERROR: i32 = 70;

const USAGE: &str = concat!(
    "usage: kobo [--tokens",
    " | --ast",
    " | --disasm | --vm",
    "] <script.kobo>",
);

/// What to do with the source. Each stage of the course adds a mode.
#[derive(Clone, Copy)]
enum Mode {
    Tokens,
    Ast,
    Disasm,
    Vm,
    Run,
}

fn main() {
    let mut mode = Mode::Run;
    let mut path: Option<String> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--tokens" => mode = Mode::Tokens,
            "--ast" => mode = Mode::Ast,
            "--disasm" => mode = Mode::Disasm,
            "--vm" => mode = Mode::Vm,
            _ if arg.starts_with("--") => die(&format!("unknown option {}", arg)),
            _ if path.is_none() => path = Some(arg),
            _ => die("give exactly one script"),
        }
    }

    let Some(path) = path else {
        die("give a script to run")
    };
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => die(&format!("cannot read {}: {}", path, e)),
    };
    process::exit(run(&source, mode));
}

fn run(source: &str, mode: Mode) -> i32 {
    let (tokens, scan_errors) = scanner::scan(source);
    if !scan_errors.is_empty() {
        return report(&scan_errors);
    }

    if let Mode::Tokens = mode {
        for t in &tokens {
            println!(
                "[line {}] {} '{}'",
                t.line,
                t.kind.name(),
                token::one_line(&t.lexeme)
            );
        }
        return 0;
    }

    let (program, parse_errors) = parser::parse(tokens);
    if !parse_errors.is_empty() {
        return report(&parse_errors);
    }

    if let Mode::Ast = mode {
        for stmt in &program.stmts {
            println!("{}", printer::statement(stmt, &program));
        }
        return 0;
    }

    // Resolve every name once, before either engine runs (spec section 7).
    let mut program = program;
    resolver::resolve(&mut program);

    if let Mode::Disasm = mode {
        print!("{}", disassembler::disassemble(&compiler::compile(&program)));
        return 0;
    }

    let outcome = match mode {
        Mode::Vm => vm::run(compiler::compile(&program)),
        _ => interpreter::interpret(&program),
    };
    if let Err(e) = outcome {
        eprintln!("[line {}] Runtime error: {}", e.line, e.message);
        return EXIT_RUNTIME_ERROR;
    }

    0
}

fn report(errors: &[String]) -> i32 {
    for e in errors {
        eprintln!("{}", e);
    }
    EXIT_COMPILE_ERROR
}

fn die(message: &str) -> ! {
    eprintln!("kobo: {}", message);
    eprintln!("{}", USAGE);
    process::exit(EXIT_USAGE);
}
