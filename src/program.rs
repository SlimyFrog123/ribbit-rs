use std::process;
use crate::keyword::Keyword;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position::Position;
use crate::token::Token;

pub struct Program {
    keywords: Vec<Keyword>,
    tokens: Vec<Token>,
    source: String
}

impl Program {
    pub fn new(keywords: Vec<Keyword>, source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();

        return Self {
            keywords,
            tokens,
            source
        };
    }

    pub fn die_with_error(&self, message: String, position: Position) {
        let source: String = self.source.to_owned();
        let line: i32 = position.to_owned().line;
        let lines: Vec<String> = source.to_owned().lines().map(|s| s.to_owned()).collect();
        let mut underline_amount: i32 = position.to_owned().end - position.to_owned().start;

        if underline_amount == 0 {
            underline_amount = 1;
        } else if underline_amount == 1 {
            underline_amount = 2;
        }

        dbg!(position.to_string());

        // Create the error message.
        let mut error_message: String = String::new();
        error_message += &("Error on line ".to_owned() + &line.to_owned().to_string());
        error_message += &(": ".to_owned() + message.as_str());
        error_message += &("\n ".to_owned() + &line.to_owned().to_string() + " | " + lines.get(
            line as usize).unwrap());
        error_message += "\n ";
        error_message += &(" ".repeat(line.to_owned().to_string().chars().count()) + "   ");
        error_message += &" ".repeat(position.to_owned().start as usize - 1);
        error_message += &("^".repeat(underline_amount as usize) + " <- Here");

        // Print the message.
        eprintln!("{}", error_message);

        // Exit with the error code 1.
        process::exit(1);
    }

    pub fn create_lexer(&mut self) -> Lexer {
        return Lexer::new(
            self,
            self.source.to_owned(),
            self.keywords.clone()
        );
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) -> () {
        self.tokens = tokens;
    }

    pub fn create_parser(&mut self) -> Parser {
        return Parser::new(
            self,
            self.tokens.clone()
        );
    }
}