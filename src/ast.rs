trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

struct Program {
    statements: Vec<StatementNode>
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

        let mut out = String::from("");
        for stmt in self.statements.iter() {
           out.push_str(&stmt.print_string()) 
        }
      out
    }

}
enum StatementNode {
    Let(LetStatement),
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

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Option<Expression>
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
            let mut  out = String::new();
            out.push_str(&self.token_literal())
            out.push_str(" ");
            out.push_str(&self.name.print_string());
            out.push_str(" = ");
            if let Some(value) =&self.value {
                out.push_str(&value.print_string());
            }
            out.push_str(";");

            out
        }
}

enum Expression {
    IdentifierNode(Identifier),
}

impl Node for Expression{
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

struct Identifier {
    name: String,
    token: Token,
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.name.clone()
        }
}
