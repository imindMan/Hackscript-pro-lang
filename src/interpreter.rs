// INFO: First start of an interpreter, the main part of the whole project
// It just basically scans (or visits) all the AST nodes and then return the result

use crate::ast_implementation::Token;
use crate::ast_implementation::AST;
use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use crate::value::value_trait::ValueTrait;
use crate::value::Value;

pub struct Interpreter {
    ast: AST,
}

impl Interpreter {
    pub fn new(ast: AST) -> Interpreter {
        Interpreter { ast }
    }

    // INFO: Main idea: the Interpreter will visit every single nodes in the AST and then
    // execute the code based on that nodes
    pub fn interpret(&self) -> Result<Value, Error> {
        self.visit(self.ast.clone())
    }

    fn visit(&self, ast: AST) -> Result<Value, Error> {
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
            AST::Tuple {
                value,
                pos_start,
                pos_end,
            } => self.visit_tuple(value, pos_start, pos_end),
            AST::Nil => Ok(Value::new()),
        }
    }

    fn visit_number(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Result<Value, Error> {
        // quick initialization for the number value
        let final_value: f32 = value.parse().unwrap();
        let new_identifier: String = if final_value == final_value.floor() {
            String::from("integer")
        } else {
            String::from("float")
        };
        Ok(Value::new_number(
            new_identifier,
            final_value.to_string(),
            pos_start,
            pos_end,
        ))
    }
    fn visit_string(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Result<Value, Error> {
        Ok(Value::new_string(value, pos_start, pos_end))
    }
    fn visit_unary(
        &self,
        sign: String,
        value: Box<AST>,
        pos_start: Position,
    ) -> Result<Value, Error> {
        let factor = self.visit(*value)?;
        match factor {
            Value::Number(number) => {
                let mut final_value: String = number.value.clone();
                // check if the number should be in an opposite sign or not
                if sign.as_str() == MINUS {
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
                Ok(Value::new_number(
                    number.identifier,
                    final_value,
                    pos_start,
                    number.pos_end,
                ))
            }
            Value::Nil => Ok(Value::new()),
            _ => panic!("Expect passing the parser"),
        }
    }

    fn visit_forming_calc(
        &self,
        node1: Box<AST>,
        operator: Option<Token>,
        node2: Box<AST>,
    ) -> Result<Value, Error> {
        let value1 = self.visit(*node1)?;

        // if an operator is none, it means that the next node2 must be none, too (according to the
        // parser), that's why we don't need to check next and just return te result we got
        if operator.is_none() {
            return Ok(value1);
        }

        let real_operator = operator.unwrap()._type;
        let value2 = self.visit(*node2)?;

        Ok(match real_operator.as_str() {
            PLUS => value1.add_to(value2)?,
            MINUS => value1.subtract_to(value2)?,
            MULTIPLY => value1.multiply_by(value2)?,
            DIVIDE => value1.divide_by(value2)?,
            GREATER => value1.greater(value2)?,
            GREATER_OR_EQUAL => value1.greater_or_equal(value2)?,
            LESS => value1.less(value2)?,
            LESS_OR_EQUAL => value1.less_or_equal(value2)?,
            EQUAL => value1.equal(value2)?,
            NOT_EQUAL => value1.not_equal(value2)?,
            AND => value1.and(value2)?,
            OR => value1.or(value2)?,
            APPEND => value1.append(value2)?,
            INDEXING => value1.indexing(value2)?,
            &_ => panic!("No existing operator, failed unexpected"),
        })
    }
    fn visit_boolean(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Result<Value, Error> {
        Ok(Value::new_boolean(
            match value.as_str() {
                TRUE => true,
                FALSE => false,
                _ => panic!("Pass in TRUE or FALSE value, dev"),
            },
            pos_start,
            pos_end,
        ))
    }
    fn visit_null(
        &self,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Result<Value, Error> {
        Ok(Value::new_null(value, pos_start, pos_end))
    }
    fn visit_tuple(
        &self,
        value: Vec<AST>,
        pos_start: Position,
        pos_end: Position,
    ) -> Result<Value, Error> {
        let mut final_vec: Vec<Value> = Vec::new();
        for i in value {
            final_vec.push(self.visit(i)?);
        }

        Ok(Value::new_tuple(final_vec, pos_start, pos_end))
    }
}
