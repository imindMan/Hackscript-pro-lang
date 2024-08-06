// INFO: This is the file for the AST
// NOTE: To understand what are the roles of those nodes in the AST, head over to src/parser/lib.rs
//
use crate::ast_implementation::Token;
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
    Tuple {
        value: Vec<AST>,
        pos_start: Position,
        pos_end: Position,
    },
    Set {
        value: Vec<AST>,
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
        let pos_start = get_pos_start(&node1);
        let pos_end = match get_pos_end(&node2) {
            Some(pos) => pos,
            _ => pos_start.clone(),
        };

        AST::FormingCalc {
            node1,
            operator,
            node2,
            pos_end,
            pos_start,
        }
    }

    // INFO: This is the initialization method of Unary
    // To know what is a Unary, check the grammar rules in parser module
    pub fn new_unaryfactor(sign: Token, value: Box<AST>) -> AST {
        let pos_end = match get_pos_end(&value) {
            Some(pos) => pos,
            _ => sign.pos_start.clone(),
        };
        AST::UnaryNumber {
            sign: sign._type,
            value,
            pos_end,
            pos_start: sign.pos_start,
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
    pub fn new_tuple(value: Vec<AST>, pos_start: Position, pos_end: Position) -> AST {
        AST::Tuple {
            value,
            pos_start,
            pos_end,
        }
    }
    pub fn new_set(value: Vec<AST>, pos_start: Position, pos_end: Position) -> AST {
        AST::Set {
            value,
            pos_start,
            pos_end,
        }
    }
}
fn get_pos_start(node: &AST) -> Position {
    match node.clone() {
        AST::Number {
            identifier: _,
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::FormingCalc {
            node1: _,
            operator: _,
            node2: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::UnaryNumber {
            sign: _,
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::String {
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::Boolean {
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::Null {
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::Tuple {
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        AST::Set {
            value: _,
            pos_start,
            pos_end: _,
        } => pos_start,
        _ => panic!(
            "This is not a valid arithmetic expression, since there's no head of this expression"
        ),
    }
}
fn get_pos_end(node: &AST) -> Option<Position> {
    match node.clone() {
        AST::Number {
            identifier: _,
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::FormingCalc {
            node1: _,
            operator: _,
            node2: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::UnaryNumber {
            sign: _,
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::String {
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::Boolean {
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::Null {
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::Tuple {
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        AST::Set {
            value: _,
            pos_start: _,
            pos_end,
        } => Some(pos_end),
        _ => None,
    }
}
