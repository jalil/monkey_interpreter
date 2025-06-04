#[cfg(test)]
use crate::token::{Token, TokenKind, lookup_ident};

#[derive(PartialEq, Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,      // current char position
    read_position: usize, // next char position
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token {
                        kind: TokenKind::Equals,
                        literal: String::from("=="),
                    }
                } else {
                    Lexer::new_token(TokenKind::Assign, self.ch)
                }
            }
            ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            '{' => Lexer::new_token(TokenKind::LeftBrace, self.ch),
            '}' => Lexer::new_token(TokenKind::RightBrace, self.ch),
            ')' => Lexer::new_token(TokenKind::RightParen, self.ch),
            '(' => Lexer::new_token(TokenKind::LeftParen, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            '*' => Lexer::new_token(TokenKind::Asterisk, self.ch),
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    Token {
                        kind: TokenKind::NotEqual,
                        literal: String::from("!="),
                    }
                } else {
                    Lexer::new_token(TokenKind::Bang, self.ch)
                }
            }
            '<' => Lexer::new_token(TokenKind::LessThan, self.ch),
            '>' => Lexer::new_token(TokenKind::GreaterThan, self.ch),
            '-' => Lexer::new_token(TokenKind::Minus, self.ch),
            '/' => Lexer::new_token(TokenKind::Slash, self.ch),
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "\0".to_string(),
            },
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_ident(&literal);
                    return Token { kind, literal };
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    return Token {
                        kind: TokenKind::Int,
                        literal,
                    };
                } else {
                    Lexer::new_token(TokenKind::Illegal, self.ch)
                }
            }
        };

        self.read_char();
        token
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn peek(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_number(&mut self) -> String {
        let mut num = String::new();
        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        num
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token {
            kind,
            literal: ch.to_string(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_monkey_examples() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x,y) {
            x + y;
        };

        let result = add(five, ten);

        !-/*5;
        5 < 10 >5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let expected = vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Function,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::If,
                literal: "if".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::True,
                literal: "true".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Else,
                literal: "else".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Return,
                literal: "return".to_string(),
            },
            Token {
                kind: TokenKind::False,
                literal: "false".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Equals,
                literal: "==".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::NotEqual,
                literal: "!=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "9".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Eof,
                literal: "\0".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (idx, exp_token) in expected.into_iter().enumerate() {
            let recv_token = lexer.next_token();
            println!("{:?},{:?}", idx, exp_token);
            assert_eq!(
                exp_token.kind, recv_token.kind,
                "test[{}] - token type wrong. expected={:?}, got={:?}",
                idx, exp_token.kind, recv_token.kind
            );
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "test[{}] - literal wrong. expected={}, got={}",
                idx, exp_token.literal, recv_token.literal
            );
        }
    }
}
