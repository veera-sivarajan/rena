mod err;
mod expr;
mod stmt;
mod interpreter;
mod environment;
mod parser;
mod scanner;
mod token;

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

fn run(source: String, interpreter: &mut interpreter::Interpreter) {
    let mut scanner = scanner::Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => interpreter.interpret(ast),
                Err(parse_error) => println!("{}", parse_error.to_string()),
            }
        }
        Err(scan_error) => println!("{}", scan_error.to_string()),
    }
}

fn main() {
    let mut interpreter = interpreter::Interpreter::new();
    loop {
        let input = get_input();
        if input == "exit" {
            break;
        }
        run(input, &mut interpreter);
    }
}
