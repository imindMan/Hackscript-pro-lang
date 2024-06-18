// Nodes for the AST
//
//
//

use lexer::Token;

// AST container itself
// For now, the FormingCalc is the root of everything

pub struct AST {
    root: FormingCalc,
    level: i32,
}

impl AST {
    pub fn new(root_: FormingCalc) -> AST {
        AST {
            root: root_,
            level: 0,
        }
    }
    pub fn display() {}
}

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

// Serve for forming the calculation between those number nodes
pub struct FormingCalc {
    node1: NumberNode,
    operator: String,
    node2: NumberNode,
}

impl FormingCalc {
    pub fn new(node1_: NumberNode, operator_: String, node2_: NumberNode) -> FormingCalc {
        FormingCalc {
            node1: node1_,
            operator: operator_,
            node2: node2_,
        }
    }
}
