use std::fmt;
use std::fmt::Display;

#[derive(PartialEq,Debug)]
 pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
 }

#[derive(PartialEq,Debug)]
pub enum TokenKind {
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

  Function,
  Let,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus=> write!(f, "+"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Function=> write!(f, "Function"),
            TokenKind::Let=> write!(f, "Let"),
        }
    }
 }
