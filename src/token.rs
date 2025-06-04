use std::fmt;
use std::{default, fmt::Display};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub enum TokenKind {
    #[default]
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Bang,
    Asterisk,
    Minus,
    Slash,

    GreaterThan,
    LessThan,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Equals,
    NotEqual,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Equals => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
        }
    }
}

pub fn lookup_ident(identifier: &String) -> TokenKind {
    match identifier.as_str() {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "else" => TokenKind::Else,
        "if" => TokenKind::If,
        "return" => TokenKind::Return,
        _ => TokenKind::Ident,
    }
}
