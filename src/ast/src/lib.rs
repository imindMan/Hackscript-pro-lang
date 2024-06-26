// INFO: This is the file for the AST
// NOTE: To understand what are the roles of those nodes in the AST, head over to src/parser/lib.rs
//
use lexer::Token;
use position::Position;

#[derive(Debug, Clone)]
pub enum AST {
    Factor {
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    },

    UnaryFactor {
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

    Nil,
}

impl AST {
    // This is the initialization method by default
    pub fn default() -> AST {
        AST::Nil
    }

    pub fn new_factor(token: Token) -> AST {
        let mut identifier: String = String::new();
        if token.value.contains('.') {
            identifier.push_str("float");
        } else {
            identifier.push_str("integer");
        }

        AST::Factor {
            identifier,
            value: token.value.clone(),
            pos_start: token.pos_start.clone(),
            pos_end: token.pos_end.clone(),
        }
    }

    pub fn new_formingcalc(node1: Box<AST>, operator: Option<Token>, node2: Box<AST>) -> AST {
        let node1_temp = node1.clone();
        let node2_temp = node2.clone();
        let pos_start = match &*node1_temp {
            AST::Factor { identifier: _, value: _, pos_start, pos_end: _ } => pos_start,
            AST::FormingCalc { node1: _, operator: _, node2:_ , pos_start, pos_end: _ } => pos_start,
            AST::UnaryFactor { sign: _, value: _, pos_start, pos_end: _ } => pos_start,
            _ => panic!("This is not a valid arithmetic expression, since there's no head of this expression"), 
        };

        let pos_end = match &*node2_temp {
            AST::Factor {
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
            AST::UnaryFactor {
                sign: _,
                value: _,
                pos_start: _,
                pos_end,
            } => pos_end,
            _ => pos_start,
        };

        AST::FormingCalc {
            node1: node1,
            operator: operator,
            node2: node2,
            pos_start: pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }
    pub fn new_unaryfactor(sign: Token, value: Box<AST>) -> AST {
        let value_temp = value.clone();
        let pos_end = match &*value_temp {
            AST::Factor {
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
            _ => panic!("No unary factor doesn't have a single value"),
        };

        AST::UnaryFactor {
            sign: sign.value.clone(),
            value: value,
            pos_start: sign.pos_start.clone(),
            pos_end: pos_end.clone(),
        }
    }
}
