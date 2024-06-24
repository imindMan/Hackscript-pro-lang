// Nodes for the AST
//
//
//

use lexer::Token;

// NUMBER NODE: for numbers only
pub struct NumberNode {
    token: Token,
    identifier: String,
}

impl NumberNode {
    pub fn new(token_: Token) -> NumberNode {
        let mut identifier_: String = String::new();
        if token_.value.contains('.') {
            identifier_.push_str("float");
        } else {
            identifier_.push_str("integer");
        }
        NumberNode {
            token: token_,
            identifier: identifier_,
        }
    }
}

// For now, Term class is served for the multiplication

pub struct Term {
    node1: NumberNode,
    operator: String,
    node2: NumberNode,
}

impl Term {
    pub fn new(node1_: NumberNode, operator_: String, node2_: NumberNode) -> Term {
        Term {
            node1: node1_,
            operator: operator_,
            node2: node2_,
        }
    }
}

// And Expr class is served for the whole arithmetic expression

pub struct Expr {
    node1: Term,
    operator: String,
    node2: Term,
}

impl Expr {
    pub fn new(node1_: Term, operator_: String, node2_: Term) -> Expr {
        Expr {
            node1: node1_,
            operator: operator_,
            node2: node2_,
        }
    }
}
