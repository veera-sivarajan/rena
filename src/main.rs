mod scanner;
mod token;
mod err;
mod parser;
mod ast;
mod interpreter;
mod expr;
mod stmt;
mod environment;

use std::io::stdout;
use std::io::Write;
use crate::err::LoxError;

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
    let mut scanner = scanner::Scanner::new(src);
    let tokens = scanner.scan_tokens()?;
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
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
