use crate::keyword::Keyword;
use crate::position::Position;
use crate::program::Program;
use crate::token::Token;
use crate::token_type::TokenType;

// The lexer, or tokenizer. This takes the input text and transforms it into tokens which are then
// parsed by the parser.
pub struct Lexer<'a> {
    program: &'a mut Program,
    source: String, // The actual source to tokenize, or lex.
    source_length: i32, // The length of the source.
    position: i32, // Position in the source.
    running: bool, // Whether or not the lexer is running.
    keywords: Vec<Keyword>, // The keywords that the lexer will check.
    line: i32,
    column: i32
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a mut Program, source: String, keywords: Vec<Keyword>) -> Self {
        return Self {
            program,
            source: source.to_owned(),
            source_length: (source.len().to_owned()) as i32,
            position: -1,
            running: false,
            keywords,
            line: 0,
            column: -1
        };
    }

    fn parse_escape_sequence(input: char) -> (char, bool) {
        return match input {
            'n' => ('\n', true),
            'r' => ('\r', true),
            't' => ('\r', true),
            '\'' => ('\'', true),
            '"' => ('\"', true),
            '\\' => ('\\', true),
            '0' => ('\0', true),
            _ => ('\0', false)
        };
    }

    fn peek(&self) -> char {
        if self.position < self.source_length {
            return self.source.chars().nth(self.position as usize).unwrap();
        }

        return '\0';
    }

    fn peek_ahead(&self, amount: i32) -> char {
        if self.position + amount < self.source_length {
            return self.source.chars().nth((self.position + amount) as usize).unwrap();
        }

        return '\0';
    }

    fn advance(&mut self) -> () {
        self.position += 1;

        if self.position < self.source_length {
            self.running = true;

            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        } else {
            // The lexer has reached the end of the input string.
            self.running = false;
        }
    }

    fn consume(&mut self, amount: i32) -> () {
        self.position += amount;

        for _ in 0..amount {
            self.advance()
        }
    }

    fn die_with_error(&mut self, message: String, position: Position) {
        self.program.die_with_error(message, position);
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // The match strings.
        let alphabet: String = "abcdefghijklmnopqrstuvwxyz".to_owned();
        let alphabet_upper: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned();
        let numbers: String = "0123456789".to_owned();

        let identifier_start: String = alphabet.to_owned() + &alphabet_upper + "_";
        let identifier_full: String = identifier_start.to_owned() + &numbers;

        let number_start: String = numbers.to_owned();
        let number_full: String = number_start.to_owned() + "._";

        let skip: String = " \n\t\r".to_owned();

        // Go to the first character.
        self.advance();

        while self.running {
            if identifier_start.to_owned().contains(self.peek()) {
                let mut identifier_string: String = String::new();
                let line: i32 = self.line.to_owned();
                let start_column: i32 = self.column.to_owned();

                while identifier_full.to_owned().contains(self.peek()) {
                    identifier_string.push(self.peek());
                    self.advance();
                }

                let end_column: i32 = self.column.to_owned();
                let position: Position = Position::new(line, start_column, end_column);

                if identifier_string == "true" || identifier_string == "false" {
                    let token: Token = Token::new(TokenType::Boolean,
                                                  Some(identifier_string),
                                                  position.to_owned());
                    tokens.push(token);

                    continue;
                } else {
                    // Check if the found identifier matches any of the keywords.
                    for keyword in self.keywords.clone().into_iter() {
                        if keyword.matches(identifier_string.to_owned()) {
                            let token: Token = Token::new_empty(
                                keyword.token_type,
                                position.to_owned()
                            );
                            tokens.push(token);

                            continue;
                        }
                    }
                }

                // Create the identifier token and add it to the list of tokens.
                let token: Token = Token::new(
                    TokenType::Identifier,
                    Some(identifier_string),
                    position
                );

                tokens.push(token);

                continue;
            } else if number_start.to_owned().contains(self.peek()) {
                let mut number_string: String = String::new();
                let mut dot_count: i32 = 0;
                let line: i32 = self.line.to_owned();
                let start_column: i32 = self.column.to_owned();

                while number_full.to_owned().contains(self.peek()) {
                    if self.peek() == '.' {
                        dot_count += 1;

                        if dot_count > 1 {
                            self.die_with_error(
                                "Floating point numbers can only have a single decimal \
                                point.".to_owned(),
                                Position::new_single(
                                    self.line.to_owned(),
                                    self.column.to_owned()
                                )
                            );
                        } else {
                            number_string.push(self.peek());
                        }
                    } else if self.peek() != '_' {
                        number_string.push(self.peek());
                    }

                    self.advance();
                }

                let end_column: i32 = self.column.to_owned();

                let token_type: TokenType = if dot_count == 1 {
                    TokenType::Float
                } else {
                    TokenType::Integer
                };

                // Create the number token with the specified token type and add it to the list of
                // tokens.
                let token: Token = Token::new(
                    token_type,
                    Some(number_string),
                    Position::new(
                        line,
                        start_column,
                        end_column
                    )
                );
                tokens.push(token);

                continue;
            } else if self.peek() == '"' {
                // String literal.
                let mut string_value: String = String::new();
                let line: i32 = self.line.to_owned();
                let start_column: i32 = self.column.to_owned();
                let mut closed: bool = false;

                self.advance(); // Move past the opening double quote.

                while self.running {
                    if self.peek() == '"' {
                        closed = true;
                        self.advance(); // Move past the closing double quote.

                        break;
                    } else {
                        if self.peek() == '\\' {
                            let escape_sequence_start: i32 = self.column.to_owned();
                            let mut escape_sequence: char = '\0';
                            let mut success: bool = false;

                            self.advance(); // Move past the backslash.

                            (escape_sequence, success) =
                                Self::parse_escape_sequence(self.peek());

                            if success {
                                string_value.push(escape_sequence);
                            } else {
                                self.die_with_error(
                                    "Invalid escape sequence: `\\".to_owned() + &self
                                        .peek().to_string() + "`.",
                                    Position::new(
                                        self.line.to_owned(),
                                        escape_sequence_start,
                                        self.column.to_owned()
                                    )
                                );
                            }
                        } else {
                            // Append the current character to the value string.
                            string_value.push(self.peek());
                        }
                    }

                    self.advance();
                }

                if closed {
                    let token: Token = Token::new(
                        TokenType::String,
                        Some(string_value),
                        Position::new(
                            line,
                            start_column,
                            self.column.to_owned()
                        )
                    );
                    tokens.push(token);
                } else {
                    self.die_with_error(
                        "Unclosed string literal.".to_owned(),
                        Position::new_single(
                            line,
                            start_column
                        )
                    );
                }
            } else if self.peek() == '\'' {
                // Char literal.
                let mut char_value: char = '\0';
                let line: i32 = self.line.to_owned();
                let start_column: i32 = self.column.to_owned();

                self.advance(); // Move past the opening single quote.-

                if self.peek() == '\\' {
                    let escape_sequence_start: i32 = self.column.to_owned();
                    let mut escape_sequence: char = '\0';
                    let mut success: bool = false;

                    self.advance(); // Move past the backslash.

                    (escape_sequence, success) =
                        Self::parse_escape_sequence(self.peek());

                    if success {
                        char_value = escape_sequence;
                    } else {
                        self.die_with_error(
                            "Invalid escape sequence: `\\".to_owned() + &self
                                .peek().to_string() + "`.",
                            Position::new(
                                self.line.to_owned(),
                                escape_sequence_start,
                                self.column.to_owned()
                            )
                        );
                    }
                } else {
                    char_value = self.peek();
                }

                self.advance(); // (Hopefully) move to the closing single quote.

                if self.peek() == '\'' {
                    self.advance(); // Move past the closing single quote.

                    let token: Token = Token::new(
                        TokenType::Char,
                        Some(char_value.to_string()),
                        Position::new(
                            line,
                            start_column,
                            self.column.to_owned()
                        )
                    );
                    tokens.push(token);

                    continue;
                } else {
                    self.die_with_error(
                        "Unexpected character `".to_owned() + &self.peek().to_string() +
                            "`. Expected (closing) single quote.",
                        Position::new_single(
                            line,
                            self.column.to_owned()
                        )
                    );
                }
            } else if !skip.contains(self.peek()) {
                let single_position: Position = Position::new_single(
                    self.line.to_owned(),
                    self.column.to_owned()
                );

                let double_position: Position = Position::new(
                    self.line.to_owned(),
                    self.column.to_owned(),
                    self.column.to_owned() + 1,
                );

                if self.peek() == '+' {
                    if self.peek_ahead(1) == '+' {
                        // Increment.
                        let token: Token = Token::new_empty(TokenType::Increment,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else if self.peek_ahead(1) == '=' {
                        // Positional add.
                        let token: Token = Token::new_empty(TokenType::PositionalAdd,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Plus.
                        let token: Token = Token::new_empty(TokenType::Plus,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '-' {
                    if self.peek_ahead(1) == '-' {
                        // Decrement.
                        let token: Token = Token::new_empty(TokenType::Decrement,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else if self.peek_ahead(1) == '=' {
                        // Positional subtract.
                        let token: Token =
                            Token::new_empty(TokenType::PositionalSubtract,
                                             double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Minus.
                        let token: Token = Token::new_empty(TokenType::Minus,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '*' {
                    if self.peek_ahead(1) == '=' {
                        // Positional multiply.
                        let token: Token =
                            Token::new_empty(TokenType::PositionalMultiply,
                                             double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Star.
                        let token: Token = Token::new_empty(TokenType::Star,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '/' {
                    if self.peek_ahead(1) == '=' {
                        // Positional divide.
                        let token: Token = Token::new_empty(TokenType::PositionalDivide,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // (Forward) slash.
                        let token: Token = Token::new_empty(TokenType::ForwardSlash,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '%' {
                    if self.peek_ahead(1) == '=' {
                        // Positional remainder.
                        let token: Token =
                            Token::new_empty(TokenType::PositionalRemainder,
                                             double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Remainder.
                        let token: Token = Token::new_empty(TokenType::Remainder,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '^' {
                    if self.peek_ahead(1) == '=' {
                        // Positional power.
                        let token: Token = Token::new_empty(TokenType::PositionalPower,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Power.
                        let token: Token = Token::new_empty(TokenType::Power,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '.' {
                    let token: Token = Token::new_empty(TokenType::Period,
                                                        single_position.to_owned());
                    tokens.push(token);
                } else if self.peek() == ',' {
                    let token: Token = Token::new_empty(TokenType::Comma,
                                                        single_position.to_owned());
                    tokens.push(token);
                } else if self.peek() == ';' {
                    let token: Token = Token::new_empty(TokenType::Semicolon,
                                                        single_position.to_owned());
                    tokens.push(token);
                } else if self.peek() == ':' {
                    let token: Token = Token::new_empty(TokenType::Colon,
                                                        single_position.to_owned());
                    tokens.push(token);
                } else if self.peek() == '=' {
                    if self.peek_ahead(1) == '=' {
                        // Equality check.
                        let token: Token = Token::new_empty(TokenType::Equal,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Assign (single equal).
                        let token: Token = Token::new_empty(TokenType::Assign,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '!' {
                    if self.peek_ahead(1) == '=' {
                        // Not equal.
                        let token: Token = Token::new_empty(TokenType::NotEqual,
                                                            double_position.to_owned());
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Exclamation mark.
                        let token: Token = Token::new_empty(TokenType::ExclamationMark,
                                                            single_position.to_owned());
                        tokens.push(token);
                    }
                } else if self.peek() == '<' {
                    if self.peek_ahead(1) == '=' {
                        // Less than or equal to.
                        let token: Token = Token::new_empty(
                            TokenType::LessThanOrEqual,
                            double_position.to_owned()
                        );
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Less than.
                        let token: Token = Token::new_empty(
                            TokenType::LessThan,
                            single_position.to_owned()
                        );
                        tokens.push(token);
                    }
                } else if self.peek() == '>' {
                    if self.peek_ahead(1) == '=' {
                        // Greater than or equal to.
                        let token: Token = Token::new_empty(
                            TokenType::GreaterThanOrEqual,
                            double_position.to_owned()
                        );
                        tokens.push(token);

                        self.consume(2);
                        continue;
                    } else {
                        // Greater than.
                        let token: Token = Token::new_empty(
                            TokenType::GreaterThan,
                            single_position.to_owned()
                        );
                        tokens.push(token);
                    }
                } else if self.peek() == '&' && self.peek_ahead(1) == '&' {
                    let token: Token = Token::new_empty(TokenType::And,
                                                        single_position.to_owned());
                    tokens.push(token);

                    self.consume(2);
                    continue;
                } else if self.peek() == '|' && self.peek_ahead(1) == '|' {
                    let token: Token = Token::new_empty(TokenType::Or,
                                                        single_position.to_owned());
                    tokens.push(token);

                    self.consume(2);
                    continue;
                } else {
                    // Unknown character.
                    let token: Token = Token::new(TokenType::Character,
                                                  Some(self.peek().to_string()),
                                                  single_position.to_owned());
                    tokens.push(token);
                }
            }

            self.advance();
        }

        // Finally, append the empty end of file token to the list of tokens.
        tokens.push(Token::new_empty(
            TokenType::EOF,
            Position::new_single(
                self.line.to_owned(),
                self.column.to_owned()
            )
        ));

        return tokens;
    }
}
