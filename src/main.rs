mod scanner;
mod token;

use crate::scanner::Scanner;

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

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!{"{}", token.token_type.to_string()};
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
