#[cfg(test)]
use crate::lexer::Lexer;
use crate::ast::Program;
use crate::token::Token;
use crate::ast::StatementNode;
use crate::ast::Node;

pub struct Parser {
    pub lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}


impl Parser {
   pub  fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> Option<Program> {

    }
}


#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use super::Parser;
    use crate::ast::StatementNode;

    fn test_let_statements() {
        let input  = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let program = parser.parse_program();

        match program {
            Some(program) => {
                assert_eq!(program.statements.len(),3,"statements does not contain 3 statements. got={}", program.statements.len());
            let expected = vec!["x","y","foobar"];

            for (idx, exp) in expected.into_iter().enumerate() {
                let stmt =  &program.statements[idx];
                test_let_statement(stmt,expected);

            } 
            }
            
            None => panic!("parse program should not be none"),
        }
    }


    fn test_let_statement(stmt: &StatementNode, expected: &str ) {
        assert_eq!(stmt.token_literal(), "let", "token is not `let`. got={}", stmt.token_literal() );
        match stmt {
            StatementNode::Let(let_stmt) => { 
                assert_eq!(let_stmt.name.value, expected, "LetStatement name value not {}. got {}", expected, let_stmt.name.value );
                assert_eq!(let_stmt.name.token_literal(), expected, "LetStatement name value not {}. got {}", expected, let_stmt.name.token_literal() );
            }
        }
    }
}
