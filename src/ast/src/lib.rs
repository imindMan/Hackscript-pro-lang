// Nodes for the AST
//
//
//
use lexer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Factor {
        identifier: String,
        token: Token,
    },

    FormingCalc {
        node1: Box<AST>,
        operator: String,
        node2: Box<AST>,
    },

    Nil,
}

impl AST {
    // This is by default
    pub fn new() -> AST {
        AST::Nil
    }

    pub fn new_factor(token: Token) -> AST {
        let mut identifier: String = String::new();
        if token.value.contains('.') {
            identifier.push_str("float");
        } else {
            identifier.push_str("integer");
        }

        AST::Factor { identifier, token }
    }

    pub fn new_formingcalc(node1: Box<AST>, operator: String, node2: Box<AST>) -> AST {
        AST::FormingCalc {
            node1,
            operator,
            node2,
        }
    }
}
