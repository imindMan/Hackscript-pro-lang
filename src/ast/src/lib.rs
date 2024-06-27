// Nodes for the AST
//
//
//
use lexer::Token;
// // NUMBER NODE: for numbers only
// pub struct NumberNode {
//     token: Token,
//     identifier: String,
// }

// impl NumberNode {
//     pub fn new(token_: Token) -> NumberNode {
//         let mut identifier_: String = String::new();
//         if token_.value.contains('.') {
//             identifier_.push_str("float");
//         } else {
//             identifier_.push_str("integer");
//         }
//         NumberNode {
//             token: token_,
//             identifier: identifier_,
//         }
//     }
// }

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
