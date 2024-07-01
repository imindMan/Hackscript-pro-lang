// INFO: First start of an interpreter, the main part of the whole project
// It just basically scans (or visits) all the AST nodes and then return the result

use ast::AST;
use error_handling::Error;
use lexer::Token;
use position::Position;
use value::Value;

pub struct Interpreter {
    ast: AST,
}

impl Interpreter {
    pub fn new(ast: AST) -> Interpreter {
        Interpreter { ast }
    }

    pub fn interpret(&self) -> (Option<Value>, Option<Error>) {
        return self.visit(self.ast);
    }

    fn visit(&self, ast: AST) -> (Option<Value>, Option<Error>) {
        match &ast {
            AST::FormingCalc {
                node1,
                operator,
                node2,
                pos_start,
                pos_end,
            } => self.visit_forming_calc(node1.clone(), operator.unwrap().clone(), node2.clone()),
            AST::Factor {
                identifier,
                value,
                pos_start,
                pos_end,
            } => self.visit_factor(
                identifier.to_string(),
                value.clone(),
                pos_start.clone(),
                pos_end.clone(),
            ),
            AST::UnaryFactor {
                sign,
                value,
                pos_start,
                pos_end,
            } => self.visit_unary(
                sign.to_string(),
                value.clone(),
                pos_start.clone(),
                pos_end.clone(),
            ),
            AST::Nil => {
                let factor: Option<Value> = None;
                let err: Option<Error> = None;
                (factor, err)
            }
        }
    }

    fn visit_factor(
        &self,
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
    }

    fn visit_unary(
        &self,
        sign: String,
        value: Box<AST>,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
    }

    fn visit_forming_calc(
        &self,
        node1: Box<AST>,
        operator: Token,
        node2: Box<AST>,
    ) -> (Option<Value>, Option<Error>) {
    }
}
