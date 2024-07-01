// INFO: First start of an interpreter, the main part of the whole project
// It just basically scans (or visits) all the AST nodes and then return the result

use ast::AST;
use error_handling::Error;
use lexer::Token;

pub struct Interpreter {
    ast: AST,
}

impl Interpreter {
    pub fn new(ast: AST) -> Interpreter {
        Interpreter { ast }
    }

    pub fn interpret(&self) -> String {
        return self.visit(self.ast);
    }

    fn visit(&self, ast: AST) -> String {
        match &ast {
            AST::FormingCalc {
                node1,
                operator,
                node2,
            } => self.visit_forming_calc(node1.clone(), operator.unwrap().clone(), node2.clone()),
            AST::Factor { identifier, token } => {
                self.visit_factor(identifier.to_string(), token.clone())
            }
            AST::UnaryFactor { sign, value } => self.visit_unary(sign.to_string(), value.clone()),
            AST::Nil => String::new(),
        }
    }

    fn visit_factor(&self, identifier: String, token: Token) -> String {}

    fn visit_unary(&self, sign: String, value: Box<AST>) -> String {}

    fn visit_forming_calc(&self, node1: Box<AST>, operator: Token, node2: Box<AST>) -> String {}
}
