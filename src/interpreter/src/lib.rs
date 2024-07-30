// INFO: First start of an interpreter, the main part of the whole project
// It just basically scans (or visits) all the AST nodes and then return the result

use ast::AST;
use error_handling::Error;
use lexer::Token;
use position::Position;
use value::value_trait::ValueTrait;
use value::Value;

pub struct Interpreter {
    ast: AST,
}

impl Interpreter {
    pub fn new(ast: AST) -> Interpreter {
        Interpreter { ast }
    }

    // INFO: Main idea: the Interpreter will visit every single nodes in the AST and then
    // execute the code based on that nodes
    pub fn interpret(&self) -> (Option<Value>, Option<Error>) {
        self.visit(self.ast.clone())
    }

    fn visit(&self, ast: AST) -> (Option<Value>, Option<Error>) {
        match ast {
            AST::FormingCalc {
                node1,
                operator,
                node2,
                ..
            } => self.visit_forming_calc(node1, operator, node2),
            AST::Number {
                identifier: _,
                value,
                pos_start,
                pos_end,
            } => self.visit_number(value, pos_start, pos_end),
            AST::String {
                value,
                pos_start,
                pos_end,
            } => self.visit_string(value, pos_start, pos_end),
            AST::UnaryNumber {
                sign,
                value,
                pos_start,
                ..
            } => self.visit_unary(sign, value, pos_start),
            AST::Boolean {
                value,
                pos_start,
                pos_end,
            } => self.visit_boolean(value, pos_start, pos_end),
            AST::Null {
                value,
                pos_start,
                pos_end,
            } => self.visit_null(value, pos_start, pos_end),
            AST::Nil => {
                let factor: Option<Value> = Some(Value::new());
                let err: Option<Error> = None;
                (factor, err)
            }
        }
    }

    fn visit_number(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        // quick initialization for the number value
        let final_value: f32 = value.parse().unwrap();
        let new_identifier: String;
        if final_value == final_value.floor() {
            new_identifier = String::from("integer");
        } else {
            new_identifier = String::from("float");
        };
        let factor: Option<Value> = Some(Value::new_number(
            new_identifier,
            final_value.to_string(),
            pos_start,
            pos_end,
        ));
        let err: Option<Error> = None;

        (factor, err)
    }
    fn visit_string(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let string: Option<Value> = Some(Value::new_string(value, pos_start, pos_end));
        let err: Option<Error> = None;

        (string, err)
    }
    fn visit_unary(
        &self,
        sign: String,
        value: Box<AST>,
        pos_start: Position,
    ) -> (Option<Value>, Option<Error>) {
        let (factor, err) = self.visit(*value);
        if err.is_some() {
            return (factor, err);
        }
        match factor.unwrap() {
            Value::Number(number) => {
                let mut final_value: String = number.value.clone();
                // check if the number should be in an opposite sign or not
                if sign.as_str() == hacktypes::MINUS {
                    match number.identifier.as_str() {
                        "integer" => {
                            let mut number_value: i32 = number.value.parse().unwrap();
                            number_value *= -1;
                            final_value = number_value.to_string();
                        }
                        "float" => {
                            let mut number_value: f32 = number.value.parse().unwrap();
                            number_value *= -1.0;
                            final_value = number_value.to_string();
                        }
                        &_ => panic!("No existing data types"),
                    };
                };
                // then return that number
                let final_number: Option<Value> = Some(Value::new_number(
                    number.identifier,
                    final_value,
                    pos_start,
                    number.pos_end,
                ));
                let err: Option<Error> = None;

                (final_number, err)
            }
            Value::Nil => {
                let final_val: Option<Value> = Some(Value::new());
                let err: Option<Error> = None;
                (final_val, err)
            }
            _ => panic!("Expect passing the parser"),
        }
    }

    fn visit_forming_calc(
        &self,
        node1: Box<AST>,
        operator: Option<Token>,
        node2: Box<AST>,
    ) -> (Option<Value>, Option<Error>) {
        let (value1, err1) = self.visit(*node1);

        // if an operator is none, it means that the next node2 must be none, too (according to the
        // parser), that's why we don't need to check next and just return te result we got
        if err1.is_some() || operator.is_none() {
            return (value1, err1);
        }

        let real_operator = operator.unwrap()._type;
        let (value2, err2) = self.visit(*node2);
        if err2.is_some() {
            return (value2, err2);
        };

        let (res, err) = match real_operator.as_str() {
            hacktypes::PLUS => value1.unwrap().add_to(value2.unwrap()),
            hacktypes::MINUS => value1.unwrap().subtract_to(value2.unwrap()),
            hacktypes::MULTIPLY => value1.unwrap().multiply_by(value2.unwrap()),
            hacktypes::DIVIDE => value1.unwrap().divide_by(value2.unwrap()),
            hacktypes::GREATER => value1.unwrap().greater(value2.unwrap()),
            hacktypes::GREATER_OR_EQUAL => value1.unwrap().greater_or_equal(value2.unwrap()),
            hacktypes::LESS => value1.unwrap().less(value2.unwrap()),
            hacktypes::LESS_OR_EQUAL => value1.unwrap().less_or_equal(value2.unwrap()),
            hacktypes::EQUAL => value1.unwrap().equal(value2.unwrap()),
            hacktypes::NOT_EQUAL => value1.unwrap().not_equal(value2.unwrap()),
            hacktypes::AND => value1.unwrap().and(value2.unwrap()),
            hacktypes::OR => value1.unwrap().or(value2.unwrap()),
            &_ => panic!("No existing operator, failed unexpected"),
        };

        (res, err)
    }
    fn visit_boolean(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let bool: Option<Value> = Some(Value::new_boolean(
            match value.as_str() {
                hacktypes::TRUE => true,
                hacktypes::FALSE => false,
                _ => panic!("Pass in TRUE or FALSE value, dev"),
            },
            pos_start,
            pos_end,
        ));
        let err: Option<Error> = None;

        (bool, err)
    }
    fn visit_null(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let null_val: Option<Value> = Some(Value::new_null(value, pos_start, pos_end));
        let err: Option<Error> = None;
        (null_val, err)
    }
}
