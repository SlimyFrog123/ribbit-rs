#[derive(PartialEq, Clone)]
pub enum TokenType {
    // Base tokens.
    Identifier,
    // Unknown character, used for throwing syntax errors. All expected characters have their own
    // respective token types.
    Character,

    // Single character tokens.
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Period,
    Comma,
    Semicolon,
    Colon,

    // Single or double character tokens.
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    ExclamationMark,
    NotEqual,
    Assign, // Single equal, only used for assigning.
    Equal, // Double equal, used for equality check.
    And,
    Or,

    // Operation tokens.
    Increment, // ++
    Decrement, // --
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Remainder, // %
    Power, // ^

    // Positional operation tokens; +=, -=, etc.
    PositionalAdd,
    PositionalSubtract,
    PositionalMultiply,
    PositionalDivide,
    PositionalRemainder,
    PositionalPower,

    // Literals.
    String,
    Char, // Single character, can be an escape sequence like a newline, etc.
    Boolean,
    Integer, // Whole number.
    Float, // Floating point number.

    // Keywords.
    // As this is a statically-typed programming language, functions will specify their return type
    // rather than using a function keyword, so the function keyword is not necessary.
    // Class/object-related.
    Class,
    Super, // Calls to the parent class are routed through the `super` keyword.
    This,
    Extends,
    // Loops.
    For,
    While,
    Loop,
    Break,
    Continue,
    // Logical.
    If,
    Else,

    // Other.
    Return,
    Is,

    // End of file.
    EOF
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        return match self {
            TokenType::Identifier => String::from("Identifier"),
            TokenType::Character => String::from("Character"),
            TokenType::LeftParenthesis => String::from("LeftParenthesis"),
            TokenType::RightParenthesis => String::from("RightParenthesis"),
            TokenType::LeftBracket => String::from("LeftBracket"),
            TokenType::RightBracket => String::from("RightBracket"),
            TokenType::LeftBrace => String::from("LeftBrace"),
            TokenType::RightBrace => String::from("RightBrace"),
            TokenType::Period => String::from("Period"),
            TokenType::Comma => String::from("Comma"),
            TokenType::Semicolon => String::from("Semicolon"),
            TokenType::Colon => String::from("Colon"),
            TokenType::LessThan => String::from("LessThan"),
            TokenType::LessThanOrEqual => String::from("LessThanOrEqual"),
            TokenType::GreaterThan => String::from("GreaterThan"),
            TokenType::GreaterThanOrEqual => String::from("GreaterThanOrEqual"),
            TokenType::ExclamationMark => String::from("ExclamationMark"),
            TokenType::NotEqual => String::from("NotEqual"),
            TokenType::Assign => String::from("Assign"),
            TokenType::Equal => String::from("Equal"),
            TokenType::And => String::from("And"),
            TokenType::Or => String::from("Or"),
            TokenType::Increment => String::from("Increment"),
            TokenType::Decrement => String::from("Decrement"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Star => String::from("Star"),
            TokenType::ForwardSlash => String::from("ForwardSlash"),
            TokenType::Remainder => String::from("Remainder"),
            TokenType::Power => String::from("Power"),
            TokenType::PositionalAdd => String::from("PositionalAdd"),
            TokenType::PositionalSubtract => String::from("PositionalSubtract"),
            TokenType::PositionalMultiply => String::from("PositionalMultiply"),
            TokenType::PositionalDivide => String::from("PositionalDivide"),
            TokenType::PositionalRemainder => String::from("PositionalRemainder"),
            TokenType::PositionalPower => String::from("PositionalPower"),
            TokenType::String => String::from("String"),
            TokenType::Char => String::from("Char"),
            TokenType::Boolean => String::from("Boolean"),
            TokenType::Integer => String::from("Integer"),
            TokenType::Float => String::from("Float"),
            TokenType::Class => String::from("Class"),
            TokenType::Super => String::from("Super"),
            TokenType::This => String::from("This"),
            TokenType::Extends => String::from("Extends"),
            TokenType::For => String::from("For"),
            TokenType::While => String::from("While"),
            TokenType::Loop => String::from("Loop"),
            TokenType::Break => String::from("Break"),
            TokenType::Continue => String::from("Continue"),
            TokenType::If => String::from("If"),
            TokenType::Else => String::from("Else"),
            TokenType::Return => String::from("Return"),
            TokenType::Is => String::from("Is"),
            TokenType::EOF => String::from("EOF")
        };
    }
}
