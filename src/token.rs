use crate::position::Position;
use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    token_value: Option<String>,
    token_position: Position
}

impl Token {
    pub fn new(token_type: TokenType, token_value: Option<String>, token_position: Position) ->
    Self {
        return Self {
            token_type,
            token_value,
            token_position
        };
    }

    // This constructor is to create a new token that is initialized without a value.
    pub fn new_empty(token_type: TokenType, token_position: Position) -> Self {
        return Self {
            token_type,
            token_value: None,
            token_position
        };
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        if self.token_value.is_none() {
            return self.token_type.to_string() + ", " + &self.token_position.to_string()
        }

        return self.token_type.to_string() + ": " + &self.token_value.clone().unwrap() + ", " +
            &self.token_position.to_string();
    }
}
