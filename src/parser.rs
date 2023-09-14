use crate::program::Program;
use crate::token::Token;

pub struct Parser<'a> {
    program: &'a mut Program,
    input: Vec<Token>,
    position: i32,
    current_token: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a mut Program, input: Vec<Token>) -> Self {
        return Self {
            program,
            input,
            position: -1,
            current_token: None
        };
    }
}
