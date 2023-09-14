mod program;
mod token_type;
mod position;
mod token;
mod keyword;
mod lexer;
mod binary_operator;
mod literals;
mod expressions;
mod parser;
mod statements;
mod nodes;

use std::fs;
use std::env;
use std::process;
use crate::keyword::Keyword;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program::Program;
use crate::token::Token;
use crate::token_type::TokenType;

fn run(source: String) -> () {
    let keywords: Vec<Keyword> = vec![
        Keyword::new("class".to_owned(), TokenType::Class),
        Keyword::new("super".to_owned(), TokenType::Super),
        Keyword::new("this".to_owned(), TokenType::This),
        Keyword::new("for".to_owned(), TokenType::For),
        Keyword::new("while".to_owned(), TokenType::While),
        Keyword::new("loop".to_owned(), TokenType::Loop),
        Keyword::new("break".to_owned(), TokenType::Break),
        Keyword::new("continue".to_owned(), TokenType::Continue),
        Keyword::new("if".to_owned(), TokenType::If),
        Keyword::new("else".to_owned(), TokenType::Else),
        Keyword::new("return".to_owned(), TokenType::Return),
        Keyword::new("is".to_owned(), TokenType::Is),
        Keyword::new("extends".to_owned(), TokenType::Extends)
    ];

    let mut program: Program = Program::new(keywords, source);

    let mut lexer: Lexer = program.create_lexer();
    let tokens: Vec<Token> = lexer.lex();

    program.set_tokens(tokens);

    let mut parser: Parser = program.create_parser();
}

fn run_file(filepath: String) -> () {
    // Read the contents of the file.
    let file_contents: String = fs::read_to_string(filepath).unwrap();

    // Run the source code.
    run(file_contents);
}

fn main() -> () {
    // Gather the args.
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        // Get the filepath.
        let filepath: String = (&args[1]).to_owned();
        
        // Then, attempt to run the file.
        run_file(filepath);
    } else {
        // Print the correct usage of the program and then exit.
        eprintln!("Usage: ribbit-rs <input>");

        process::exit(1);
    }
}