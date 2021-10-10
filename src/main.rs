mod scanner;
mod token;
mod err;
mod parser;
mod ast;
mod interpreter;
mod expr;

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

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    match tokens {
        Ok(ref _vector) => {},
        Err(ref e) => {
            println!("{}", e.to_string());
            return;
        }
    }

    let mut parser = parser::Parser::new(tokens.unwrap());
    let ast_node = parser.parse();
    match ast_node {
        Ok(expr) => interpreter::interpret(expr),
        Err(lox_error) => {
            println!("{}", lox_error.to_string());
        },
    }
}

fn main() {
    loop {
        let input = get_input(); 
        if input == "exit" {
            break;
        }
        run(input);
    }
}
