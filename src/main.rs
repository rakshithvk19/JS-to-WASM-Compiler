mod ast;
mod codegen;
mod lexer;
mod optimizer;
mod parser;

use std::env;
use std::fs;

use codegen::CodeGen;
use lexer::Lexer;
use optimizer::optimize_program;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: compiler <input.js>");
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program();

    optimize_program(&mut program);

    let mut codegen = CodeGen::new();
    let wat = codegen.generate(&program);

    println!("{}", wat);
}
