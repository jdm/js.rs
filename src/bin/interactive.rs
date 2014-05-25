#![feature(globs)]
#![crate_type = "bin"]
extern crate script;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use std::io;

fn main() {
	let mut engine : Interpreter = Executor::new();
        loop {
                print!("> ");
                match io::stdin().read_line() {
			Ok(line) => {
                                io::stdout().flush().unwrap();
				let tokens = Lexer::<io::BufferedReader<io::BufReader>>::lex_str(line.as_slice());
			        let expr = match Parser::new(tokens).parse_all() {
                                    Ok(expr) => expr,
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        continue;
                                    }
                                };
				let result = engine.run(&expr);
				match result {
					Ok(v) => println!("{}", v.borrow()),
					Err(v) => println!("Error: {}", v.borrow())
				}
			},
			Err(err) => {
				fail!("{}", err);
			}
		}
	}
}