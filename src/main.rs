mod scanner;
mod token;
mod err;
mod parser;

use crate::parser::{Expr};

use std::io::stdout;
use std::io::Write;

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!(">> ");
    stdout().flush();
    let _bytes_read = std::io::stdin().read_line(&mut input).unwrap();
    let _last_char = input.pop();

    input
}

fn print_ast(expr: &Expr) {
    match expr {
        Expr::Number(float) => println!("{}", float),
        Expr::String(str_value) => println!("{}", str_value),
        Expr::Boolean(value) => println!("{}", value),
        Expr::Binary(b_value) => println!("({} {} {})", b_value.oper.lexeme,
                                          b_value.left, b_value.right),
    }
}

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // for token in tokens {
    //     println!{"{}", token.token_type.to_string()};
    // }

    let mut parser = parser::Parser::new(tokens);
    let ast_node = parser.parse();
    match ast_node {
        Ok(expr) => print_ast(&expr),
        Err(_lox_error) => {
            println!("parser error");
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
