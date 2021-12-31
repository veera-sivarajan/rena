#[macro_use]
mod err;
mod environment;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use crate::interpreter::Interpreter;
use crate::err::LoxError;
use std::{fs, env, io::{stdout, Write}};

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!(">> ");
    let _flush = stdout().flush();
    let _bytes_read = std::io::stdin().read_line(&mut input).unwrap();
    let _last_char = input.pop();

    input
}

fn run(src: String, intp: &mut Interpreter) -> Result<(), LoxError> {
    let tokens = scanner::Scanner::new(src).scan_tokens()?;
    let ast = parser::Parser::new(tokens).parse()?;
    intp.interpret(ast)?;
    Ok(())
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
                    Err(some_error) => eprintln!("{}", some_error.to_string()),
                }
            } else {
                continue;
            }
        }
    } else {
        match run_file(&args[1], &mut interpreter) {
            Ok(()) => {},
            Err(some_error) => eprintln!("{}", some_error.to_string()),
        }
    }
}
