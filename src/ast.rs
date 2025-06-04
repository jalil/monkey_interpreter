use crate::token::Token;
use crate::ast::Node;
// Trait for all AST nodes
pub trait Node {
   fn token_literal(&self) -> String;
   fn print_string(&self) -> String;
}

// Identifier type (used in LetStatement and Expression)
#[derive(Debug)]
pub struct Identifier {
   pub  name: String,
   pub  token: Token,
}

// Expression enum
#[derive(Debug)]
enum Expression {
    IdentifierNode(Identifier),
}

// LetStatement type (a kind of statement)
#[derive(Debug)]
pub struct LetStatement {
   pub  token: Token,
   pub  name: Identifier,
   pub  value: Option<Expression>,
}

// StatementNode enum (wraps different statement types)
#[derive(Debug)]
pub enum StatementNode {
  Let(LetStatement),
}

// Program (top-level AST node)
pub struct Program {
    pub statements: Vec<StatementNode>,
}

// ===== Implementations of the Node trait for each type =====
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.name.clone()
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::IdentifierNode(identifier) => identifier.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Expression::IdentifierNode(identifier) => identifier.print_string(),
        }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.print_string());
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.print_string());
        }
        out.push_str(";");
        out
    }
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            StatementNode::Let(let_stmt) => let_stmt.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            StatementNode::Let(let_stmt) => let_stmt.print_string(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Let(let_stmt) => let_stmt.token_literal(),
            }
        } else {
            String::from("")
        }
    }

    fn print_string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.print_string());
        }
        out
    }
}
