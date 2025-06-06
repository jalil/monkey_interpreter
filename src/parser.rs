use crate::lexer::Lexer;
use crate::ast::Program;
use crate::token::{Token, TokenKind};
use crate::ast::{StatementNode, LetStatement};
use crate::ast::{Identifier};

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>
}


impl Parser {
   pub  fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
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
        let mut program =  Program {
            statements: vec![]
        };

        while self.current_token.kind != TokenKind::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            _ => None,
        }

    }

    fn parse_let_statement(&mut self) -> Option<StatementNode> {
       let mut stmt = LetStatement {
           token: self.current_token.clone(),
           name: Default::default(),
           value: Default::default(),
        };

       return if !self.expect_peek(TokenKind::Ident) {
           None
       } else {
           stmt.name = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
           };

           if !self.expect_peek(TokenKind::Assign) {
               None
           } else {
               self.next_token();
              while  !self.current_token_is(TokenKind::Semicolon) {
                self.next_token();
               }
              Some(StatementNode::Let(stmt))

           }
       };

    }
       fn expect_peek(&mut self, token_kind: TokenKind) ->  bool {
           if self.peek_token_is(token_kind.clone()) {
               self.next_token();
               return true;
           }
           self.peek_error(token_kind);
           false
       }

       fn peek_token_is(&self, token_kind: TokenKind) -> bool {
           self.peek_token.kind == token_kind
       }


       fn current_token_is(&self, token_kind: TokenKind)  -> bool {
           self.current_token.kind == token_kind
       }

       fn errors(&self) -> &Vec<String> {
          &self.errors
       }

       fn peek_error(&mut self, token_kind: TokenKind) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            token_kind, self.peek_token.kind
        );
        self.errors.push(msg);
       }
}


#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use super::Parser;
    use crate::ast::StatementNode;
    use crate::ast::Node;
    
    #[test]
    fn test_let_statements() {
        let input  = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parse_errors(parser);

        match program {
            Some(program) => {
                assert_eq!(
                program.statements.len(),
                3,
                "statements does not contain 3 statements. got={}",
                program.statements.len()
            );

            let expected = vec!["x","y","foobar"];

            for (idx, exp) in expected.into_iter().enumerate() {
                let stmt =  &program.statements[idx];
                test_let_statement(stmt,exp);
                } 
            }
            
            None => panic!("parse program should not be none"),
        }
    }


    fn test_let_statement(stmt: &StatementNode, expected: &str ) {
        assert_eq!(
            stmt.token_literal(),
            "let", 
            "token literal is not `let`. got={}",
            stmt.token_literal()
        );
        match stmt {
            StatementNode::Let(let_stmt) => { 
                assert_eq!(
                let_stmt.name.value,
                expected,
                "LetStatement name value not {}. got {}", expected, let_stmt.name.value
            );
                assert_eq!(
                    let_stmt.name.token_literal(),
                    expected,
                    "LetStatement name value not {}. got {}",
                    expected,
                    let_stmt.name.token_literal()
                );
            }
        }
    }

    fn check_parse_errors(parser: Parser) {
        let errors  = parser.errors();

        if errors.len() == 0 {
            return ;
        }

        for error in errors {
            eprintln!("parser error: {}", error);
        }

        panic!("parse errors present");
    }
}
