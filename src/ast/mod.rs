// INFO: This is the file for the AST
// NOTE: To understand what are the roles of those nodes in the AST, head over to src/parser/lib.rs
//
use crate::lexer::Token;
use crate::position::Position;

#[derive(Debug, Clone)]
pub enum AST {
    Number {
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    },

    UnaryNumber {
        sign: String,
        value: Box<AST>,
        pos_start: Position,
        pos_end: Position,
    },

    FormingCalc {
        node1: Box<AST>,
        operator: Option<Token>,
        node2: Box<AST>,
        pos_start: Position,
        pos_end: Position,
    },
    String {
        value: String,
        pos_start: Position,
        pos_end: Position,
    },
    Boolean {
        value: String,
        pos_start: Position,
        pos_end: Position,
    },
    Null {
        value: String,
        pos_start: Position,
        pos_end: Position,
    },
    Nil,
}

impl AST {
    // INFO: This is the initialization method by default
    // It will just return the Nil value
    pub fn new() -> AST {
        AST::Nil
    }

    // INFO: This is the initialization method for the Factor attribute
    // To know what is a Factor, check the grammar rules in parser module
    pub fn new_number(token: Token) -> AST {
        let mut identifier: String = String::new();
        if token.value.contains('.') {
            identifier.push_str("float");
        } else {
            identifier.push_str("integer");
        }

        AST::Number {
            identifier,
            value: token.value.clone(),
            pos_start: token.pos_start.clone(),
            pos_end: token.pos_end.clone(),
        }
    }
    // INFO: This is the initialization method for the FormingCalc attribute
    // FormingCalc is the main representative of Term and Expr
    // To know what are Term and Expr, check the grammar rules in parser module
    pub fn new_formingcalc(node1: Box<AST>, operator: Option<Token>, node2: Box<AST>) -> AST {
        let node1_temp = node1.clone();
        let node2_temp = node2.clone();
        let pos_start = match &*node1_temp {
            AST::Number { identifier: _, value: _, pos_start, pos_end: _ } => pos_start,
            AST::FormingCalc { node1: _, operator: _, node2:_ , pos_start, pos_end: _ } => pos_start,
            AST::UnaryNumber { sign: _, value: _, pos_start, pos_end: _ } => pos_start,
            AST::String {value: _, pos_start, pos_end: _} => pos_start,
            AST::Boolean { value: _, pos_start, pos_end: _ } => pos_start,
            AST::Null { value: _, pos_start, pos_end: _ } => pos_start,
            _ => panic!("This is not a valid arithmetic expression, since there's no head of this expression"), 
        };

        let pos_end = match &*node2_temp {
            AST::Number {
                identifier: _,
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::FormingCalc {
                node1: _,
                operator: _,
                node2: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::UnaryNumber {
                sign: _,
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::String {
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::Boolean {
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::Null {
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            _ => pos_start,
        };

        AST::FormingCalc {
            node1,
            operator,
            node2,
            pos_start: pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }

    // INFO: This is the initialization method of Unary
    // To know what is a Unary, check the grammar rules in parser module
    pub fn new_unaryfactor(sign: Token, value: Box<AST>) -> AST {
        let value_temp = value.clone();
        let pos_end = match &*value_temp {
            AST::Number {
                identifier: _,
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::FormingCalc {
                node1: _,
                operator: _,
                node2: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::UnaryNumber {
                sign: _,
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::Boolean {
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            AST::Null {
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            _ => panic!("No unary factor doesn't have a single value"),
        };

        AST::UnaryNumber {
            sign: sign._type,
            value,
            pos_start: sign.pos_start,
            pos_end: pos_end.clone(),
        }
    }
    pub fn new_string(token: Token) -> AST {
        AST::String {
            value: token.value,
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }

    pub fn new_boolean(token: Token) -> AST {
        AST::Boolean {
            value: token._type,
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }
    pub fn new_null(token: Token) -> AST {
        AST::Null {
            value: token._type,
            pos_start: token.pos_start,
            pos_end: token.pos_end,
        }
    }
}