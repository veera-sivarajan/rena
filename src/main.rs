#[macro_use]
mod err;
mod environment;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;

use crate::err::LoxError;
use std::io::stdout;
use std::io::Write;

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!(">> ");
    let _flush = stdout().flush();
    let _bytes_read = std::io::stdin().read_line(&mut input).unwrap();
    let _last_char = input.pop();

    input
}

fn run(src: String, intp: &mut interpreter::Interpreter) -> Result<(), LoxError> {
    let tokens = scanner::Scanner::new(src).scan_tokens()?;
    let ast = parser::Parser::new(tokens).parse()?;
    intp.interpret(ast)?;
    Ok(())
}

fn main() {
    let mut interpreter = interpreter::Interpreter::new();
    loop {
        let input = get_input();
        if input == "exit" {
            std::process::exit(0);
        } else {
            match run(input, &mut interpreter) {
                Ok(()) => continue,
                Err(some_error) => eprintln!("{}", some_error.to_string()),
            }
        }
    }
}
