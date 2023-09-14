use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Keyword {
    pattern: String,
    pub(crate) token_type: TokenType
}

impl Keyword {
    pub fn new(pattern: String, token_type: TokenType) -> Self {
        return Self {
            pattern,
            token_type
        };
    }

    pub fn matches(&self, pattern: String) -> bool {
        return pattern.eq(&self.pattern);
    }
}
