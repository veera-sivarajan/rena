#[macro_use]
mod err;
mod environment;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod functions;

use crate::err::LoxError;
use crate::interpreter::Interpreter;
use std::{
    env, fs,
    io::{stdout, Write},
};

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!("rena$ ");
    let _ = stdout().flush();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let _ = input.pop();

    input
}

fn run(src: String, intp: &mut Interpreter) -> Result<(), LoxError> {
    let tokens = scanner::Scanner::new(src).scan_tokens()?;
    let ast = parser::Parser::new(tokens).parse()?;
    intp.interpret(&ast)
}

fn run_file(path: &str, intp: &mut Interpreter) -> Result<(), LoxError> {
    let file_string = fs::read_to_string(path)
        .expect("Source file cannot be read.");
    run(file_string, intp)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() == 1 {
        loop {
            let input = get_input();
            if input == "exit" {
                std::process::exit(0);
            } else if !input.is_empty() {
                match run(input, &mut interpreter) {
                    Ok(()) => continue,
                    Err(some_error) => eprintln!("{}", some_error),
                }
            } else {
                continue;
            }
        }
    } else {
        match run_file(&args[1], &mut interpreter) {
            Ok(()) => {}
            Err(some_error) => eprintln!("{}", some_error),
        }
    }
}
